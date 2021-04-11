use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub const STRENGTH_DICE_SIDES: u32 = 2;

#[derive(PartialEq, Eq, Clone, Copy, Deserialize, Serialize, Default)]
pub struct Strength {
    #[serde(rename = "strength")]
    pub dice: u32,
}

impl Strength {
    pub fn new(dice: u32) -> Strength {
        Strength { dice }
    }

    pub fn roll<R: Rng + ?Sized>(&self, rng: &mut R) -> u32 {
        let fixed_count = self.dice / 2;
        let open_count = self.dice - fixed_count;
        let fixed_value = fixed_count * STRENGTH_DICE_SIDES;
        let open_value = rng.gen_range(open_count, (open_count * STRENGTH_DICE_SIDES) + 1);
        fixed_value + open_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_value() {
        let mut rng = StdRng::seed_from_u64(42);
        let d = Strength::new(14);
        let v = d.roll(&mut rng);
        // 7 * 2 + 7 * (1,2) = 21 - 28
        assert!(v >= 21 && v <= 28);
    }
}
