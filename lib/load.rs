//! Load `MSCode` with Vec based structures.
//! Requires std

use std::{
    fmt::Display,
    io::{BufRead, Stdin},
    iter::once,
    num::{ParseIntError, TryFromIntError},
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
pub enum Error {
    InvalidInstruction(IntoInstructionError),
    InvalidInteger(ParseIntError),
    InvalidStackPointer(TryFromIntError),
    StackPointerOutOfRange(Pointer),
    MissingStackPointer(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use Error::{
            InvalidInstruction, InvalidInteger, InvalidStackPointer, MissingStackPointer,
            StackPointerOutOfRange,
        };
        match self {
            InvalidInstruction(err) => err.fmt(f),
            InvalidInteger(err) => err.fmt(f),
            InvalidStackPointer(err) => err.fmt(f),
            StackPointerOutOfRange(pointer) => {
                write!(f, "stack pointer out of range: {pointer:?}")
            }
            MissingStackPointer(line) => write!(f, "stack line missing pointer: \"{line:?}\""),
        }
    }
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

/// The returned machine type when loaded
pub type Machine<N> = machine::Machine<N, VecPlane<Instruction>, VecStack<N>, VecPlane<VecStack<N>>>;

/// Load `MSCode` from a str
pub fn from_str<N: Number>(
    source: &str,
    try_parse_n: &dyn Fn(&str) -> Result<N, ParseIntError>,
) -> Result<Machine<N>, Error> {
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
pub fn from_stdin<N: Number>(
    source: &Stdin,
    try_parse_n: &dyn Fn(&str) -> Result<N, ParseIntError>,
) -> Result<Machine<N>, Error> {
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

pub fn parse_line<N: Number>(
    line: &str,
    instructions: &mut Vec<Vec<Instruction>>,
    stack_instructions: &mut Vec<(usize, usize, Vec<N>)>,
    try_parse_n: &dyn Fn(&str) -> Result<N, ParseIntError>,
) -> Result<(), Error> {
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
            let stack: Result<Vec<N>, ParseIntError> = numbers_string
                .map(|number_str| try_parse_n(number_str))
                .collect();
            stack_instructions.push((x, y, stack?));
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

pub fn create_stacks<N: Number>(
    stack_instructions: Vec<(usize, usize, Vec<N>)>,
    instructions: &VecPlane<Instruction>,
) -> Result<VecPlane<VecStack<N>>, Error> {
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
