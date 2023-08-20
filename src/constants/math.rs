//! Functions allowing calculation of the resulting values of formulas used by
//! game mechanics related to constant values.

use crate::constants::*;

/// Provides the total number of control points needed to achieve a given Global
/// Control Level
///
/// Calculates the total number of control points needed to achieve a given
/// Global Control Level. The game's API only exposes current level plus
/// progress toward the next level; this allows you to see much many points
/// you've spent to achieve your current level
///
/// [Code reference](https://github.com/screeps/engine/blob/6d498f2f0db4e0744fa6bf8563836d36b49b6a29/src/game/game.js#L117)
#[must_use]
pub fn control_points_for_gcl(level: u32) -> f64 {
    f64::from(level - 1).powf(GCL_POW) * f64::from(GCL_MULTIPLY)
}

/// Provides the total number of processed power needed to achieve a given
/// Global Power Level
///
/// Calculates the total number of power that need to be processed to achieve a
/// given Global Power Level. The game's API only exposes current level plus
/// progress toward the next level; this allows you to see how much you
/// processed to achieve your current level
///
/// [Code reference](https://github.com/screeps/engine/blob/6d498f2f0db4e0744fa6bf8563836d36b49b6a29/src/game/game.js#L120)
#[must_use]
pub const fn power_for_gpl(level: u32) -> u128 {
    (level as u128).pow(POWER_LEVEL_POW) * POWER_LEVEL_MULTIPLY as u128
}

#[cfg(test)]
mod test {
    use assert_approx_eq::assert_approx_eq;

    use super::{control_points_for_gcl, power_for_gpl};

    #[test]
    fn gcl_formula() {
        // the sanity of these values has been validated up to GCL 33
        // on the MMO game server
        assert_approx_eq!(control_points_for_gcl(1), 0.);
        assert_approx_eq!(control_points_for_gcl(2), 1_000_000.);
        assert_approx_eq!(control_points_for_gcl(3), 5_278_031.643_091_577);
        assert_approx_eq!(control_points_for_gcl(4), 13_966_610.165_238_237);
        assert_approx_eq!(control_points_for_gcl(5), 27_857_618.025_475_968);
        assert_approx_eq!(control_points_for_gcl(6), 47_591_348.467_896_95);
        assert_approx_eq!(control_points_for_gcl(7), 73_716_210.398_851_89);
        assert_approx_eq!(control_points_for_gcl(8), 106_717_414.799_656_2);
        assert_approx_eq!(control_points_for_gcl(9), 147_033_389.439_620_47);
        assert_approx_eq!(control_points_for_gcl(10), 195_066_199.507_736_03);
        assert_approx_eq!(control_points_for_gcl(11), 251_188_643.150_957_97);
        assert_approx_eq!(control_points_for_gcl(12), 315_749_334.868_743_6);
        assert_approx_eq!(control_points_for_gcl(13), 389_076_491.093_936_56);
        assert_approx_eq!(control_points_for_gcl(14), 471_480_836.665_255_37);
        assert_approx_eq!(control_points_for_gcl(15), 563_257_892.181_514_7);
        assert_approx_eq!(control_points_for_gcl(16), 664_689_811.289_124_7);
        assert_approx_eq!(control_points_for_gcl(17), 776_046_882.053_323_6);
        assert_approx_eq!(control_points_for_gcl(18), 897_588_771.961_744_3);
        assert_approx_eq!(control_points_for_gcl(19), 1_029_565_573.499_445_2);
        assert_approx_eq!(control_points_for_gcl(20), 1_172_218_691.999_976_2);
        assert_approx_eq!(control_points_for_gcl(25), 2_053_558_031.576_835_2);
        assert_approx_eq!(control_points_for_gcl(30), 3_234_113_036.195_188_5);
        assert_approx_eq!(control_points_for_gcl(31), 3_508_253_856.824_569);
        assert_approx_eq!(control_points_for_gcl(32), 3_795_491_867.419_434_5);
        assert_approx_eq!(control_points_for_gcl(33), 4_095_999_999.999_998_6);
        assert_approx_eq!(control_points_for_gcl(34), 4_409_947_870.045_006);
        assert_approx_eq!(control_points_for_gcl(35), 4_737_501_940.897_796);
        assert_approx_eq!(control_points_for_gcl(40), 6_584_989_046.083_984);
        assert_approx_eq!(control_points_for_gcl(45), 8_796_024_362.571_56);
        assert_approx_eq!(control_points_for_gcl(50), 11_388_606_621.521_88);
        assert_approx_eq!(control_points_for_gcl(100), 61_592_022_749.941_284);
        assert_approx_eq!(control_points_for_gcl(1000), 15_810_921_110_646.998);
        assert_approx_eq!(control_points_for_gcl(u32::MAX), 1.315_538_815_090_698_2e29);
    }

    #[test]
    #[should_panic]
    fn bad_gcl_formula_input() {
        // players cannot be GCL 0, and subtracting 1 (as the formula does)
        // overflows the u32 - this should panic.
        control_points_for_gcl(0);
    }

    #[test]
    fn gpl_formula() {
        // the sanity of these values has been validated up to GCL 33
        // on the MMO game server
        assert_eq!(power_for_gpl(0), 0);
        assert_eq!(power_for_gpl(1), 1_000);
        assert_eq!(power_for_gpl(2), 4_000);
        assert_eq!(power_for_gpl(3), 9_000);
        assert_eq!(power_for_gpl(4), 16_000);
        assert_eq!(power_for_gpl(5), 25_000);
        assert_eq!(power_for_gpl(6), 36_000);
        assert_eq!(power_for_gpl(7), 49_000);
        assert_eq!(power_for_gpl(8), 64_000);
        assert_eq!(power_for_gpl(9), 81_000);
        assert_eq!(power_for_gpl(10), 100_000);
        assert_eq!(power_for_gpl(50), 2_500_000);
        assert_eq!(power_for_gpl(100), 10_000_000);
        assert_eq!(power_for_gpl(1_000), 1_000_000_000);
        assert_eq!(power_for_gpl(5_000), 25_000_000_000);
        assert_eq!(power_for_gpl(10_000), 100_000_000_000);
        assert_eq!(power_for_gpl(50_000), 2_500_000_000_000);
        assert_eq!(power_for_gpl(100_000), 10_000_000_000_000);
        assert_eq!(power_for_gpl(1_000_000), 1_000_000_000_000_000);
        assert_eq!(power_for_gpl(5_000_000), 25_000_000_000_000_000);
        assert_eq!(power_for_gpl(10_000_000), 100_000_000_000_000_000);
        assert_eq!(power_for_gpl(100_000_000), 10_000_000_000_000_000_000);
        // beyond this value the return overflows a u64
        assert_eq!(power_for_gpl(135_818_791), 18_446_743_988_701_681_000);
        // must be u128 return to fit this one!
        assert_eq!(power_for_gpl(135_818_792), 18_446_744_260_339_264_000);
        assert_eq!(power_for_gpl(1_000_000_000), 1_000_000_000_000_000_000_000);
        assert_eq!(power_for_gpl(4_000_000_000), 16_000_000_000_000_000_000_000);
        assert_eq!(power_for_gpl(u32::MAX), 18_446_744_065_119_617_025_000);
    }
}
