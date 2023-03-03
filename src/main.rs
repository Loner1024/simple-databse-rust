use std::io::{stdin, stdout, Write};
use std::process::{ExitCode};

fn print_prompt() {
    print!("db > ");
    stdout().flush().expect("failed out to std");
}

fn main() -> ExitCode {
    loop {
        print_prompt();
        let mut buf = String::new();
        match stdin().read_line(&mut buf) {
            Ok(_) => {}
            Err(err) => panic!("failed to read line: {}", err)
        }
        if buf.trim().eq(".exit") {
            return ExitCode::SUCCESS;
        }
        println!("Unrecognized command {}", buf);
    }
}
