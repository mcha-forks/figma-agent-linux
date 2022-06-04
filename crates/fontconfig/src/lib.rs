use std::ptr;

pub use fontconfig_sys::*;

pub mod config;
pub mod font_set;
pub mod object_set;
pub mod pattern;
pub mod str_list;
pub mod str_set;

pub use crate::config::*;
pub use crate::font_set::*;
pub use crate::object_set::*;
pub use crate::pattern::*;
pub use crate::str_list::*;
pub use crate::str_set::*;

pub fn init() -> Option<Config> {
    let result = unsafe { FcInit() };
    if result != FcFalse {
        Some(unsafe { Config::from_raw(ptr::null_mut()) })
    } else {
        None
    }
}
