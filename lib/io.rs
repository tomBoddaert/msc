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

#[cfg(test)]
mod test {
    use super::IO;

    macro_rules! test_io {
        ( $name:ident, $io_op:path, $reg:literal, $( $reg_match:pat_param )|+ $( if $reg_guard: expr )?, $input_wait:literal ) => {
            #[test]
            fn $name() {
                let register = $reg;
                let (output, input_wait) = $io_op.apply(&register);

                assert!(matches!(output, $( $reg_match )|+ $( if $reg_guard )?));
                assert_eq!(input_wait, $input_wait);
            }
        };
    }

    test_io!(print, IO::Print, 5, Some(&new_register) if new_register == 5, false);

    test_io!(input, IO::Input, 5, None, true);
}
