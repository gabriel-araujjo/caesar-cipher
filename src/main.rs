extern crate libc;
extern crate unicode_normalization;

use std::env::args as env_args;
use std::io::Write;
use std::process::exit;
use std::error::Error as StdError;

mod caesar;
mod args;
mod error;

use args::Args;
use error::Error;

fn usage() {
    print!(
r#"
Caesar encrypter tool
Encrypt options:
  -k, --key <key>            - specifies a key to encrypting or decrypting process.
                                 The key must be a number between 1(including)
                                 and 25(including).
                                 Default: 3
  -o, --output <source_file> - sets the destiny output file.
                                 Default: stdout
  -i, --input <destny_file>  - sets the clear message source.
                                 Default: stdin
  -f, --force                - forces decryptation (brute force) and shows all
                                 possible results.
                                 Default: not force
Commands:
  encrypt                    - encrypts a message
  decrypt                    - decrypts a message

Usage:
  caesar encrypt [-k <key>] [-i <clear_text>] [-o <cipher>]
  caesar decrypt [-f] [-k <key>] [-i <cipher>] [-o <clear_text>]
"#
    )
}

fn exec_command<'a>(command: String, args: &mut Args) -> Result<(), Error> {

    let mut text = String::new();

    match (* args.input).read_to_string(&mut text) {
        Ok(0) => {
            return Err(Error{message: String::from("Empty input")})
        },
        Ok(_) => {},
        Err(_) => {
            return Err(Error{message: String::from("Can't read input")})
        }
    };

    match command.as_str() {
        "decrypt" => {
            if args.force {
                for key in 1..26 {
                    try!(writeln!(* args.output, "Result using key {}:", key));
                    try!(writeln!(* args.output, "{}", caesar::decrypt(&text, key)));
                }
            } else {
                try!(write!(* args.output, "{}", caesar::decrypt(&text, args.key)));
                try!((* args.output).flush());
            }
        },
        "encrypt" => {
            try!(write!(* args.output, "{}", caesar::encrypt(&text, args.key)));
            try!((* args.output).flush());
        },
        _ => {
            return Err(Error{message: String::from("Invalid command, type `caesar --help` for help")})
        }
    };

    Ok(())
}

fn main() {

    let mut args = match Args::from_env_args(env_args()) {
        Ok(args) => args,
        Err(err) => {
            match writeln!(&mut std::io::stderr(), "Error: {}", err) {
                Ok(_) => {},
                Err(_) => panic!("WTF!"),
            };
            exit(1);
        }
    };

    let command = match args.command {
        Some(ref s) => s.clone(),
        None => {
            usage();
            exit(1);
        }
    };

    match exec_command(command, &mut args) {
        Err(err) => {
            match writeln!(&mut std::io::stderr(), "Error: {}", err.description()) {
                Ok(_) => {},
                Err(_) => panic!("WTF!"),
            };
            exit(1);
        },
        _ => {},
    };
}
