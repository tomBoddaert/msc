use msc::{
    self,
    instruction::Instruction,
    load::{from_stdin, from_str},
    machine::{Machine, State},
    plane::VecPlane,
    stack::VecStack,
};
use std::{
    env, fs,
    io::{self, stdin, stdout, Write},
    num::{ParseIntError, Wrapping},
};

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // If file paths were provided, like when executing files
        // with shebangs, run the files
        for path in &args[1..] {
            let file = match fs::read_to_string(path) {
                Ok(file) => file,
                Err(err) => {
                    eprintln!("{err}");
                    return Err(());
                }
            };

            let machine = match from_str(&file, &parse_str_n) {
                Ok(machine) => machine,
                Err(err) => {
                    eprintln!("{err}");
                    return Err(());
                }
            };

            if let Err(err) = run_machine(machine, false) {
                eprintln!("{err}");
                return Err(());
            };
        }
    } else {
        // Otherwise, read from stdin, like when the file is piped
        // in
        let stdin = io::stdin();
        let machine = match from_stdin(&stdin, &parse_str_n) {
            Ok(machine) => machine,
            Err(err) => {
                eprintln!("{err}");
                return Err(());
            }
        };

        if let Err(err) = run_machine(machine, true) {
            eprintln!("{err}");
            return Err(());
        };
    }

    Ok(())
}

type N = Wrapping<i32>;
fn parse_str_n(value: &str) -> Result<N, ParseIntError> {
    Ok(Wrapping(value.parse()?))
}

fn run_machine(
    mut machine: Machine<N, VecPlane<Instruction>, VecStack<N>, VecPlane<VecStack<N>>>,
    using_stdin: bool,
) -> Result<(), String> {
    loop {
        match machine.get_state() {
            State::Stopped => break,
            State::Running => {
                if let Some(n) = machine.step() {
                    println!("{n}");
                }
            }
            State::InputWaiting => {
                print!("> ");
                if let Err(err) = stdout().flush() {
                    return Err(err.to_string());
                };
                let mut buffer = String::new();
                if let Err(err) = stdin().read_line(&mut buffer) {
                    return Err(err.to_string());
                };
                let buffer = buffer.trim_end();

                // If the buffer is empty and the program was
                // run from stdin, it is most likely that it
                // was run through a pipe and cannot run
                // interactively
                if buffer.is_empty() && using_stdin {
                    return Err(
                        "Inputs cannot be used when the program is piped into the interpreter!\nRun the program by passing the file path as an argument.".to_owned()
                    );
                }

                machine.input(match parse_str_n(buffer) {
                    Ok(value) => value,
                    Err(err) => {
                        println!("{buffer:?}");
                        println!("{err}");
                        continue;
                    }
                });
            }
        }
    }

    Ok(())
}
