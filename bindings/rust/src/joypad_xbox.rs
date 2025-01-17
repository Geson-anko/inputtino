use std::ffi::{c_int, c_void};
use crate::{get_nodes, make_device, JoypadStickPosition};
use crate::common::{DeviceDefinition, error_handler_fn};
use crate::ffi::{
    inputtino_joypad_xone_create,
    inputtino_joypad_xone_destroy,
    inputtino_joypad_xone_get_nodes,
    inputtino_joypad_xone_set_on_rumble,
    inputtino_joypad_xone_set_pressed_buttons,
    inputtino_joypad_xone_set_stick,
    inputtino_joypad_xone_set_triggers,
};

pub struct XboxOneJoypad {
    joypad: *mut crate::ffi::InputtinoXOneJoypad,
    on_rumble_fn: Box<dyn FnMut(i32, i32) -> ()>,
}

impl XboxOneJoypad {
    pub fn new(device: &DeviceDefinition) -> Result<Self, String> {
        unsafe {
            let dev = make_device!(inputtino_joypad_xone_create, device);
            match dev {
                Ok(joypad) => {
                    Ok(XboxOneJoypad { joypad, on_rumble_fn: Box::new(|_, _| {}) })
                }
                Err(e) => Err(e),
            }
        }
    }

    pub fn get_nodes(&self) -> Result<Vec<String>, String> {
        unsafe {
            get_nodes!(inputtino_joypad_xone_get_nodes, self.joypad)
        }
    }

    pub fn set_pressed(&self, buttons: i32) {
        unsafe {
            inputtino_joypad_xone_set_pressed_buttons(self.joypad, buttons);
        }
    }

    pub fn set_triggers(&self, left_trigger: i16, right_trigger: i16) {
        unsafe {
            inputtino_joypad_xone_set_triggers(self.joypad, left_trigger, right_trigger);
        }
    }

    pub fn set_stick(&self, stick_type: JoypadStickPosition, x: i16, y: i16) {
        unsafe {
            inputtino_joypad_xone_set_stick(self.joypad, stick_type, x, y);
        }
    }

    pub fn set_on_rumble(&mut self, on_rumble_fn: impl FnMut(i32, i32) -> () + 'static) {
        self.on_rumble_fn = Box::new(on_rumble_fn);
        unsafe {
            let state_ptr = self as *const _ as *mut c_void;
            inputtino_joypad_xone_set_on_rumble(self.joypad, Some(on_rumble_c_fn), state_ptr);
        }
    }
}

impl Drop for XboxOneJoypad {
    fn drop(&mut self) {
        unsafe {
            inputtino_joypad_xone_destroy(self.joypad);
        }
    }
}

#[allow(dead_code)]
pub unsafe extern "C" fn on_rumble_c_fn(left_motor: c_int, right_motor: c_int, user_data: *mut ::core::ffi::c_void) {
    let joypad: &mut XboxOneJoypad = &mut *(user_data as *mut XboxOneJoypad);
    ((*joypad).on_rumble_fn)(left_motor, right_motor);
}

unsafe impl Send for XboxOneJoypad { }
