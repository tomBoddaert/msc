//! The library for MatrixStack-Code.
//! This is a 2d, stack-based, esoteric language

#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::perf,
    clippy::cargo
)]
#![cfg_attr(not(feature = "std"), no_std)]

mod number;
pub use number::Number;

pub mod comparator;
pub mod deflector;
pub mod instruction;
pub mod io;
pub mod machine;
pub mod operator;
pub mod plane;
pub mod stack;

#[cfg(feature = "std")]
pub mod load;

pub mod build;

pub type Velocity = u8;
pub type Pointer = (usize, usize);

#[must_use]
pub fn add_velocity_to_pointer(velocity: Velocity, mut pointer: Pointer) -> (usize, usize) {
    let a = if velocity & 0b10 == 0 {
        &mut pointer.0
    } else {
        &mut pointer.1
    };

    if velocity & 0b01 == 0 {
        *a = a.wrapping_add(1);
    } else {
        *a = a.wrapping_sub(1);
    }

    pointer
}
