#![allow(non_camel_case_types)]

pub mod ffi;
pub mod library;
pub mod rest_interface;
pub mod prelude {
    pub use crate::library::*;
}
