use std::path::PathBuf;

use crate::JoypadStickPosition;

pub trait Joypad: Send {
    fn set_pressed(&self, buttons: i32);

    fn set_triggers(&self, left_trigger: i16, right_trigger: i16);

    fn set_stick(&self, stick_type: JoypadStickPosition, x: i16, y: i16);

    fn set_on_rumble(&mut self, on_rumble_fn: impl FnMut(i32, i32) + 'static);

    fn get_nodes(&self) -> Result<Vec<PathBuf>, String>;
}
