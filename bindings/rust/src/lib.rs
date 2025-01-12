mod common;
pub use common::DeviceDefinition;

mod mouse;
pub use mouse::Mouse;

mod keyboard;
pub use keyboard::Keyboard;

mod joypad;
pub use joypad::{JoypadKind, Joypad};

mod joypad_ps5;
pub use joypad_ps5::PS5Joypad;

mod joypad_switch;
pub use joypad_switch::SwitchJoypad;

pub use joypad_xone::XOneJoypad;
mod joypad_xone;

// Private low level automatic c bindings
mod c_bindings;
pub use c_bindings::{
    INPUTTINO_JOYPAD_STICK_POSITION as JoypadStickPosition,
    INPUTTINO_JOYPAD_BTN as JoypadButton,
    INPUTTINO_JOYPAD_MOTION_TYPE as JoypadMotionType,
    INPUTTINO_MOUSE_BUTTON as MouseButton,
};
