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

#[cfg(test)]
mod test {
    use crate::stack::test_stacks::{FakeStack, SinglePopStack};

    use super::Comparator;

    macro_rules! comp_test {
        ( $name:ident, $comp:path, $reg:expr, $stack:expr, $(( $test:literal, $expected:literal )),* , ) => {
            #[test]
            fn $name() {
                let register = $reg;

                let tests = [
                    $(($test, $expected)),*
                ];

                for (velocity, expected) in tests {
                    let mut stack = $stack;
                    let new_velocity = $comp.apply(&register, &mut stack, velocity);
                    assert_eq!(new_velocity, expected, "{} comparison redirected {velocity:0>2b} -> {new_velocity:0>2b} rather than {velocity:0>2b} -> {expected:0>2b}", stringify!($name));
                }
            }
        };
    }

    // Test that x < 0 redirects correctly
    comp_test!(
        zero_less,
        Comparator::Zero,
        -1,
        FakeStack::new(),
        (0b00, 0b10), // Right -> Down
        (0b01, 0b11), // Left -> Up
        (0b10, 0b01), // Down -> Left
        (0b11, 0b00), // Up -> Right
    );

    // Test that x == 0 redirects correctly
    comp_test!(
        zero_equal,
        Comparator::Zero,
        0,
        FakeStack::new(),
        (0b00, 0b00), // Right -> Right
        (0b01, 0b01), // Left -> Left
        (0b10, 0b10), // Down -> Down
        (0b11, 0b11), // Up -> Up
    );

    // Test that x > 0 redirects correctly
    comp_test!(
        zero_greater,
        Comparator::Zero,
        1,
        FakeStack::new(),
        (0b00, 0b11), // Right -> Up
        (0b01, 0b10), // Left -> Down
        (0b10, 0b00), // Down -> Right
        (0b11, 0b01), // Up -> Left
    );

    // Test that x < stack redirects correctly
    comp_test!(
        stack_less,
        Comparator::Stack,
        2,
        SinglePopStack::new(5),
        (0b00, 0b10), // Right -> Down
        (0b01, 0b11), // Left -> Up
        (0b10, 0b01), // Down -> Left
        (0b11, 0b00), // Up -> Right
    );

    // Test that x == stack redirects correctly
    comp_test!(
        stack_equal,
        Comparator::Stack,
        5,
        SinglePopStack::new(5),
        (0b00, 0b00), // Right -> Right
        (0b01, 0b01), // Left -> Left
        (0b10, 0b10), // Down -> Down
        (0b11, 0b11), // Up -> Up
    );

    // Test that x > stack redirects correctly
    comp_test!(
        stack_greater,
        Comparator::Stack,
        8,
        SinglePopStack::new(5),
        (0b00, 0b11), // Right -> Up
        (0b01, 0b10), // Left -> Down
        (0b10, 0b00), // Down -> Right
        (0b11, 0b01), // Up -> Left
    );
}
