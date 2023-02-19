//! `MSCode` instructions for input and output

#[derive(Clone, Copy)]
pub enum IO {
    Print,
    Input,
}

impl IO {
    pub const fn apply<N>(self, register: &N) -> (Option<&N>, bool) {
        use IO::{Input, Print};
        match self {
            Print => (Some(register), false),
            Input => (None, true),
        }
    }
}

impl From<IO> for char {
    fn from(value: IO) -> Self {
        use IO::{Input, Print};
        match value {
            Print => 'p',
            Input => 'i',
        }
    }
}
