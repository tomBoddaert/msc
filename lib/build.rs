//! Build `MSCode` with only core.
//! Can be used with `no_std`

use core::{
    iter::once,
    num::{ParseIntError, TryFromIntError},
};

use crate::{
    instruction::{Instruction, IntoInstructionError},
    machine,
    plane::{ArrayPlane, Plane},
    stack::{ArrayStack, Stack},
    Number, Pointer,
};

/// `MSCode` build errors
#[derive(Debug)]
pub enum Error {
    InvalidInstruction(IntoInstructionError),
    InstructionOutOfRange(Pointer, char),
    InvalidInteger(ParseIntError),
    InvalidStackPointer(TryFromIntError),
    StackPointerOutOfRange(Pointer),
}

impl From<IntoInstructionError> for Error {
    fn from(value: IntoInstructionError) -> Self {
        Self::InvalidInstruction(value)
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidInteger(value)
    }
}

impl From<TryFromIntError> for Error {
    fn from(value: TryFromIntError) -> Self {
        Self::InvalidStackPointer(value)
    }
}

/// The returned machine type when built
pub type Machine<
    N,
    const WIDTH: usize,
    const HEIGHT: usize,
    const STACK_CAPACITY: usize,
    const STACK_WIDTH: usize,
    const STACK_HEIGHT: usize,
> = machine::Machine<
    N,
    ArrayPlane<WIDTH, HEIGHT, Instruction>,
    ArrayStack<STACK_CAPACITY, N>,
    ArrayPlane<STACK_WIDTH, STACK_HEIGHT, ArrayStack<STACK_CAPACITY, N>>,
>;

/// Build `MSCode` from a str
pub fn from_str<
    N: Number,
    const WIDTH: usize,
    const HEIGHT: usize,
    const STACK_CAPACITY: usize,
    const STACK_WIDTH: usize,
    const STACK_HEIGHT: usize,
>(
    source: &str,
    try_parse_n: &dyn Fn(&str) -> Result<N, ParseIntError>,
    try_usize_from_n: &dyn Fn(N) -> Result<usize, TryFromIntError>,
) -> Result<Machine<N, WIDTH, HEIGHT, STACK_CAPACITY, STACK_WIDTH, STACK_HEIGHT>, Error> {
    let mut instructions = ArrayPlane::default();
    let mut stacks = ArrayPlane::default();

    // The code body line number
    let mut y = 0;

    for line in source.lines() {
        let mut chars = line.chars();
        // Match the first char of the line
        match chars.next() {
            Some('#') => {}
            Some('s') => {
                let mut stack: Option<&mut ArrayStack<STACK_CAPACITY, N>> = None;
                let (mut stack_x, mut stack_y) = (None, None);

                for mut number_str in chars.as_str().split_whitespace() {
                    // Check if this part contains a comment
                    let comment = if number_str.contains('#') {
                        // If there's nothing before the comment, stop here
                        number_str = number_str.split('#').next().unwrap_or_default();
                        if number_str.is_empty() {
                            break;
                        }
                        true
                    } else {
                        false
                    };

                    let number = try_parse_n(number_str)?;
                    match (&mut stack, stack_x, stack_y) {
                        // If the stack has been identified, push to it
                        (Some(stack), _, _) => stack.push(number),
                        // If the x coordinate is known, add the y coordinate
                        (None, Some(x), _) => {
                            let y = try_usize_from_n(number)?;
                            stack_y = Some(y);
                            stack = Some(match stacks.get_mut((x, y)) {
                                Some(stack) => stack,
                                None => return Err(Error::StackPointerOutOfRange((x, y))),
                            });
                        }
                        // If the x coordinate is not known, add it
                        (None, None, _) => stack_x = Some(try_usize_from_n(number)?),
                    }

                    // If there was a comment, stop here
                    if comment {
                        break;
                    }
                }
            }
            Some(char) => {
                // Parse each character as an instruction and add it to the plane
                for (x, new_instruction) in once(char)
                    .chain(chars)
                    .map(Instruction::try_from)
                    .enumerate()
                {
                    let Some(instruction) = instructions.get_mut((x, y)) else {
                        return Err(Error::InstructionOutOfRange((x + 1, y), new_instruction?.into()));
                    };

                    *instruction = new_instruction?;
                }

                y += 1;
            }
            None => {
                y += 1;
            }
        }
    }

    Ok(Machine::new(instructions, stacks))
}
