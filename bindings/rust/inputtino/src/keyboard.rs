use std::path::PathBuf;

use crate::common::{get_nodes, make_device, DeviceDefinition};
use crate::sys::{
    inputtino_keyboard_create,
    inputtino_keyboard_get_nodes,
    inputtino_keyboard_press,
    inputtino_keyboard_release,
    inputtino_keyboard_destroy
};
use crate::InputtinoError;

pub struct Keyboard {
    kb: *mut crate::sys::InputtinoKeyboard,
}

impl Keyboard {
    pub fn new(device: &DeviceDefinition) -> Result<Self, InputtinoError> {
        make_device(inputtino_keyboard_create, device)
            .map(|kb| Keyboard { kb })
    }

    pub fn get_nodes(&self) -> Result<Vec<PathBuf>, InputtinoError> {
        get_nodes(inputtino_keyboard_get_nodes, self.kb)
    }

    pub fn press_key(&self, key: i16) {
        // TODO: Export key mapping in Rust.
        unsafe {
            inputtino_keyboard_press(self.kb, key);
        }
    }

    pub fn release_key(&self, key: i16) {
        // TODO: Export key mapping in Rust.
        unsafe {
            inputtino_keyboard_release(self.kb, key);
        }
    }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        unsafe {
            inputtino_keyboard_destroy(self.kb);
        }
    }
}

unsafe impl Send for Keyboard { }
