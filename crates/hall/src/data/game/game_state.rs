use crate::data::game::game_stage::{GamePhase, GameStage};
use crate::data::game::GameUser;
use rand::{distr::Uniform, rngs::ThreadRng, Rng};
use shared_data::game::card::ErgType;
use shared_data::player::attribute::{Attributes, AttributeValueType};
use shared_data::types::{AuthType, UserIdType};

use crate::data::game::game_remote::GameRemote;
use crate::data::util;
use shared_net::op;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::iter::zip;

pub(crate) type RemoteIdType = u128;
type TickType = u32;

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
}

impl GameState {
    pub fn new(num_remotes: usize, mut rng: &mut impl Rng) -> Self {
        //TODO: temp
        let mut remotes = HashMap::new();
        for i in 1..=num_remotes {
            let attributes = Attributes::from_arrays([util::pick_values(&mut rng), util::pick_values(&mut rng), util::pick_values(&mut rng), util::pick_values(&mut rng)]);
            remotes.insert(i as RemoteIdType, GameRemote::new(attributes));
        }

        Self {
            remotes,
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

    pub fn set_phase(&mut self, phase: GamePhase) {
        self.set_stage(GameStage::Running(phase));
    }

    pub fn get_user(&mut self, user_id_type: UserIdType) -> Option<&GameUser> {
        self.users.get(&user_id_type)
    }

    pub fn get_user_mut(&mut self, user_id_type: UserIdType) -> Option<&mut GameUser> {
        self.users.get_mut(&user_id_type)
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

    pub fn all_users_last_command(&self, command: op::Command) -> bool {
        self.users.iter().all(|(_, user)| user.state.last_command == Some(command))
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

    pub fn pick_remote(&mut self) -> Option<RemoteIdType> {
        self.remotes.keys().next().cloned()
    }

    pub fn split_get_remote(remotes: &mut RemoteMapType, remote: RemoteIdType) -> Option<&GameRemote> {
        remotes.get(&remote)
    }

    pub fn is_empty(&self) -> bool {
        self.users.is_empty()
    }

    pub fn tick(&mut self) {
        self.current_tick += 1;

        self.users.values_mut().for_each(|user| user.machine.tick());
        self.remotes.values_mut().for_each(|remote| remote.machine.tick());
    }

    pub fn roll(&mut self) {
        let range = Uniform::new_inclusive(1, 6).unwrap();
        for erg in self.erg_roll.iter_mut() {
            *erg = self.rng.sample(range);
        }
    }

    pub fn split_borrow_for_resolve(&mut self) -> (&[ErgType; 4], &mut UserMapType, &mut RemoteMapType) {
        (&self.erg_roll, &mut self.users, &mut self.remotes)
    }

    pub fn resolve_matchups(erg_roll: &[ErgType], p1: &[AttributeValueType; 4], p2: &[AttributeValueType; 4]) -> ([ErgType; 4], [ErgType; 4]) {
        let matchups = zip(erg_roll, zip(p1, p2)).collect::<Vec<_>>();

        let mut p_alloc = [0, 0, 0, 0];
        let mut a_alloc = [0, 0, 0, 0];

        for (idx, (erg, (protag, antag))) in matchups.iter().enumerate() {
            match protag.cmp(antag) {
                Ordering::Greater => p_alloc[idx] = **erg,
                Ordering::Less => a_alloc[idx] = **erg,
                Ordering::Equal => {
                    p_alloc[idx] = **erg / 2;
                    a_alloc[idx] = **erg / 2;
                }
            };
        }

        (p_alloc, a_alloc)
    }
}

#[cfg(test)]
mod tests {
    use crate::data::game::GameState;
    use shared_data::game::card::ErgType;

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
