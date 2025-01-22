use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;

use rand::{distr::Uniform, rngs::ThreadRng, Rng};

use hall::core::{ActorIdType, AttributeArray, Attributes, ErgArray, ErgType, Host, Phase, RemoteIdType, Stage, TickType};
use hall::hall::{HallCard, HallMission};
use hall::message::GameUpdateStateResponse;
use hall::player::PlayerCard;
use hall::util;
use shared_net::{op, AuthType, UserIdType};

use crate::private::game::{GameActor, GameMachine, GameMission, GameRemote, GameUser, GameUserCommandState};

pub(crate) type UserMapType = HashMap<UserIdType, GameUser>;
pub(crate) type RemoteMapType = HashMap<RemoteIdType, GameRemote>;
pub(crate) type ActorMapType = HashMap<ActorIdType, GameActor>;

#[derive(Default)]
pub(crate) struct GameState {
    pub(crate) users: UserMapType,
    pub(crate) remotes: RemoteMapType,
    pub(crate) actors: ActorMapType,
    current_tick: TickType,
    stage: Stage,
    pub(crate) erg_roll: ErgArray,
    pub(crate) rng: ThreadRng,
    pub(crate) mission: GameMission,
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) enum TargetIdType {
    None,
    Local(UserIdType),
    Remote(RemoteIdType),
    Actor(ActorIdType),
}

fn random_unique<T, U>(existing: &HashMap<T, U>, rng: &mut impl Rng) -> T
where
    rand::distr::StandardUniform: rand::distr::Distribution<T>,
    T: Eq,
    T: std::hash::Hash,
{
    loop {
        let attempt = rng.random();
        if !existing.contains_key(&attempt) {
            return attempt;
        }
    }
}

impl GameState {
    pub(crate) fn new(hall_mission: HallMission, rng: &mut impl Rng) -> Self {
        let mut remotes = HashMap::new();
        let mut actors = HashMap::new();

        let mut mission = GameMission::from(hall_mission);

        for node in mission.node.iter_mut() {
            let attributes = Attributes::from_arrays([util::pick_values(rng), util::pick_values(rng), util::pick_values(rng), util::pick_values(rng)]);
            node.remote = random_unique(&remotes, rng);
            remotes.insert(node.remote, GameRemote::new(attributes));

            let actor_count: usize = rng.random_range(..5);
            for _ in 0..actor_count {
                let actor = random_unique(&actors, rng);
                node.actors.push(actor);
                actors.insert(actor, GameActor::new());
            }
        }

        Self {
            remotes,
            mission,
            actors,
            ..Default::default()
        }
    }

    fn is_valid_transition(&self, game_stage: &Stage) -> bool {
        if self.stage == *game_stage {
            return true;
        }
        match game_stage {
            Stage::Idle => false,
            Stage::Building => self.stage == Stage::Idle,
            Stage::Running(phase) => match phase {
                Phase::ChooseIntent => self.stage == Stage::Idle || self.stage == Stage::Running(Phase::TurnEnd),
                Phase::ChooseAttr => self.stage == Stage::Running(Phase::ChooseIntent),
                Phase::CardPlay => self.stage == Stage::Running(Phase::ChooseAttr),
                Phase::TurnEnd => self.stage == Stage::Running(Phase::CardPlay),
            },
            Stage::End => matches!(self.stage, Stage::Running(_)),
        }
    }

    pub(crate) fn set_stage(&mut self, stage: Stage) {
        if self.is_valid_transition(&stage) {
            self.stage = stage;
        } // else log error
    }

    pub(crate) fn set_phase(&mut self, phase: Phase) {
        self.set_stage(Stage::Running(phase));
        self.users.iter_mut().for_each(|(_, user)| user.state.command.should_be(phase.expected_command()));
    }

    pub(crate) fn get_user_auth(&self, user_id_type: UserIdType, user_auth: AuthType) -> Option<&GameUser> {
        if let Some(user) = self.users.get(&user_id_type) {
            if user.auth == user_auth {
                return Some(user);
            }
        }
        None
    }
    pub(crate) fn split_get_user_auth_mut(users: &mut UserMapType, user_id_type: UserIdType, user_auth: AuthType) -> Option<&mut GameUser> {
        if let Some(user) = users.get_mut(&user_id_type) {
            if user.auth == user_auth {
                return Some(user);
            }
        }
        None
    }

    pub(crate) fn all_users_have_players(&self) -> bool {
        self.users.iter().all(|(_, user)| user.player.is_some())
    }

    pub(crate) fn determine_last_command(&self) -> Option<op::Command> {
        let first_op = match self.users.iter().next().map(|(_, user)| user.state.command) {
            None => None,
            Some(command) => match command {
                GameUserCommandState::Invalid => None,
                GameUserCommandState::Expected(_) => None,
                GameUserCommandState::Actual(actual) => Some(actual),
            },
        };
        let command = first_op?;

        if self.users.iter().all(|(_, user)| user.state.command.is(command)) {
            first_op
        } else {
            None
        }
    }

    pub(crate) fn user_add(&mut self, user_id_type: UserIdType, game_user: GameUser) {
        self.users.insert(user_id_type, game_user);
    }

    // pub(crate) fn user_remove(&mut self, user_id_type: UserIdType, user_auth: AuthType) {
    //     match self.users.entry(user_id_type) {
    //         Entry::Occupied(user) if user.get().auth == user_auth => self.users.remove(&user_id_type),
    //         _ => None,
    //     };
    // }

    pub(crate) fn get_remote(&self, remote: RemoteIdType) -> Option<&GameRemote> {
        self.remotes.get(&remote)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.users.is_empty()
    }

    pub(crate) fn now(&self) -> TickType {
        self.current_tick
    }

