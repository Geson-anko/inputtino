use crate::{JoypadStickPosition, PS5Joypad, SwitchJoypad, XboxOneJoypad};

pub enum Joypad {
    Switch(SwitchJoypad),
    PS5(PS5Joypad),
    XboxOne(XboxOneJoypad),
}

impl Joypad {
    pub fn set_pressed(&self, buttons: i32) {
        match self {
            Joypad::Switch(joypad) => joypad.set_pressed(buttons),
            Joypad::PS5(joypad) => joypad.set_pressed(buttons),
            Joypad::XboxOne(joypad) => joypad.set_pressed(buttons),
        }
    }

    pub fn set_triggers(&self, left_trigger: i16, right_trigger: i16) {
        match self {
            Joypad::Switch(joypad) => joypad.set_triggers(left_trigger, right_trigger),
            Joypad::PS5(joypad) => joypad.set_triggers(left_trigger, right_trigger),
            Joypad::XboxOne(joypad) => joypad.set_triggers(left_trigger, right_trigger),
        }
    }

    pub fn set_stick(&self, stick_type: JoypadStickPosition, x: i16, y: i16) {
        match self {
            Joypad::Switch(joypad) => joypad.set_stick(stick_type, x, y),
            Joypad::PS5(joypad) => joypad.set_stick(stick_type, x, y),
            Joypad::XboxOne(joypad) => joypad.set_stick(stick_type, x, y),
        }
    }

    pub fn set_on_rumble(&mut self, on_rumble_fn: impl FnMut(i32, i32) + 'static) {
        match self {
            Joypad::Switch(joypad) => joypad.set_on_rumble(on_rumble_fn),
            Joypad::PS5(joypad) => joypad.set_on_rumble(on_rumble_fn),
            Joypad::XboxOne(joypad) => joypad.set_on_rumble(on_rumble_fn),
        }
    }
}
