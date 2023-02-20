//! While this example does use std for printing, it demonstrates a setup of a machine
//! that will run in no_std mode

use core::num::{ParseIntError, TryFromIntError, Wrapping};
use msc::{build, machine::State};

const PROGRAM_SIZE: (usize, usize) = (8, 6);
const STACK_CAPACITY: usize = 2;
const STACK_SIZE: (usize, usize) = (2, 2);

const PROGRAM: &str = "#
s 0 0 1
s 1 1 100
>+  ,v

  >   .v
  ,
  ^. < d
^,    pc
#";

fn main() {
    #[cfg(feature = "std")]
    println!("Try running this example with the '--no-default-features' flag!");

    let mut machine = build::from_str::<
        N,
        { PROGRAM_SIZE.0 },
        { PROGRAM_SIZE.1 },
        STACK_CAPACITY,
        { STACK_SIZE.0 },
        { STACK_SIZE.1 },
        ParseIntError,
        TryFromIntError
    >(PROGRAM, &try_parse_n, &try_n_to_usize)
    .unwrap();

    while matches!(machine.get_state(), State::Running) {
        if let Some(n) = machine.step() {
            println!("{n}");
        }
    }

    #[cfg(feature = "std")]
    println!("Try running this example with the '--no-default-features' flag!");
}

type N = Wrapping<i32>;
fn try_parse_n(value: &str) -> Result<N, ParseIntError> {
    Ok(Wrapping(value.parse()?))
}
fn try_n_to_usize(value: N) -> Result<usize, TryFromIntError> {
    value.0.try_into()
}