    pub(crate) fn tick(&mut self) -> TickType {
        self.current_tick += 1;

        let mut executables = Vec::new();

        for user in self.users.values_mut().filter(|user| user.machine.is_active()) {
            if let Some(player) = &user.player {
                user.machine.tick(&player.attributes);
                executables.append(&mut user.machine.run(&player.attributes));
            }
        }

        for remote in self.remotes.values_mut().filter(|remote| remote.machine.is_active()) {
            remote.machine.tick(&remote.attributes);
            executables.append(&mut remote.machine.run(&remote.attributes));
        }

        for executable in executables {
            executable.execute(&mut self.users, &mut self.remotes, &mut self.actors);
        }

        for user in self.users.values_mut() {
            user.state.fill_hand();
            user.mission_state.expire_tokens(self.current_tick);
        }

        for remote in self.remotes.values_mut() {
            remote.end_turn();
        }

        self.current_tick
    }

    pub(crate) fn roll(&mut self) {
        let range = Uniform::new_inclusive(1, 6).unwrap();
        for erg in self.erg_roll.iter_mut() {
            *erg = self.rng.sample(range);
        }
    }

    pub(crate) fn split_borrow_for_resolve(&mut self) -> (&ErgArray, &mut UserMapType, &mut RemoteMapType, &GameMission) {
        (&self.erg_roll, &mut self.users, &mut self.remotes, &self.mission)
    }

    pub(crate) fn resolve_matchups(erg_roll: &[ErgType], p1: &AttributeArray, p2: &AttributeArray) -> (ErgArray, ErgArray) {
        let matchups = zip(erg_roll, zip(p1, p2)).collect::<Vec<_>>();

        let mut local_alloc: ErgArray = [0, 0, 0, 0];
        let mut remote_alloc: ErgArray = [0, 0, 0, 0];

        for (idx, (erg, (protag, antag))) in matchups.iter().enumerate() {
            match protag.cmp(antag) {
                Ordering::Greater => local_alloc[idx] = **erg,
                Ordering::Less => remote_alloc[idx] = **erg,
                Ordering::Equal => {
                    local_alloc[idx] = **erg / 2;
                    remote_alloc[idx] = **erg / 2;
                }
            };
        }

        (local_alloc, remote_alloc)
    }

    pub(crate) fn resolve_cards(&mut self, cards: Vec<(UserIdType, RemoteIdType, HallCard, TargetIdType)>) -> Vec<(UserIdType, RemoteIdType, PlayerCard, TargetIdType)> {
        let mut result = Vec::new();
        for (local_id, remote_id, card, target) in cards.into_iter() {
            let machine = match card.host {
                Host::Local => self.users.get_mut(&local_id).map(|u| &mut u.machine),
                Host::Remote => self.remotes.get_mut(&remote_id).map(|r| &mut r.machine),
                Host::None => None,
            };
            if let Some(played) = machine.and_then(|m| m.enqueue(card, target, local_id)) {
                result.push((local_id, remote_id, played, target));
            }
        }
        result
    }
}

impl GameState {
    pub(crate) fn make_response(id: UserIdType, user: &GameUser, remote: &GameMachine, mission: &GameMission) -> GameUpdateStateResponse {
        GameUpdateStateResponse {
            player_state: user.state.to_player_view(),
            local_machine: user.machine.to_player_view(id),
            remote_machine: remote.to_player_view(id),
            mission: mission.to_player_view(&user.mission_state),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::private::game::game_state::GameState;
    use hall::core::ErgType;

    #[test]
    fn test_resolve_1() {
        let erg_roll = [1, 3, 3, 6];

        let p1_a = [9, 1, 9, 1];
        let p2_a = [1, 9, 1, 9];

        let result = GameState::resolve_matchups(&erg_roll, &p1_a, &p2_a);
        assert_eq!(result.0.iter().sum::<ErgType>(), 4);
        assert_eq!(result.1.iter().sum::<ErgType>(), 9);
    }

    #[test]
    fn test_resolve_2() {
        let erg_roll = [1, 3, 3, 6];

        let p1_b = [5, 5, 5, 5];
        let p2_b = [5, 5, 5, 5];

        let result = GameState::resolve_matchups(&erg_roll, &p1_b, &p2_b);
        assert_eq!(result.0.iter().sum::<ErgType>(), 5);
        assert_eq!(result.1.iter().sum::<ErgType>(), 5);
    }

    #[test]
    fn test_resolve_3() {
        let erg_roll = [1, 3, 3, 6];

        let p1_c = [9, 1, 5, 5];
        let p2_c = [1, 9, 5, 5];
        let result = GameState::resolve_matchups(&erg_roll, &p1_c, &p2_c);
        assert_eq!(result.0.iter().sum::<ErgType>(), 5);
        assert_eq!(result.1.iter().sum::<ErgType>(), 7);
    }

    #[test]
    fn test_resolve_4() {
        let erg_roll = [1, 3, 3, 6];

        let p1_d = [5, 5, 9, 1];
        let p2_d = [5, 5, 1, 9];
        let result = GameState::resolve_matchups(&erg_roll, &p1_d, &p2_d);
        assert_eq!(result.0.iter().sum::<ErgType>(), 4);
        assert_eq!(result.1.iter().sum::<ErgType>(), 7);
    }

    #[test]
    fn test_resolve_5() {
        let erg_roll = [1, 3, 3, 6];

        let p1_d = [5, 5, 9, 1];
        let p2_c = [1, 9, 5, 5];
        let result = GameState::resolve_matchups(&erg_roll, &p1_d, &p2_c);
        assert_eq!(result.0.iter().sum::<ErgType>(), 4);
        assert_eq!(result.1.iter().sum::<ErgType>(), 9);
    }
}
