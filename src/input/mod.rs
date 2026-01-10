pub mod evdev_input;

pub use evdev_input::{
    find_device, find_keyboard, find_touchpad, get_touchpad_dimensions, DeviceType,
    TouchpadDimensions,
};
