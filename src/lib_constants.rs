pub const OK: i32 = 0;
pub const INIT_FAILED: i32 = -1;
pub const BAD_USER_GPIO: i32 = -2;
pub const BAD_GPIO: i32 = -3;
pub const BAD_MODE: i32 = -4;
pub const BAD_LEVEL: i32 = -5;
pub const BAD_PUD: i32 = -6;
pub const BAD_PULSEWIDTH: i32 = -7;
pub const BAD_DUTYCYCLE: i32 = -8;
pub const BAD_DUTYRANGE: i32 = -21;
pub const BAD_WAVE_BAUD: i32 = -35;
pub const NOT_SERIAL_GPIO: i32 = -38;
pub const GPIO_IN_USE: i32 = -50;
pub const NOT_PWM_GPIO: i32 = -92;
pub const NOT_SERVO_GPIO: i32 = -93;
pub const BAD_DATABITS: i32 = -101;

pub const DEFAULT_ERROR: &str = "Unknown error.";