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

mod joypad_nintendo;
pub use joypad_nintendo::SwitchJoypad;

pub use joypad_xbox::XboxOneJoypad;
mod joypad_xbox;

// Low level automatic c bindings.
mod ffi;
pub use ffi::{
    INPUTTINO_JOYPAD_STICK_POSITION as JoypadStickPosition,
    INPUTTINO_JOYPAD_BTN as JoypadButton,
    INPUTTINO_JOYPAD_MOTION_TYPE as JoypadMotionType,
    INPUTTINO_MOUSE_BUTTON as MouseButton,
};
