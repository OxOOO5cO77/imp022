use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::zip;

use rand::prelude::IteratorRandom;
use rand::{distr::Uniform, rngs::ThreadRng, Rng};

use shared_net::{op, AuthType, UserIdType};

use crate::data::core::{AttributeValueType, Attributes, ErgType};
use crate::data::game::{GameMission, GamePhase, GameRemote, GameStage, GameUser};
use crate::data::hall::{HallCard, HallMission};
use crate::data::player::{PlayerCard, PlayerCommandState};
use crate::data::util;

pub type RemoteIdType = u64;
pub type TickType = u16;

type UserMapType = HashMap<UserIdType, GameUser>;
type RemoteMapType = HashMap<RemoteIdType, GameRemote>;

#[derive(Default)]
pub struct GameState {
    pub users: UserMapType,
    pub remotes: RemoteMapType,
    current_tick: TickType,
    stage: GameStage,
    pub erg_roll: [ErgType; 4],
    pub rng: ThreadRng,
    pub mission: GameMission,
}

#[derive(PartialEq)]
pub enum IdType {
    Local(UserIdType),
    Remote(RemoteIdType),
}

impl GameState {
    pub fn new(hall_mission: HallMission, mut rng: &mut impl Rng) -> Self {
        let mut remotes = HashMap::new();

        let mut mission = GameMission::from(hall_mission);

        for node in mission.node.iter_mut() {
            let attributes = Attributes::from_arrays([util::pick_values(&mut rng), util::pick_values(&mut rng), util::pick_values(&mut rng), util::pick_values(&mut rng)]);
            node.remote = rng.random();
            remotes.insert(node.remote, GameRemote::new(attributes));
        }

        Self {
            remotes,
            mission,
            ..Default::default()
        }
    }

    pub fn get_stage(&self) -> GameStage {
        self.stage
    }

    fn is_valid_transition(&self, game_stage: &GameStage) -> bool {
        if self.stage == *game_stage {
            return true;
        }
        match game_stage {
            GameStage::Idle => false,
            GameStage::Building => self.stage == GameStage::Idle,
            GameStage::Running(phase) => match phase {
                GamePhase::TurnStart => self.stage == GameStage::Idle || self.stage == GameStage::Running(GamePhase::TurnEnd),
                GamePhase::ChooseAttr => self.stage == GameStage::Running(GamePhase::TurnStart),
                GamePhase::CardPlay => self.stage == GameStage::Running(GamePhase::ChooseAttr),
                GamePhase::TurnEnd => self.stage == GameStage::Running(GamePhase::CardPlay),
            },
            GameStage::End => matches!(self.stage, GameStage::Running(_)),
        }
    }

    pub fn set_stage(&mut self, stage: GameStage) {
        if self.is_valid_transition(&stage) {
            self.stage = stage;
        } // else log error
    }

    pub fn set_phase(&mut self, phase: GamePhase, expected: op::Command) {
        self.set_stage(GameStage::Running(phase));
        self.users.iter_mut().for_each(|(_, user)| user.state.command.should_be(expected));
    }

    pub fn get_user(&mut self, user_id_type: UserIdType) -> Option<&GameUser> {
        self.users.get(&user_id_type)
    }

    pub fn get_user_mut(&mut self, user_id_type: UserIdType) -> Option<&mut GameUser> {
        self.users.get_mut(&user_id_type)
    }

    pub fn get_user_auth(&self, user_id_type: UserIdType, user_auth: AuthType) -> Option<&GameUser> {
        if let Some(user) = self.users.get(&user_id_type) {
            if user.auth == user_auth {
                return Some(user);
            }
        }
        None
    }
    pub fn split_get_user_auth_mut(users: &mut UserMapType, user_id_type: UserIdType, user_auth: AuthType) -> Option<&mut GameUser> {
        if let Some(user) = users.get_mut(&user_id_type) {
            if user.auth == user_auth {
                return Some(user);
            }
        }
        None
    }

    pub fn all_users_have_players(&self) -> bool {
        self.users.iter().all(|(_, user)| user.player.is_some())
    }

    pub fn determine_last_command(&self) -> Option<op::Command> {
        let first_op = match self.users.iter().next().map(|(_, user)| user.state.command) {
            None => None,
            Some(command) => match command {
                PlayerCommandState::Invalid => None,
                PlayerCommandState::Expected(_) => None,
                PlayerCommandState::Actual(actual) => Some(actual),
            },
        };
        let command = first_op?;

        if self.users.iter().all(|(_, user)| user.state.command.is(command)) {
            first_op
        } else {
            None
        }
    }

    pub fn user_add(&mut self, user_id_type: UserIdType, game_user: GameUser) {
        self.users.insert(user_id_type, game_user);
    }

    pub fn user_remove(&mut self, user_id_type: UserIdType, user_auth: AuthType) {
        match self.users.entry(user_id_type) {
            Entry::Occupied(user) if user.get().auth == user_auth => self.users.remove(&user_id_type),
            _ => None,
        };
    }

    pub fn pick_remote(&mut self, rng: &mut impl Rng) -> Option<RemoteIdType> {
        self.remotes.keys().choose(rng).cloned()
    }

    pub fn get_remote(&self, remote: RemoteIdType) -> Option<&GameRemote> {
        self.remotes.get(&remote)
    }

    pub fn split_get_remote(remotes: &mut RemoteMapType, remote: RemoteIdType) -> Option<&GameRemote> {
        remotes.get(&remote)
    }

    pub fn is_empty(&self) -> bool {
        self.users.is_empty()
    }

    pub fn tick(&mut self) -> TickType {
        self.current_tick += 1;

        self.users.values_mut().for_each(|user| {
            if let Some(player) = &user.player {
                user.machine.tick(&player.attributes);
                user.state.fill_hand();
            }
        });
        self.remotes.values_mut().for_each(|remote| remote.machine.tick(&remote.attributes));

        self.current_tick
    }

    pub fn roll(&mut self) {
        let range = Uniform::new_inclusive(1, 6).unwrap();
        for erg in self.erg_roll.iter_mut() {
            *erg = self.rng.sample(range);
        }
    }

    pub fn split_borrow_for_resolve(&mut self) -> (&[ErgType; 4], &mut UserMapType, &mut RemoteMapType, &GameMission) {
        (&self.erg_roll, &mut self.users, &mut self.remotes, &self.mission)
    }

    pub fn resolve_matchups(erg_roll: &[ErgType], p1: &[AttributeValueType; 4], p2: &[AttributeValueType; 4]) -> ([ErgType; 4], [ErgType; 4]) {
        let matchups = zip(erg_roll, zip(p1, p2)).collect::<Vec<_>>();

        let mut local_alloc = [0, 0, 0, 0];
        let mut remote_alloc = [0, 0, 0, 0];

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

    pub fn resolve_cards(&mut self, cards: Vec<(IdType, HallCard, IdType)>) -> Vec<(IdType, PlayerCard, IdType)> {
        let mut result = Vec::new();
        for (source, card, target) in cards.into_iter() {
            let machine = match target {
                IdType::Local(local_id) => self.users.get_mut(&local_id).map(|u| &mut u.machine),
                IdType::Remote(remote_id) => self.remotes.get_mut(&remote_id).map(|r| &mut r.machine),
            };
            if let Some(played) = machine.and_then(|m| m.enqueue(card, source == target)) {
                result.push((source, played, target));
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use crate::data::core::ErgType;
    use crate::data::game::GameState;

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
