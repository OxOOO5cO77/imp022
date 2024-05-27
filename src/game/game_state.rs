use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::iter::zip;

use rand::{distributions::Uniform, Rng, rngs::ThreadRng};

use crate::data::player::card::Kind;
use crate::game::player_state::PlayerState;

struct GameState {
    protagonist: PlayerState,
    antagonist: PlayerState,
    erg_roll: [u32; 4],
    rng: ThreadRng,
}

impl GameState {
    fn from_players(protagonist: PlayerState, antagonist: PlayerState) -> Self {
        GameState {
            protagonist,
            antagonist,
            erg_roll: [0, 0, 0, 0],
            rng: ThreadRng::default(),
        }
    }
    fn roll(&mut self) {
        let range = Uniform::from(1..=6);
        for erg in self.erg_roll.iter_mut() {
            *erg = self.rng.sample(range);
        }
    }

    fn increment(alloc: &mut (u32, u32), erg: u32) {
        alloc.0 += 1;
        alloc.1 += erg;
    }

    pub(crate) fn resolve_matchups(erg_roll: &[u32], p_kind: Kind, p_attr: &[u8], p_erg: &mut HashMap<Kind, u32>, a_kind: Kind, a_attr: &[u8], a_erg: &mut HashMap<Kind, u32>) {
        let mut matchups = zip(erg_roll, zip(p_attr, a_attr)).collect::<Vec<_>>();
        matchups.sort_unstable_by_key(|(erg, (_, _))| Reverse(*erg));

        let mut p_alloc = (0, 0);
        let mut a_alloc = (0, 0);

        for (erg, (protag, antag)) in matchups.iter() {
            match protag.cmp(antag) {
                Ordering::Greater => Self::increment(&mut p_alloc, **erg),
                Ordering::Less => Self::increment(&mut a_alloc, **erg),
                Ordering::Equal => {}
            };
        }

        for (erg, (protag, antag)) in matchups.iter() {
            match protag.cmp(antag) {
                Ordering::Greater => {}
                Ordering::Less => {}
                Ordering::Equal => match p_alloc.cmp(&a_alloc) {
                    Ordering::Greater => Self::increment(&mut a_alloc, **erg),
                    Ordering::Less => Self::increment(&mut p_alloc, **erg),
                    Ordering::Equal => Self::increment(&mut p_alloc, **erg),
                },
            };
        }

        *p_erg.entry(p_kind).or_default() += p_alloc.1;
        *a_erg.entry(a_kind).or_default() += a_alloc.1;
    }

    fn resolve(&mut self) -> Option<()> {
        let protag_attr = self.protagonist.get_pick_attr()?;
        let antag_attr = self.antagonist.get_pick_attr()?;

        Self::resolve_matchups(&self.erg_roll, self.protagonist.pick?, &protag_attr, &mut self.protagonist.erg, self.antagonist.pick?, &antag_attr, &mut self.antagonist.erg);
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::data::player::card::Kind;
    use crate::game::game_state::GameState;

    #[test]
    fn test_resolve() {
        let mut protag_erg = HashMap::<Kind, u32>::new();
        let mut antag_erg = HashMap::<Kind, u32>::new();

        let protag_a_attr = [9, 1, 9, 1];
        let protag_b_attr = [5, 5, 5, 5];
        let protag_c_attr = [9, 1, 5, 5];
        let protag_d_attr = [5, 5, 9, 1];
        let antag_a_attr = [1, 9, 1, 9];
        let antag_b_attr = [5, 5, 5, 5];
        let antag_c_attr = [1, 9, 5, 5];
        let antag_d_attr = [5, 5, 1, 9];

        let erg_roll = [1, 3, 5, 6];

        let kind = Kind::Analyze;
        GameState::resolve_matchups(&erg_roll, kind, &protag_a_attr, &mut protag_erg, kind, &antag_a_attr, &mut antag_erg);

        assert_eq!(*protag_erg.get(&kind).unwrap_or(&0), 6);
        assert_eq!(*antag_erg.get(&kind).unwrap_or(&0), 9);

        let kind = Kind::Breach;
        GameState::resolve_matchups(&erg_roll, kind, &protag_b_attr, &mut protag_erg, kind, &antag_b_attr, &mut antag_erg);

        assert_eq!(*protag_erg.get(&kind).unwrap_or(&0), 7);
        assert_eq!(*antag_erg.get(&kind).unwrap_or(&0), 8);

        let kind = Kind::Compute;
        GameState::resolve_matchups(&erg_roll, kind, &protag_c_attr, &mut protag_erg, kind, &antag_c_attr, &mut antag_erg);

        assert_eq!(*protag_erg.get(&kind).unwrap_or(&0), 7);
        assert_eq!(*antag_erg.get(&kind).unwrap_or(&0), 8);

        let kind = Kind::Disrupt;
        GameState::resolve_matchups(&erg_roll, kind, &protag_d_attr, &mut protag_erg, kind, &antag_d_attr, &mut antag_erg);

        assert_eq!(*protag_erg.get(&kind).unwrap_or(&0), 8);
        assert_eq!(*antag_erg.get(&kind).unwrap_or(&0), 7);

        let kind = Kind::Disrupt;
        GameState::resolve_matchups(&erg_roll, kind, &protag_d_attr, &mut protag_erg, kind, &antag_d_attr, &mut antag_erg);

        assert_eq!(*protag_erg.get(&kind).unwrap_or(&0), 16);
        assert_eq!(*antag_erg.get(&kind).unwrap_or(&0), 14);

        GameState::resolve_matchups(&erg_roll, Kind::Disrupt, &protag_d_attr, &mut protag_erg, Kind::Compute, &antag_c_attr, &mut antag_erg);

        assert_eq!(*protag_erg.get(&Kind::Compute).unwrap_or(&0), 7);
        assert_eq!(*protag_erg.get(&Kind::Disrupt).unwrap_or(&0), 22);
        assert_eq!(*antag_erg.get(&Kind::Compute).unwrap_or(&0), 17);
        assert_eq!(*antag_erg.get(&Kind::Disrupt).unwrap_or(&0), 14);
    }
}
