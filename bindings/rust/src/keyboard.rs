use crate::common::{make_device, DeviceDefinition};
use crate::get_nodes;
use crate::ffi::{inputtino_keyboard_create, inputtino_keyboard_get_nodes, inputtino_keyboard_press, inputtino_keyboard_release, inputtino_keyboard_destroy};

pub struct Keyboard {
    kb: *mut crate::ffi::InputtinoKeyboard,
}

impl Keyboard {
    pub fn new(device: &DeviceDefinition) -> Result<Self, String> {
        make_device(inputtino_keyboard_create, device)
            .map(|kb| Keyboard { kb })
    }

    pub fn get_nodes(&self) -> Result<Vec<String>, String> {
        unsafe {
            get_nodes!(inputtino_keyboard_get_nodes, self.kb)
        }
    }

    pub fn press_key(&self, key: i16) {
        unsafe {
            inputtino_keyboard_press(self.kb, key);
        }
    }

    pub fn release_key(&self, key: i16) {
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
