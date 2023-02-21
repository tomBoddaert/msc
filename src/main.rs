#[cfg(not(feature = "std"))]
compile_error!("This binary must be compiled with the 'std' feature!\nTo use no_std, use the library (remove '--no-default-features').");

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

macro_rules! HELP_TEXT {
    () => {
        "\
Usage: {} [options...] <files...>

  Options:
    -s, --suppress   Suppress errors and input prompts
    -S, --stdin      Force reading from stdin
    -h, --help       Display this message
    -v, --version    Print the version
    -a, --author     Information about the author
"
    };
}

const AUTHOR_TEXT: &str = "\
https://github.com/tomboddaert/msc
This program was created by:

  Tom Boddaert
    https://tomboddaert.com/
";

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

// Conditional println
macro_rules! c_println {
    ($c:expr => $($arg:tt)*) => {
        if $c {
            println!($($arg)*)
        }
    };
}
// Conditional eprintln
macro_rules! c_eprintln {
    ($c:expr => $($arg:tt)*) => {
        if $c {
            eprintln!($($arg)*)
        }
    };
}

fn main() -> Result<(), ()> {
    // Get the args and command
    let mut args = env::args();
    let cmd = args.next().map_or(String::new(), |mut cmd| {
        cmd.push(' ');
        cmd
    });

    // Split the args into files, long options, and short options
    let (options, files): (Vec<String>, Vec<String>) = args.partition(|arg| arg.starts_with('-'));
    let (long_options, short_options): (Vec<String>, Vec<String>) = options.into_iter().partition(|option| option.starts_with("--"));
    // Concatenate all the short options
    let short_options = short_options.into_iter().fold(String::new(), |mut all, options| {all += &options[1..]; all});

    // Set defaults
    let mut suppress = false;
    let mut force_stdin = false;
    let mut do_help = false;
    let mut do_version = false;
    let mut do_author = false;

    // Parse and set short options
    for option in short_options.chars() {
        match option {
            's' => {
                suppress = true
            }
            'S' => {
                force_stdin = true;
            }
            'h' => {
                do_help = true
            }
            'v' => {
                do_version = true
            }
            'a' => {
                do_author = true
            }
            _ => {
                eprintln!("Unknown argument: '-{option}'\n  Use '{cmd}--help' for help.");
                return Err(());
            }
        }
    }

    // Parse and set long options
    for option in long_options {
        match option.as_str() {
            "--suppress" => {
                suppress = true;
            }
            "--stdin" => {
                force_stdin = true;
            }
            "--help" => {
                do_help = true;
            }
            "--version" => {
                do_version = true;
            }
            "--author" => {
                do_author = true;
            }
            _ => {
                eprintln!("Unknown argument: '{option}'\n  Use '{cmd}--help' for help.");
                return Err(());
            }
        }
    }

    // Is information about the program being printed
    let meta = do_help || do_version || do_author;
    let mut space = false;

    // Print header line if meta information is being printed
    c_println!(meta => "-- MSCode Interpreter --");

    if do_help {
        print!(HELP_TEXT!(), cmd);
        space = true;
    }

    if do_version {
        c_println!(space =>);
        println!("MCS version {}", VERSION.unwrap_or("unknown"));
        space = true;
    }

    if do_author {
        c_println!(space =>);
        print!("{}", AUTHOR_TEXT);
        space = true;
    }

    if meta {
        if suppress || force_stdin || !files.is_empty() {
            if space {
                println!();
            }
            eprintln!("Other arguments provided with --help, --version, or --author! Ignoring.")
        }

        // Exit
        return Ok(());
    }

    if files.is_empty() || force_stdin {
        // Otherwise, read from stdin, like when the file is piped
        // in
        let stdin = io::stdin();
        let machine = match from_stdin(&stdin, &parse_str_n) {
            Ok(machine) => machine,
            Err(err) => {
                c_eprintln!(!suppress => "{err}");
                return Err(());
            }
        };

        if let Err(err) = run_machine(machine, true, suppress) {
            c_eprintln!(!suppress => "{err}");
            return Err(());
        };

        Ok(())
    } else {
        // If file paths were provided, like when executing files
        // with shebangs, run the files
        for path in files {
            let file = match fs::read_to_string(path) {
                Ok(file) => file,
                Err(err) => {
                    c_eprintln!(!suppress => "{err}");
                    return Err(());
                }
            };

            let machine = match from_str(&file, &parse_str_n) {
                Ok(machine) => machine,
                Err(err) => {
                    c_eprintln!(!suppress => "{err}");
                    return Err(());
                }
            };

            if let Err(err) = run_machine(machine, false, suppress) {
                c_eprintln!(!suppress => "{err}");
                return Err(());
            };
        }
        Ok(())
    }
}

type N = Wrapping<i32>;
fn parse_str_n(value: &str) -> Result<N, ParseIntError> {
    Ok(Wrapping(value.parse()?))
}

fn run_machine(
    mut machine: Machine<N, VecPlane<Instruction>, VecStack<N>, VecPlane<VecStack<N>>>,
    using_stdin: bool,
    suppress: bool,
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
                if !suppress {
                    print!("> ");
                    if let Err(err) = stdout().flush() {
                        return Err(err.to_string());
                    };
                }
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
