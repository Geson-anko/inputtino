use std::ffi::{c_int, c_void};
use std::path::PathBuf;
use inputtino_sys::{inputtino_joypad_ps5_place_finger, inputtino_joypad_ps5_release_finger};

use crate::{Joypad, JoypadStickPosition};
use crate::common::{get_nodes, make_device, DeviceDefinition};
use crate::sys::{
    inputtino_joypad_ps5_create,
    inputtino_joypad_ps5_destroy,
    inputtino_joypad_ps5_get_nodes,
    inputtino_joypad_ps5_set_on_rumble,
    inputtino_joypad_ps5_set_on_led,
    inputtino_joypad_ps5_set_pressed_buttons,
    inputtino_joypad_ps5_set_stick,
    inputtino_joypad_ps5_set_triggers,
};

pub struct PS5Joypad {
    joypad: *mut crate::sys::InputtinoPS5Joypad,
    on_rumble_fn: Box<dyn FnMut(i32, i32)>,
    on_led_fn: Box<dyn FnMut(i32, i32, i32)>,
}

impl PS5Joypad {
    // TODO: Expose in C API and use the binding.
    pub const TOUCHPAD_WIDTH: i32 = 1920;
    pub const TOUCHPAD_HEIGHT: i32 = 1080;

    pub fn new(device: &DeviceDefinition) -> Result<Self, String> {
        make_device(inputtino_joypad_ps5_create, device)
            .map(|joypad| PS5Joypad {
                joypad,
                on_rumble_fn: Box::new(|_, _| {}),
                on_led_fn: Box::new(|_, _, _| {}),
            })
    }

    pub fn set_on_led(&mut self, on_led_fn: impl FnMut(i32, i32, i32) + 'static) {
        self.on_led_fn = Box::new(on_led_fn);
        unsafe {
            let state_ptr = self as *const _ as *mut c_void;
            inputtino_joypad_ps5_set_on_led(self.joypad, Some(on_led_c_fn), state_ptr);
        }
    }
}

impl Joypad for PS5Joypad {
    fn set_pressed(&self, buttons: i32) {
        unsafe {
            inputtino_joypad_ps5_set_pressed_buttons(self.joypad, buttons);
        }
    }

    fn set_triggers(&self, left_trigger: i16, right_trigger: i16) {
        unsafe {
            inputtino_joypad_ps5_set_triggers(self.joypad, left_trigger, right_trigger);
        }
    }

    fn set_stick(&self, stick_type: JoypadStickPosition, x: i16, y: i16) {
        unsafe {
            inputtino_joypad_ps5_set_stick(self.joypad, stick_type, x, y);
        }
    }

    fn set_on_rumble(&mut self, on_rumble_fn: impl FnMut(i32, i32) + 'static) {
        self.on_rumble_fn = Box::new(on_rumble_fn);
        unsafe {
            let state_ptr = self as *const _ as *mut c_void;
            inputtino_joypad_ps5_set_on_rumble(self.joypad, Some(on_rumble_c_fn), state_ptr);
        }
    }

    fn get_nodes(&self) -> Result<Vec<PathBuf>, String> {
        get_nodes(inputtino_joypad_ps5_get_nodes, self.joypad)
    }

    fn place_finger(&self, finger_id: u32, x: u16, y: u16) {
        unsafe {
            inputtino_joypad_ps5_place_finger(self.joypad, finger_id as i32, x, y);
        }
    }

    fn release_finger(&self, finger_id: u32) {
        unsafe {
            inputtino_joypad_ps5_release_finger(self.joypad, finger_id as i32);
        }
    }
}

impl Drop for PS5Joypad {
    fn drop(&mut self) {
        unsafe {
            inputtino_joypad_ps5_destroy(self.joypad);
        }
    }
}

unsafe extern "C" fn on_rumble_c_fn(left_motor: c_int, right_motor: c_int, user_data: *mut ::core::ffi::c_void) {
    let joypad: &mut PS5Joypad = &mut *(user_data as *mut PS5Joypad);
    (joypad.on_rumble_fn)(left_motor, right_motor);
}

unsafe extern "C" fn on_led_c_fn(r: c_int, g: c_int, b: c_int, user_data: *mut ::core::ffi::c_void) {
    let joypad: &mut PS5Joypad = &mut *(user_data as *mut PS5Joypad);
    (joypad.on_led_fn)(r, g, b);
}

unsafe impl Send for PS5Joypad { }
