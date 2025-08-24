use std::io::Write;

use crate::{
    cmds::{cmd_map_new, CmdMap},
    exe::{exe_cmd, ExeResult}};

mod exe;
mod cmds;

fn gshell_die(msg: &str) -> ! {
    eprintln!("gshell error: {}: exiting...", msg);
    std::io::stderr().flush().unwrap(); // TODO: handle gracefully
    std::process::exit(1);
}

fn gshell_exit()-> ! {
    println!("gshell: exiting...");
    std::io::stdout().flush().unwrap(); // TODO: handle gracefully
    std::process::exit(0);
}

fn gshell_perror(msg: &str) {
    println!("gshell error: {}", msg);
    std::io::stdout().flush().unwrap(); // TODO: handle gracefully
}

fn print_prompt() {
    print!("gshell > ");
    std::io::stdout().flush().unwrap(); // TODO: handle gracefully
}


struct ShellState {
    last_cmd_code: isize,
    cmd_map: CmdMap
}

impl ShellState {
    fn new() -> Self {
        Self {
            last_cmd_code: 0,
            cmd_map: cmd_map_new()
        }
    }
}

fn main() -> ! {
    let mut state = ShellState::new();
    loop {
        print_prompt();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => continue, // Empty
            Ok(_) => {} // Continue to parsing
            Err(_e) => gshell_die("stdin"),
        }
        match exe_cmd(input, &state) {
            ExeResult::Empty => continue,
            ExeResult::Quit => break,
            ExeResult::Ok(code) => state.last_cmd_code = code,
            ExeResult::Unknown => gshell_perror("Unknown command"),
            ExeResult::BadArgs => {},
            ExeResult::Err => {} // TODO make descriptive
        }
    }
    gshell_exit();
}