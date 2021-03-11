#![no_std]
// The WebAssembly bindings need to provide alloc error handling.
#![cfg_attr(
    target_arch = "wasm32",
    feature(core_intrinsics, lang_items, alloc_error_handler)
)]

#[cfg(target_arch = "wasm32")]
extern crate alloc;

#[cfg(target_arch = "wasm32")]
pub mod wasm32;

mod buffer;
mod pipelines;
mod value;

pub use pipelines::{Sink, Source, Transform};
pub use buffer::Buffer;
pub use value::{Value, Type, AsType};

#[derive(Copy, Clone, Debug)]
pub enum CAPABILITY {
    RAND = 1,
    SOUND = 2,
    ACCEL = 3,
    IMAGE = 4,
    RAW = 5,
}

impl CAPABILITY {
    pub fn from_u32(value: u32) -> CAPABILITY {
        match value {
            1 => CAPABILITY::RAND,
            2 => CAPABILITY::SOUND,
            3 => CAPABILITY::ACCEL,
            4 => CAPABILITY::IMAGE,
            5 => CAPABILITY::RAW,
            _ => CAPABILITY::RAW,
        }
    }

    pub fn from_str(value: &str) -> Option<CAPABILITY> {
        match value {
            "RAND" => Some(CAPABILITY::RAND),
            "SOUND" => Some(CAPABILITY::SOUND),
            "ACCEL" => Some(CAPABILITY::ACCEL),
            "IMAGE" => Some(CAPABILITY::IMAGE),
            "RAW" => Some(CAPABILITY::RAW),
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum OUTPUT {
    SERIAL = 1,
    BLE = 2,
    PIN = 3,
    WIFI = 4,
}

impl OUTPUT {
    pub fn from_u32(value: u32) -> OUTPUT {
        match value {
            1 => OUTPUT::SERIAL,
            2 => OUTPUT::BLE,
            3 => OUTPUT::PIN,
            4 => OUTPUT::WIFI,
            _ => OUTPUT::SERIAL,
        }
    }
}
