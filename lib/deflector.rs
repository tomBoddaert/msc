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
