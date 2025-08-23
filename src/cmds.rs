use std::{collections::HashMap, io::Write};

use crate::ShellState;

pub enum ExeResult {
    Empty,
    Quit,
    Unknown,
    Ok(isize),
    BadArgs,
    Err // TODO make errors more descriptive
}

fn gshell_cmd_error(cmd: &str, msg: &str) {
    eprintln!("gshell error: {cmd}: {msg}");
    std::io::stdout().flush().unwrap(); // TODO: handle gracefully
}

pub type CmdFn = fn(&[&str]) -> ExeResult;
pub type CmdMap = HashMap<&'static str, CmdFn>;

pub fn cmd_map_new() -> CmdMap {
    let mut m: CmdMap = HashMap::new();
    m.insert("pwd", exe_pwd);
    m.insert("quit", exe_quit);
    m
}

fn exe_quit(args: &[&str]) -> ExeResult {
    match args.len() {
        0 => ExeResult::Quit,
        _ => ExeResult::BadArgs
    }
}

fn exe_pwd(args: &[&str]) -> ExeResult {
    if args.len() != 0 {
        
    }
    match std::env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            ExeResult::Ok(0)
        }
        Err(_e) => { // TODO: think about how incorporate errors
            gshell_cmd_error("pwd", "getcwd failed");
            ExeResult::Err
        }
    }
}

pub fn exe_cmd(input: String, state: &ShellState) -> ExeResult {
    let parsed: Vec<&str> = input.trim().split_whitespace().collect();
    if parsed.len() == 0 {
        return ExeResult::Empty;
    }
    match state.cmd_map.get(parsed[0]) {
        Some(handler) => handler(&parsed[1..]),
        None => ExeResult::Unknown
    }
}