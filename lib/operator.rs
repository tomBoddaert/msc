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
