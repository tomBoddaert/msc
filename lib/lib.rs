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

#[cfg(test)]
mod test {
    use super::add_velocity_to_pointer;

    macro_rules! add_velocity_to_pointer_tests {
        ( $name:ident, $pointer:expr, $(( $test:literal, $expected:expr )),* , ) => {
            #[test]
            fn $name() {
                let pointer = $pointer;

                let tests = [
                    $(($test, $expected)),*
                ];

                for (velocity, expected) in tests {
                    let new_pointer = add_velocity_to_pointer(velocity, pointer);
                    assert_eq!(new_pointer, expected, "Add velocity to pointer made {pointer:?} -{velocity:0>2b}-> {new_pointer:?} rather than {pointer:?} -{velocity:0>2b}-> {expected:?}");
                }
            }
        };
    }

    add_velocity_to_pointer_tests!(
        from_non_zero,
        (5, 5),
        (0b00, (6, 5)), // Right
        (0b01, (4, 5)), // Left
        (0b10, (5, 6)), // Down
        (0b11, (5, 4)), // Up
    );

    add_velocity_to_pointer_tests!(
        from_zero,
        (0, 0),
        (0b00, (1, 0)),          // Right
        (0b01, (usize::MAX, 0)), // Left
        (0b10, (0, 1)),          // Down
        (0b11, (0, usize::MAX)), // Up
    );

    add_velocity_to_pointer_tests!(
        from_max,
        (usize::MAX, usize::MAX),
        (0b00, (0, usize::MAX)),              // Right
        (0b01, (usize::MAX - 1, usize::MAX)), // Left
        (0b10, (usize::MAX, 0)),              // Down
        (0b11, (usize::MAX, usize::MAX - 1)), // Up
    );
}
