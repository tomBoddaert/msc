//! Load `MSCode` with Vec based structures.
//! Requires std

use std::{
    error,
    fmt::Display,
    io::{BufRead, Stdin},
    iter::once,
    num::ParseIntError,
};

use crate::{
    instruction::{Instruction, IntoInstructionError},
    machine,
    plane::{Plane, VecPlane},
    stack::VecStack,
    Number, Pointer,
};

/// `MSCode` load errors
#[derive(Debug)]
pub enum Error<ParseNError: error::Error> {
    /// Invalid instruction character
    InvalidInstruction(IntoInstructionError),
    /// Invalid number
    InvalidNumber(ParseNError),
    /// Invalid coordinate number
    InvalidCoordinate(ParseIntError),
    /// Stack coordinate greater than or equal to 1/4 of the width / height
    StackPointerOutOfRange(Pointer),
    /// Missing at least one coordinate in a stack line
    MissingStackPointer(String),
}

impl<E: error::Error + 'static> error::Error for Error<E> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        use Error::{InvalidCoordinate, InvalidInstruction, InvalidNumber};
        match self {
            InvalidInstruction(err) => Some(err),
            InvalidNumber(err) => Some(err),
            InvalidCoordinate(err) => Some(err),
            _ => None,
        }
    }
}

impl<E: error::Error> Display for Error<E> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Error::{
            InvalidCoordinate, InvalidInstruction, InvalidNumber, MissingStackPointer,
            StackPointerOutOfRange,
        };
        match self {
            InvalidInstruction(err) => err.fmt(f),
            InvalidNumber(err) => Display::fmt(&err, f),
            InvalidCoordinate(err) => err.fmt(f),
            StackPointerOutOfRange(pointer) => {
                write!(f, "stack pointer out of range: {pointer:?}")
            }
            MissingStackPointer(line) => write!(f, "stack line missing pointer: \"{line:?}\""),
        }
    }
}

impl<E: error::Error> From<IntoInstructionError> for Error<E> {
    fn from(value: IntoInstructionError) -> Self {
        Self::InvalidInstruction(value)
    }
}

impl<E: error::Error> From<ParseIntError> for Error<E> {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidCoordinate(value)
    }
}

/// The returned machine type when loaded
pub type Machine<N> =
    machine::Machine<N, VecPlane<Instruction>, VecStack<N>, VecPlane<VecStack<N>>>;

/// Load `MSCode` from a str
///
/// # Errors
/// - [`Error::InvalidInstruction`] - failed to parse a character as an instruction
/// - [`Error::InvalidNumber`] - failed to parse a number
/// - [`Error::InvalidCoordinate`] - failed to parse a coordinate number
/// - [`Error::StackPointerOutOfRange`] - a stack coordinate is greater than or equal to 1/4 of the width / height
/// - [`Error::MissingStackPointer`] - missing at least one coordinate in a stack line
pub fn from_str<N: Number, ParseNError: error::Error>(
    source: &str,
    try_parse_n: &dyn Fn(&str) -> Result<N, ParseNError>,
) -> Result<Machine<N>, Error<ParseNError>> {
    let mut instructions = Vec::new();
    let mut stack_instructions = Vec::new();

    for line in source.lines() {
        parse_line(
            line,
            &mut instructions,
            &mut stack_instructions,
            try_parse_n,
        )?;
    }

    let instructions: VecPlane<Instruction> = instructions.into();
    let stacks = create_stacks(stack_instructions, &instructions)?;

    Ok(Machine::new(instructions, stacks))
}

