#![no_std]
#![macro_use]
#![allow(unused_macros)]

pub use defmt;
pub use log;
pub mod macros;
mod traits;
pub use traits::*;

pub use defmt_or_log_macros::*;

pub const DEFMT: bool = cfg!(feature = "defmt");
