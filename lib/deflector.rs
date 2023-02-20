//! `MSCode` instructions for changing direction

use crate::Velocity;

#[derive(Clone, Copy)]
pub enum Deflector {
    RightArrow,
    LeftArrow,
    UpArrow,
    DownArrow,
    OmniMirror,
    ForwardMirror,
    BackMirror,
}

impl Deflector {
    #[must_use]
    pub const fn apply(self, velocity: Velocity) -> Velocity {
        use Deflector::{
            BackMirror, DownArrow, ForwardMirror, LeftArrow, OmniMirror, RightArrow, UpArrow,
        };
        match self {
            RightArrow => 0b00,
            LeftArrow => 0b01,
            DownArrow => 0b10,
            UpArrow => 0b11,
            OmniMirror => velocity ^ 0b01,
            BackMirror => velocity ^ 0b10,
            ForwardMirror => velocity ^ 0b11,
        }
    }
}

impl From<Deflector> for char {
    fn from(val: Deflector) -> Self {
        use Deflector::{
            BackMirror, DownArrow, ForwardMirror, LeftArrow, OmniMirror, RightArrow, UpArrow,
        };
        match val {
            RightArrow => '>',
            LeftArrow => '<',
            UpArrow => '^',
            DownArrow => 'v',
            OmniMirror => 'o',
            ForwardMirror => '/',
            BackMirror => '\\',
        }
    }
}

#[cfg(test)]
mod test {
    use super::Deflector;

    macro_rules! arrow_tests {
        ( $(( $name:ident, $arrow:path, $expected:literal )),* , ) => {
            $(
                #[test]
                fn $name() {
                    for velocity in 0..4 {
                        let new_velocity = $arrow.apply(velocity);
                        assert_eq!(new_velocity, $expected, "{} arrow redirected {velocity:0>2b} -> {new_velocity:0>2b} rather than {velocity:0>2b} -> {:0>2b}", stringify!($name), $expected)
                    }
                }
            )*
        };
    }

    // Test all the arrows
    arrow_tests!(
        (arrow_right, Deflector::RightArrow, 0b00),
        (arrow_left, Deflector::LeftArrow, 0b01),
        (arrow_down, Deflector::DownArrow, 0b10),
        (arrow_up, Deflector::UpArrow, 0b11),
    );

    macro_rules! mirror_test {
        ( $name:ident, $mirror:path, $(( $test:literal, $expected:literal )),* , ) => {
            #[test]
            fn $name() {
                let tests = [
                    $(($test, $expected)),*
                ];

                for (velocity, expected) in tests {
                    let new_velocity = $mirror.apply(velocity);
                    assert_eq!(new_velocity, expected, "{} mirror redirected {velocity:0>2b} -> {new_velocity:0>2b} rather than {velocity:0>2b} -> {expected:0>2b}", stringify!($name));
                }
            }
        };
    }

    // Test that omni mirror redirects correctly
    mirror_test!(
        mirror_omni,
        Deflector::OmniMirror,
        (0b00, 0b01), // Right -> Left
        (0b01, 0b00), // Left -> Right
        (0b10, 0b11), // Down -> Up
        (0b11, 0b10), // Up -> Down
    );

    // Test that forward mirror redirects correctly
    mirror_test!(
        mirror_forward,
        Deflector::ForwardMirror,
        (0b00, 0b11), // Right -> Up
        (0b01, 0b10), // Left -> Down
        (0b10, 0b01), // Down -> Left
        (0b11, 0b00), // Up -> Right
    );

    // Test that back mirror redirects correctly
    mirror_test!(
        mirror_back,
        Deflector::BackMirror,
        (0b00, 0b10), // Right -> Down
        (0b01, 0b11), // Left -> Up
        (0b10, 0b00), // Down -> Right
        (0b11, 0b01), // Up -> Left
    );
}
