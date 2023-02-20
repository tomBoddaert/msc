//! Build `MSCode` with only core.
//! Can be used with `no_std`

use core::{fmt::Display, iter::once};

use crate::{
    instruction::{Instruction, IntoInstructionError},
    machine,
    plane::{ArrayPlane, Plane},
    stack::{ArrayStack, Stack},
    Number, Pointer,
};

/// `MSCode` build errors
#[derive(Debug)]
pub enum Error<ParseNError: Display, NToUsizeError: Display> {
    /// Invalid instruction character
    InvalidInstruction(IntoInstructionError),
    /// Instruction out of the width and height set as constants
    InstructionOutOfRange(Pointer, char),
    /// Invalid number
    InvalidNumber(ParseNError),
    /// Invalid coordinate number
    InvalidCoordinate(NToUsizeError),
    /// Stack coordinate greater than or equal to 1/4 of the width / height
    StackPointerOutOfRange(Pointer),
    /// Missing at least one coordinate in a stack line
    MissingStackPointer,
}

/// <span style="color: var(--codeblock-error-hover-color);">
/// Only implemented when using std!
/// </span>
#[cfg(feature = "std")]
impl<PNE: std::error::Error + 'static, NUE: std::error::Error + 'static> std::error::Error
    for Error<PNE, NUE>
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Error::{InvalidCoordinate, InvalidInstruction, InvalidNumber};
        match self {
            InvalidInstruction(err) => Some(err),
            InvalidNumber(err) => Some(err),
            InvalidCoordinate(err) => Some(err),
            _ => None,
        }
    }
}

impl<PNE: Display, NUE: Display> Display for Error<PNE, NUE> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Error::{
            InstructionOutOfRange, InvalidCoordinate, InvalidInstruction, InvalidNumber,
            MissingStackPointer, StackPointerOutOfRange,
        };
        match self {
            InvalidInstruction(err) => err.fmt(f),
            InstructionOutOfRange(pointer, char) => {
                write!(f, "instruction out of range: {pointer:?} ('{char}')")
            }
            InvalidNumber(err) => err.fmt(f),
            InvalidCoordinate(err) => err.fmt(f),
            StackPointerOutOfRange(pointer) => {
                write!(f, "stack pointer out of range: {pointer:?}")
            }
            MissingStackPointer => write!(f, "stack line missing at least one coordinate"),
        }
    }
}

