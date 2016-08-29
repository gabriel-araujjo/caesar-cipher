use std::boxed::Box;
use std::env::Args as EnvArgs;
use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::io::stdin;
use std::io::stdout;

use libc::isatty;
use libc::STDIN_FILENO;

use super::error::Error;

enum Reading {
    Key,
    Output,
    Input
}

pub struct Args<'a> {
    pub command: Option<String>,
    pub input: Box<Read + 'a>,
    pub output: Box<Write + 'a>,
    pub key: u8,
    pub force: bool,
}

impl<'a> Args<'a> {
    pub fn from_env_args(env_args: EnvArgs) -> Result<Args<'a>, Error> {
        let mut args = Args {
            command: None,
            input: Box::new(stdin()),
            output: Box::new(stdout()),
            key: 3,
            force: false,
        };

        let mut reading: Option<Reading> = None;
        let mut input_from_stdin = true;

        for arg in env_args {
            match reading {
                Some(stuff) => {
                    match stuff {
                        Reading::Key => try!(args.parse_key(&arg)),
                        Reading::Input => {
                            try!(args.parse_input(&arg));
                            input_from_stdin = false;
                        },
                        Reading::Output => try!(args.parse_output(&arg)),
                    }
                    reading = None;
                },
                None => {
                    match arg.as_str() {
                        "--key" | "-k" => reading = Some(Reading::Key),
                        "--output" | "-o" => reading = Some(Reading::Output),
                        "--input" | "-i" => reading = Some(Reading::Input),
                        "--force" | "-f" => args.force = true,
                        command @ "decrypt" |
                        command @ "encrypt" => args.command = Some(String::from(command)),
                        _ => {},
                    }
                }
            }
        }

        if args.command.is_some() && input_from_stdin {
            try!(assert_stdin_is_piped());
        }

        Ok(args)
    }

    fn parse_key(&mut self, arg: &String) -> Result<(), Error> {
        match arg.parse::<u8>() {
            Ok(x @ 1...26) => {
                self.key = x
            },
            _ => {
                return Err(Error {message: String::from("Invalid key! key must be in [1,25]")})
            }
        }
        Ok(())
    }

    fn parse_input(&mut self, arg: &String) -> Result<(), Error> {
        let file = try!(File::open(&arg));

        self.input = Box::new(file);
        Ok(())
    }

    fn parse_output(&mut self, arg: &String) -> Result<(), Error> {
        let file = try!(File::create(&arg));

        self.output = Box::new(file);
        Ok(())
    }

}

fn assert_stdin_is_piped() -> Result<(), Error> {
    unsafe {
        if isatty(STDIN_FILENO) == 0 {
            Ok(())
        } else {
            Err(Error {message: String::from("No input set, use `caesar --help` for more information")})
        }
    }
}