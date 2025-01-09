use crate::math::types::FixFlt;

pub const GBA_SCREEN_X_I32: i32 = 240;
pub const GBA_SCREEN_Y_I32: i32 = 160;
pub const GBA_SCREEN_X: FixFlt = FixFlt::from_f32(240.0);
pub const GBA_SCREEN_Y: FixFlt = FixFlt::from_f32(160.0);
pub const GBA_SCREEN_1_OVER_X: FixFlt = FixFlt::from_f32(0.0041666);
pub const GBA_SCREEN_1_OVER_Y: FixFlt = FixFlt::from_f32(0.00625);
