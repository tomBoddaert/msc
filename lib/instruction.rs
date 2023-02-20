//! `MSCode` instructions

use core::fmt::Display;

#[cfg(feature = "std")]
use std::error::Error;

use crate::{
    comparator::{self, Comparator},
    deflector::{self, Deflector},
    io::{self, IO},
    operator::{self, Operator},
};

#[derive(Clone, Copy, Default)]
pub enum Instruction {
    #[default]
    Space,
    Deflector(Deflector),
    Operator(Operator),
    Comparator(Comparator),
    IO(IO),
}

#[derive(Clone, Debug)]
pub enum IntoInstructionError {
    /// Character does not match any instructions
    UnknownChar(char),
}

#[cfg(feature = "std")]
impl Error for IntoInstructionError {}

impl Display for IntoInstructionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use IntoInstructionError::UnknownChar;
        match self {
            UnknownChar(char) => write!(f, "unknown instruction: {char}"),
        }
    }
}

impl TryFrom<char> for Instruction {
    type Error = IntoInstructionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use comparator::Comparator::{Stack, Zero};
        use deflector::Deflector::{
            BackMirror, DownArrow, ForwardMirror, LeftArrow, OmniMirror, RightArrow, UpArrow,
        };
        use io::IO::{Input, Print};
        use operator::Operator::{
            Add, And, Divide, Duplicate, Multiply, Not, Or, Pop, Push, Subtract, Xor,
        };
        use Instruction::{Comparator, Deflector, Operator, Space, IO};
        match value {
            ' ' => Ok(Space),

            '>' => Ok(Deflector(RightArrow)),
            '<' => Ok(Deflector(LeftArrow)),
            'v' => Ok(Deflector(DownArrow)),
            '^' => Ok(Deflector(UpArrow)),
            'o' => Ok(Deflector(OmniMirror)),
            '\\' => Ok(Deflector(BackMirror)),
            '/' => Ok(Deflector(ForwardMirror)),

            ',' => Ok(Operator(Push)),
            '.' => Ok(Operator(Pop)),
            'd' => Ok(Operator(Duplicate)),
            '+' => Ok(Operator(Add)),
            '-' => Ok(Operator(Subtract)),
            '*' => Ok(Operator(Multiply)),
            '~' => Ok(Operator(Divide)),
            '!' => Ok(Operator(Not)),
            '|' => Ok(Operator(Or)),
            '&' => Ok(Operator(And)),
            ':' => Ok(Operator(Xor)),

            'z' => Ok(Comparator(Zero)),
            'c' => Ok(Comparator(Stack)),

            'p' => Ok(IO(Print)),
            'i' => Ok(IO(Input)),

            _ => Err(IntoInstructionError::UnknownChar(value)),
        }
    }
}

impl From<Instruction> for char {
    fn from(val: Instruction) -> Self {
        use Instruction::{Comparator, Deflector, Operator, Space, IO};
        match val {
            Space => ' ',
            Deflector(deflector) => deflector.into(),
            Operator(operation) => operation.into(),
            Comparator(comparator) => comparator.into(),
            IO(io) => io.into(),
        }
    }
}
