use std::{collections::VecDeque, io::Write};

const INIT_HISTORY_SIZE: usize = 500;

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

pub struct History {
    capacity: usize,
    queue: VecDeque<String>,
}

impl History {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity,
            queue: VecDeque::with_capacity(capacity)
        }
    }
    pub fn push(&mut self, entry: String) {
        self.queue.push_back(entry);
        if self.queue.len() > self.capacity {
            self.queue.pop_back();
        }
    }
    fn print(&self) {
        for entry in &self.queue {
            println!("{}", entry);
        }
    }
}

struct ShellState {
    last_cmd_code: isize,
    history: History,
}

impl ShellState {
    fn new() -> Self {
        Self {
            last_cmd_code: 0,
            history: History::new(INIT_HISTORY_SIZE),
        }
    }
}

fn main() -> ! {
    let cmd_map = cmd_map_new();
    let mut state = ShellState::new();
    loop {
        print_prompt();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => continue, // Empty
            Ok(_) => {} // Continue to parsing
            Err(_e) => gshell_die("stdin"),
        }
        match exe_cmd(input, &mut state, &cmd_map) {
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