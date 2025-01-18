use crate::core::AttributeArray;
use rand::Rng;

pub fn pick_values(rng: &mut impl Rng) -> AttributeArray {
    let v1 = rng.random_range(1..=9);
    let v2 = rng.random_range(1..=9);
    let remain = 20 - v1 - v2;
    let v3_lower = remain.max(10) - 9;
    let v3_upper = (remain - 1).min(9);
    let v3 = rng.random_range(v3_lower..=v3_upper);
    let v4 = remain - v3;

    [v1, v2, v3, v4]
}

#[cfg(test)]
mod player_builder_test {
    use crate::util;
    use rand::prelude::StdRng;
    use rand::SeedableRng;

    #[test]
    fn test_pick_values() -> Result<(), String> {
        let mut rng = StdRng::seed_from_u64(0x1234567890ABCDEF);

        let values = util::pick_values(&mut rng);
        assert_eq!(values.iter().sum::<u8>(), 20);
        Ok(())
    }
}
