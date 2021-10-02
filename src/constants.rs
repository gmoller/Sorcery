pub(crate) const HALF: f32 = 0.5;
pub(crate) const SCALE: (f32, f32) = (1.0, 1.0);

pub(crate) const UNIT_MOVEMENT_SPEED: f32 = 400.0 * SCALE.0;

//pub(crate) const HEX_SIZE: (f32, f32) = (32.0, 32.0);
//pub(crate) const HEX_SIZE: (f32, f32) = (64.0, 64.0);
//pub(crate) const HEX_SIZE: (f32, f32) = (128.0, 128.0);
pub(crate) const HEX_SIZE: (f32, f32) = (256.0, 256.0);
pub(crate) const HEX_EXTRA_Y: f32 = HEX_SIZE.1 * HALF;
pub(crate) const LAYOUT_SIZE: (f32, f32) = (HEX_SIZE.0 * 0.57421875, (HEX_SIZE.1 + HEX_EXTRA_Y) * 0.3351);

pub(crate) const UNIT_FRAME_INACTIVE: u8 = 0;
pub(crate) const UNIT_FRAME_HOVERED: u8 = 1;
pub(crate) const UNIT_FRAME_ACTIVE: u8 = 2;
pub(crate) const UNIT_HP_FILL: u8 = 3;
pub(crate) const BACKLIGHT: u8 = 4;
// pub(crate) const BACKDROP_BLACK: u8 = 5;
// pub(crate) const BACKDROP_BROWN: u8 = 6;
// pub(crate) const BACKDROP_DARKRED: u8 = 7;
// pub(crate) const BACKDROP_GOLD: u8 = 8;
// pub(crate) const BACKDROP_GRAY_25: u8 = 9;
// pub(crate) const BACKDROP_GRAY_50: u8 = 10;
// pub(crate) const BACKDROP_GREEN: u8 = 11;
pub(crate) const BACKDROP_INDIGO: u8 = 12;
// pub(crate) const BACKDROP_LIGHTYELLOW: u8 = 13;
// pub(crate) const BACKDROP_ORANGE: u8 = 14;
// pub(crate) const BACKDROP_PURPLE: u8 = 15;
// pub(crate) const BACKDROP_RED: u8 = 16;
// pub(crate) const BACKDROP_ROSE: u8 = 17;
// pub(crate) const BACKDROP_TURQUOISE: u8 = 18;
// pub(crate) const BACKDROP_WHITE: u8 = 19;
// pub(crate) const BACKDROP_YELLOW: u8 = 20;

pub(crate) const UNIT_ICON_SETTLERS_TRANSPARENT: u8 = 21;
pub(crate) const UNIT_ICON_BARBARIAN_SPEARMEN_TRANSPARENT: u8 = 22;
pub(crate) const UNIT_ICON_BARBARIAN_SWORDSMEN_TRANSPARENT: u8 = 23;

//pub(crate) const CROSSHAIR: &str = "images/crosshair.png";
