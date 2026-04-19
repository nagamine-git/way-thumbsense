pub mod evdev_input;

pub use evdev_input::{
    find_device, find_keyboard, find_touchpad, find_touchpad_with_name, get_touchpad_dimensions,
    DeviceType, TouchpadDimensions,
};
