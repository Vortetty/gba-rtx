use crate::math::types::FixFlt;

pub const GBA_SCREEN_X_I32: i32 = 240;
pub const GBA_SCREEN_Y_I32: i32 = 160;
pub const GBA_SCREEN_X: FixFlt = FixFlt::lit("240");
pub const GBA_SCREEN_Y: FixFlt = FixFlt::lit("160");
pub const GBA_SCREEN_1_OVER_X: FixFlt = FixFlt::lit("0.0041666");
pub const GBA_SCREEN_1_OVER_Y: FixFlt = FixFlt::lit("0.00625");

pub const FixedType_VAL_1: FixFlt = FixFlt::lit("1");