impl<PNE: Display, NUE: Display> From<IntoInstructionError> for Error<PNE, NUE> {
    fn from(value: IntoInstructionError) -> Self {
        Self::InvalidInstruction(value)
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
///
/// # Errors
/// - [`Error::InvalidInstruction`] - failed to parse a character as an instruction
/// - [`Error::InstructionOutOfRange`] - instruction out of width and height set as constants
/// - [`Error::InvalidNumber`] - failed to parse a number
/// - [`Error::InvalidCoordinate`] - failed to parse a coordinate number
/// - [`Error::StackPointerOutOfRange`] - a stack coordinate is greater than or equal to 1/4 of the width / height
/// - [`Error::MissingStackPointer`] - missing at least one coordinate in a stack line
pub fn from_str<
    N: Number,
    const WIDTH: usize,
    const HEIGHT: usize,
    const STACK_CAPACITY: usize,
    const STACK_WIDTH: usize,
    const STACK_HEIGHT: usize,
    ParseNError: Display,
    NToUsizeError: Display,
>(
    source: &str,
    try_parse_n: &dyn Fn(&str) -> Result<N, ParseNError>,
    try_usize_from_n: &dyn Fn(N) -> Result<usize, NToUsizeError>,
) -> Result<
    Machine<N, WIDTH, HEIGHT, STACK_CAPACITY, STACK_WIDTH, STACK_HEIGHT>,
    Error<ParseNError, NToUsizeError>,
> {
    let mut instructions = ArrayPlane::default();
    let mut stacks = ArrayPlane::default();

    // The code body line number
    let mut y = 0;

    for line in source.lines() {
        parse_line(
            line,
            &mut y,
            &mut instructions,
            &mut stacks,
            try_parse_n,
            try_usize_from_n,
        )?;
    }

    Ok(Machine::new(instructions, stacks))
}

#[cfg(feature = "std")]
/// Build `MSCode` from a stdin (<span style="color: var(--codeblock-error-hover-color);">REQUIRES STD!</span>)
///
/// # Errors
/// - [`Error::InvalidInstruction`] - failed to parse a character as an instruction
/// - [`Error::InstructionOutOfRange`] - instruction out of width and height set as constants
/// - [`Error::InvalidNumber`] - failed to parse a number
/// - [`Error::InvalidCoordinate`] - failed to parse a coordinate number
/// - [`Error::StackPointerOutOfRange`] - a stack coordinate is greater than or equal to 1/4 of the width / height
/// - [`Error::MissingStackPointer`] - missing at least one coordinate in a stack line
pub fn from_stdin<
    N: Number,
    const WIDTH: usize,
    const HEIGHT: usize,
    const STACK_CAPACITY: usize,
    const STACK_WIDTH: usize,
    const STACK_HEIGHT: usize,
    ParseNError: Display,
    NToUsizeError: Display,
>(
    source: &std::io::Stdin,
    try_parse_n: &dyn Fn(&str) -> Result<N, ParseNError>,
    try_usize_from_n: &dyn Fn(N) -> Result<usize, NToUsizeError>,
) -> Result<
    Machine<N, WIDTH, HEIGHT, STACK_CAPACITY, STACK_WIDTH, STACK_HEIGHT>,
    Error<ParseNError, NToUsizeError>,
> {
    use std::io::BufRead;

    let mut instructions = ArrayPlane::default();
    let mut stacks = ArrayPlane::default();

    // The code body line number
    let mut y = 0;

    let mut lines = source.lock().lines();
    while let Some(Ok(line)) = lines.next() {
        parse_line(
            &line,
            &mut y,
            &mut instructions,
            &mut stacks,
            try_parse_n,
            try_usize_from_n,
        )?;
    }

    Ok(Machine::new(instructions, stacks))
}

/// Parse a code line from a str
///
/// # Errors
/// - [`Error::InvalidInstruction`] - failed to parse a character as an instruction
/// - [`Error::InstructionOutOfRange`] - instruction out of width and height set as constants
/// - [`Error::InvalidNumber`] - failed to parse a number
/// - [`Error::InvalidCoordinate`] - failed to parse a coordinate number
/// - [`Error::StackPointerOutOfRange`] - a stack coordinate is greater than or equal to 1/4 of the width / height
/// - [`Error::MissingStackPointer`] - missing at least one coordinate in a stack line
pub fn parse_line<
    N: Number,
    const WIDTH: usize,
    const HEIGHT: usize,
    const STACK_CAPACITY: usize,
    const STACK_WIDTH: usize,
    const STACK_HEIGHT: usize,
    ParseNError: Display,
    NToUsizeError: Display,
>(
    line: &str,
    y: &mut usize,
    instructions: &mut ArrayPlane<WIDTH, HEIGHT, Instruction>,
    stacks: &mut ArrayPlane<STACK_WIDTH, STACK_HEIGHT, ArrayStack<STACK_CAPACITY, N>>,
    try_parse_n: &dyn Fn(&str) -> Result<N, ParseNError>,
    try_usize_from_n: &dyn Fn(N) -> Result<usize, NToUsizeError>,
) -> Result<(), Error<ParseNError, NToUsizeError>> {
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

                let number = match try_parse_n(number_str) {
                    Ok(value) => value,
                    Err(err) => return Err(Error::InvalidNumber(err)),
                };
                match (&mut stack, stack_x, stack_y) {
                    // If the stack has been identified, push to it
                    (Some(stack), _, _) => stack.push(number),
                    // If the x coordinate is known, add the y coordinate
                    (None, Some(x), _) => {
                        let y = match try_usize_from_n(number) {
                            Ok(value) => value,
                            Err(err) => return Err(Error::InvalidCoordinate(err)),
                        };
                        stack_y = Some(y);
                        stack = Some(match stacks.get_mut((x, y)) {
                            Some(stack) => stack,
                            None => return Err(Error::StackPointerOutOfRange((x, y))),
                        });
                    }
                    // If the x coordinate is not known, add it
                    (None, None, _) => {
                        stack_x = Some(match try_usize_from_n(number) {
                            Ok(value) => value,
                            Err(err) => return Err(Error::InvalidCoordinate(err)),
                        });
                    }
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
                let Some(instruction) = instructions.get_mut((x, *y)) else {
                        return Err(Error::InstructionOutOfRange((x + 1, *y), new_instruction?.into()));
                    };

                *instruction = new_instruction?;
            }

            *y += 1;
        }
        None => {
            *y += 1;
        }
    }

    Ok(())
}
