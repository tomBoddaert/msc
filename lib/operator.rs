//! `MSCode` instructions for numerical and bitwise operations

use crate::{stack::Stack, Number};

#[derive(Clone, Copy)]
pub enum Operator {
    Push,
    Pop,
    Duplicate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Or,
    And,
    Xor,
}

impl Operator {
    #[must_use]
    pub fn apply<N: Number, StackType: Stack<Item = N>>(
        self,
        register: N,
        stack: &mut StackType,
    ) -> N {
        use Operator::{Add, And, Divide, Duplicate, Multiply, Not, Or, Pop, Push, Subtract, Xor};
        match self {
            Push => {
                stack.push(register);
                register
            }
            Pop => stack.pop().unwrap_or_default(),
            Duplicate => {
                let value = stack.pop().map_or(N::ZERO, |value| {
                    stack.push(value);
                    value
                });
                stack.push(value);
                register
            }
            Add => register.add(stack.pop().unwrap_or_default()),
            Subtract => register.sub(stack.pop().unwrap_or_default()),
            Multiply => register.mul(stack.pop().unwrap_or(N::ONE)),
            Divide => {
                let mut rhs = stack.pop().unwrap_or(N::ONE);
                if rhs == N::ZERO {
                    rhs = N::ONE;
                }
                register.div(rhs)
            }
            Not => register.not(),
            Or => register.bitor(stack.pop().unwrap_or_default()),
            And => register.bitand(stack.pop().unwrap_or_default()),
            Xor => register.bitxor(stack.pop().unwrap_or_default()),
        }
    }
}

impl From<Operator> for char {
    fn from(val: Operator) -> Self {
        use Operator::{Add, And, Divide, Duplicate, Multiply, Not, Or, Pop, Push, Subtract, Xor};
        match val {
            Push => ',',
            Pop => '.',
            Duplicate => 'd',
            Add => '+',
            Subtract => '-',
            Multiply => '*',
            Divide => '~',
            Not => '!',
            Or => '|',
            And => '&',
            Xor => ':',
        }
    }
}

#[cfg(test)]
mod test {
    use crate::stack::{test_stacks::TestVecStack, Stack};

    use super::Operator;

    macro_rules! setup_stack {
        ( $name:ident, [] ) => {
            let mut $name = TestVecStack::new();
        };
        ( $name:ident, [$( $item:literal ),*] ) => {
            let mut $name = TestVecStack::new();
            $( $name.push( $item ) );*
        };
    }

    macro_rules! operation_test {
        ( $name:ident, $operation:path, $reg:literal, [$( $stack:literal ),*], $expected_reg:literal, $expected_stack:pat ) => {
            #[test]
            fn $name() {
                setup_stack!(stack, [$( $stack ),*]);

                let new_reg = $operation.apply($reg, &mut stack);
                let stack = stack.destructure();

                assert_eq!(new_reg, $expected_reg);
                assert!(matches!(stack[..], $expected_stack));
            }
        };
    }

    operation_test!(push_empty, Operator::Push, 5, [], 5, [5]);
    operation_test!(push_non_empty, Operator::Push, 5, [10], 5, [10, 5]);

    operation_test!(pop_empty, Operator::Pop, 5, [], 0, []);
    operation_test!(pop_non_empty, Operator::Pop, 5, [20, 10], 10, [20]);

    operation_test!(duplicate_empty, Operator::Duplicate, 5, [], 5, [0]);
    operation_test!(
        duplicate_non_empty,
        Operator::Duplicate,
        5,
        [10],
        5,
        [10, 10]
    );

    operation_test!(add_empty, Operator::Add, 5, [], 5, []);
    operation_test!(add_non_empty, Operator::Add, 5, [20, 10], 15, [20]);

    operation_test!(subtract_empty, Operator::Subtract, 5, [], 5, []);
    operation_test!(
        subtract_non_empty,
        Operator::Subtract,
        5,
        [20, 10],
        -5,
        [20]
    );

    operation_test!(multiply_empty, Operator::Multiply, 5, [], 5, []);
    operation_test!(
        multiply_non_empty,
        Operator::Multiply,
        5,
        [20, 10],
        50,
        [20]
    );

    operation_test!(divide_empty, Operator::Divide, 5, [], 5, []);
    operation_test!(divide_non_empty, Operator::Divide, 10, [20, 2], 5, [20]);
    operation_test!(divide_zero, Operator::Divide, 5, [0], 5, []);

    operation_test!(not, Operator::Not, 0b01100011u8, [], 0b10011100, []);

    operation_test!(or_empty, Operator::Or, 0b00111100u8, [], 0b00111100u8, []);
    operation_test!(
        or_non_empty,
        Operator::Or,
        0b00111100u8,
        [0b10000000, 0b10101010],
        0b10111110,
        [0b10000000]
    );

    operation_test!(and_empty, Operator::And, 0b00111100u8, [], 0b00000000, []);
    operation_test!(
        and_non_empty,
        Operator::And,
        0b00111100u8,
        [0b10000000, 0b10101010],
        0b00101000,
        [0b10000000]
    );

    operation_test!(xor, Operator::Xor, 0b00111100u8, [], 0b00111100u8, []);
    operation_test!(
        xor_non_empty,
        Operator::Xor,
        0b00111100u8,
        [0b10000000, 0b10101010],
        0b10010110,
        [0b10000000]
    );
}
