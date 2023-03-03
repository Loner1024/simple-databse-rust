use std::io::{stdin, stdout, Write};
use std::process::{exit, ExitCode};

fn main() -> ExitCode {
    loop {
        print_prompt();
        let mut buf = String::new();
        match stdin().read_line(&mut buf) {
            Ok(_) => {}
            Err(err) => panic!("failed to read line: {}", err)
        }
        buf = buf.trim().to_string();
        if buf.starts_with('.') {
            match do_meta_command(&buf) {
                Ok(_) => continue,
                Err(_) => {
                    println!("Unrecognized command '{}'", buf);
                    continue;
                }
            }
        }
        match prepare_statement(&buf) {
            Ok(statement) => {
                execute_statement(&statement);
                println!("Executed.");
                continue;
            }
            Err(_) => {
                println!("Unrecognized keyword at start of '{}'", buf);
                continue;
            }
        }
    }
}

fn print_prompt() {
    print!("db > ");
    stdout().flush().expect("failed out to std");
}

struct MetaCommandErr;


fn do_meta_command(command: &String) -> Result<(), MetaCommandErr> {
    if command.eq(".exit") {
        println!("Bye!");
        exit(0);
    }
    Err(MetaCommandErr)
}

struct PrepareErr;

enum StatementType {
    Insert,
    Select,
}

struct Statement {
    statement_type: StatementType,
}

fn prepare_statement(command: &String) -> Result<Statement, PrepareErr> {
    match command {
        c if c.starts_with("insert") => {
            Ok(Statement { statement_type: StatementType::Insert })
        }
        c if c.starts_with("select") => {
            Ok(Statement { statement_type: StatementType::Select })
        }
        _ => Err(PrepareErr)
    }
}

fn execute_statement(statement: &Statement) {
    match statement.statement_type {
        StatementType::Insert => println!("This is where we would do an insert."),
        StatementType::Select => println!("This is where we would do an select.")
    }
}


