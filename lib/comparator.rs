//! `MSCode` instructions for comparisons

use core::cmp::Ordering;

use crate::{stack::Stack, Number, Velocity};

#[derive(Clone, Copy)]
pub enum Comparator {
    Zero,
    Stack,
}

impl Comparator {
    pub fn apply<N: Number, StackType: Stack<Item = N>>(
        self,
        register: &N,
        stack: &mut StackType,
        velocity: Velocity,
    ) -> Velocity {
        use Comparator::{Stack, Zero};
        match self {
            // Compare register with 0
            Zero => match register.cmp(&N::ZERO) {
                Ordering::Equal => velocity,
                Ordering::Less => velocity ^ 0b10 ^ ((velocity >> 1) & 0b01),
                Ordering::Greater => velocity ^ 0b11 ^ ((velocity >> 1) & 0b01),
            },
            // Compare register with the top of the underlying stack
            Stack => match register.cmp(&stack.pop().unwrap_or_default()) {
                Ordering::Equal => velocity,
                Ordering::Less => velocity ^ 0b10 ^ ((velocity >> 1) & 0b01),
                Ordering::Greater => velocity ^ 0b11 ^ ((velocity >> 1) & 0b01),
            },
        }
    }
}

impl From<Comparator> for char {
    fn from(value: Comparator) -> Self {
        use Comparator::{Stack, Zero};
        match value {
            Zero => 'z',
            Stack => 'c',
        }
    }
}