/// Load `MSCode` from stdin
///
/// # Errors
/// - [`Error::InvalidInstruction`] - failed to parse a character as an instruction
/// - [`Error::InvalidNumber`] - failed to parse a number
/// - [`Error::InvalidCoordinate`] - failed to parse a coordinate number
/// - [`Error::StackPointerOutOfRange`] - a stack coordinate is greater than or equal to 1/4 of the width / height
/// - [`Error::MissingStackPointer`] - missing at least one coordinate in a stack line
pub fn from_stdin<N: Number, ParseNError: error::Error>(
    source: &Stdin,
    try_parse_n: &dyn Fn(&str) -> Result<N, ParseNError>,
) -> Result<Machine<N>, Error<ParseNError>> {
    let mut instructions = Vec::new();
    let mut stack_instructions = Vec::new();

    let mut lines = source.lock().lines();
    while let Some(Ok(line)) = lines.next() {
        parse_line(
            &line,
            &mut instructions,
            &mut stack_instructions,
            try_parse_n,
        )?;
    }

    let instructions: VecPlane<Instruction> = instructions.into();
    let stacks = create_stacks(stack_instructions, &instructions)?;

    Ok(Machine::new(instructions, stacks))
}

/// Load one line of `MSCode` from a str
///
/// # Errors
/// - [`Error::InvalidInstruction`] - failed to parse a character as an instruction
/// - [`Error::InvalidNumber`] - failed to parse a number
/// - [`Error::InvalidCoordinate`] - failed to parse a coordinate number
/// - [`Error::MissingStackPointer`] - missing at least one coordinate in a stack line
pub fn parse_line<N: Number, ParseNError: error::Error>(
    line: &str,
    instructions: &mut Vec<Vec<Instruction>>,
    stack_instructions: &mut Vec<(usize, usize, Vec<N>)>,
    try_parse_n: &dyn Fn(&str) -> Result<N, ParseNError>,
) -> Result<(), Error<ParseNError>> {
    let mut chars = line.chars();
    match chars.next() {
        Some('#') => {}
        Some('s') => {
            // Remove comments
            let line_string =
                chars
                    .take_while(|&char| char != '#')
                    .fold(String::new(), |mut code_line, char| {
                        code_line.push(char);
                        code_line
                    });
            let mut numbers_string = line_string.split_whitespace();

            // Pop x and y off from the numbers
            let (Some(x), Some(y)) = (numbers_string.next(), numbers_string.next()) else {
                    return Err(Error::MissingStackPointer(line.to_owned()));
                };

            let (x, y) = (x.parse()?, y.parse()?);

            // Collect the rest of the numbers into a stack
            let stack: Result<Vec<N>, ParseNError> = numbers_string
                .map(|number_str| try_parse_n(number_str))
                .collect();
            stack_instructions.push((
                x,
                y,
                match stack {
                    Ok(value) => value,
                    Err(err) => return Err(Error::InvalidNumber(err)),
                },
            ));
        }
        Some(char) => {
            let code_line: Result<Vec<Instruction>, IntoInstructionError> = once(char)
                .chain(chars)
                // Remove comments
                .map_while(|char| {
                    if char == '#' {
                        None
                    } else {
                        Some(Instruction::try_from(char))
                    }
                })
                .collect();

            instructions.push(code_line?);
        }
        None => {
            instructions.push(Vec::new());
        }
    };
    Ok(())
}

/// Create stacks from `stack_instructions`
///
/// # Errors
/// - [`Error::StackPointerOutOfRange`] - a stack coordinate is greater than or equal to 1/4 of the width / height
pub fn create_stacks<N: Number, ParseNError: error::Error>(
    stack_instructions: Vec<(usize, usize, Vec<N>)>,
    instructions: &VecPlane<Instruction>,
) -> Result<VecPlane<VecStack<N>>, Error<ParseNError>> {
    // Create blank stacks
    let mut stacks: VecPlane<VecStack<N>> =
        vec![
            vec![VecStack::new(); (instructions.width() + 3) / 4];
            (instructions.height() + 3) / 4
        ]
        .into();

    for (x, y, new_stack) in stack_instructions {
        // Attempt to get a reference to the stack
        let stack = match stacks.get_mut((x, y)) {
            Some(stack) => stack,
            None => return Err(Error::StackPointerOutOfRange((x, y))),
        };

        // Concatenate the stacks
        stack.extend(new_stack);
    }

    Ok(stacks)
}
