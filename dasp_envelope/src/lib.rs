#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(core_intrinsics))]

pub mod detect;

pub use self::detect::{Detect, Detector};
