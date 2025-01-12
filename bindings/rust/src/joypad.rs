use strum_macros::FromRepr;

use crate::{
    DeviceDefinition, JoypadStickPosition, PS5Joypad, SwitchJoypad, XboxOneJoypad
};

#[derive(Debug, FromRepr)]
#[repr(u8)]
pub enum JoypadKind {
	Unknown = 0x00,
	Xbox = 0x01,
	PlayStation = 0x02,
	Nintendo = 0x03,
}

#[repr(u8)]
pub enum Joypad {
    XOne(XboxOneJoypad),
    PS5(PS5Joypad),
    Switch(SwitchJoypad),
}

impl Joypad {
    pub fn new(kind: &JoypadKind, device: &DeviceDefinition) -> Result<Joypad, String> {
        match kind {
            JoypadKind::Unknown => XboxOneJoypad::new(device).map(Joypad::XOne),
            JoypadKind::Xbox => XboxOneJoypad::new(device).map(Joypad::XOne),
            JoypadKind::PlayStation => PS5Joypad::new(device).map(Joypad::PS5),
            JoypadKind::Nintendo => SwitchJoypad::new(device).map(Joypad::Switch),
        }
    }

    pub fn new_xone(device: &DeviceDefinition) -> Result<Joypad, String> {
        Joypad::new(&JoypadKind::Xbox, device)
    }

    pub fn new_ps5(device: &DeviceDefinition) -> Result<Joypad, String> {
        Joypad::new(&JoypadKind::PlayStation, device)
    }

    pub fn new_switch(device: &DeviceDefinition) -> Result<Joypad, String> {
        Joypad::new(&JoypadKind::Xbox, device)
    }

    pub fn set_pressed(&self, buttons: i32) {
        match self {
            Joypad::XOne(joypad) => joypad.set_pressed(buttons),
            Joypad::PS5(joypad) => joypad.set_pressed(buttons),
            Joypad::Switch(joypad) => joypad.set_pressed(buttons),
        }
    }

    pub fn set_triggers(&self, left_trigger: i16, right_trigger: i16) {
        match self {
            Joypad::XOne(joypad) => joypad.set_triggers(left_trigger, right_trigger),
            Joypad::PS5(joypad) => joypad.set_triggers(left_trigger, right_trigger),
            Joypad::Switch(joypad) => joypad.set_triggers(left_trigger, right_trigger),
        }
    }

    pub fn set_stick(&self, stick_type: JoypadStickPosition, x: i16, y: i16) {
        match self {
            Joypad::XOne(joypad) => joypad.set_stick(stick_type, x, y),
            Joypad::PS5(joypad) => joypad.set_stick(stick_type, x, y),
            Joypad::Switch(joypad) => joypad.set_stick(stick_type, x, y),
        }
    }
}
