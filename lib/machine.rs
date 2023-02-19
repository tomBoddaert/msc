//! The `MSCode` machine that runs `MSCode`

use crate::{
    add_velocity_to_pointer, instruction::Instruction, plane::Plane, stack::Stack, Number, Pointer,
    Velocity,
};

/// The machine state
#[derive(Clone, Copy, Default, Debug)]
pub enum State {
    #[default]
    Running,
    Stopped,
    InputWaiting,
}

/// The machine runs `MSCode` programs
pub struct Machine<N, InstructionPlane, StackType, StackPlane>
where
    N: Default,
    InstructionPlane: Plane<Item = Instruction>,
    StackType: Stack<Item = N>,
    StackPlane: Plane<Item = StackType>,
{
    state: State,
    instructions: InstructionPlane,
    stacks: StackPlane,
    register: N,
    pointer: Pointer,
    velocity: Velocity,
}

impl<N, InstructionPlane, StackType, StackPlane> Machine<N, InstructionPlane, StackType, StackPlane>
where
    N: Number,
    InstructionPlane: Plane<Item = Instruction>,
    StackType: Stack<Item = N>,
    StackPlane: Plane<Item = StackType>,
{
    /// Create a new machine from instructions and stacks
    #[must_use]
    pub fn new(instructions: InstructionPlane, stacks: StackPlane) -> Self {
        Self {
            state: State::default(),
            instructions,
            stacks,
            register: N::ZERO,
            pointer: Pointer::default(),
            velocity: Velocity::default(),
        }
    }

    /// Run an iteration on the machine
    pub fn step(&mut self) -> Option<&N> {
        if !matches!(self.state, State::Running) {
            return None;
        }

        let Some(instruction) = self.instructions.get(self.pointer) else {
            self.state = State::Stopped;
            return None;
        };

        let output = {
            use Instruction::{Comparator, Deflector, Operator, Space, IO};
            match instruction {
                Space => None,
                Deflector(deflector) => {
                    self.velocity = deflector.apply(self.velocity);
                    None
                }
                Operator(operation) => {
                    let stack = self
                        .stacks
                        .get_mut((self.pointer.0 / 4, self.pointer.1 / 4))
                        .expect("Stack pointer out of range!");

                    self.register = operation.apply(self.register, stack);
                    None
                }
                Comparator(comparator) => {
                    let stack = self
                        .stacks
                        .get_mut((self.pointer.0 / 4, self.pointer.1 / 4))
                        .expect("Stack pointer out of range!");

                    self.velocity = comparator.apply(&self.register, stack, self.velocity);
                    None
                }
                IO(io) => {
                    let (output, io_wait) = io.apply(&self.register);
                    if io_wait {
                        self.state = State::InputWaiting;
                    }
                    output
                }
            }
        };

        self.pointer = add_velocity_to_pointer(self.velocity, self.pointer);
        output
    }

    /// Provide input to the machine when in the `InputWaiting` state
    pub fn input(&mut self, input: N) {
        if matches!(self.state, State::InputWaiting) {
            self.register = input;
            self.state = State::Running;
        }
    }

    pub const fn get_state(&self) -> State {
        self.state
    }

    pub const fn get_pointer(&self) -> Pointer {
        self.pointer
    }

    pub const fn get_register(&self) -> N {
        self.register
    }
}
