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

fn gshell_cmd_wrong_nargs(cmd_name: &str, cmd: &Cmd, nargs: usize) -> ExeResult {
    eprintln!("gshell error: {}: expected {} args, received {} args", 
        cmd_name, cmd.nargs, nargs);
    ExeResult::BadArgs
}

pub type CmdFn = fn(&[&str]) -> ExeResult;
pub struct Cmd {
    handler: CmdFn,
    nargs: usize
}

impl Cmd {
    pub fn new(cmd_fn: CmdFn, nargs: usize) -> Self {
        Self {
            handler: cmd_fn,
            nargs: nargs
        }
    }
}

pub type CmdMap = HashMap<&'static str, Cmd>;

pub fn cmd_map_new() -> CmdMap {
    let mut m: CmdMap = HashMap::new();
    m.insert("pwd", Cmd::new(exe_pwd, 0));
    m.insert("quit", Cmd::new(exe_quit, 0));
    m
}

fn exe_quit(args: &[&str]) -> ExeResult {
    ExeResult::Quit
}

fn exe_pwd(args: &[&str]) -> ExeResult {
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

fn exe_cd(args: &[&str]) -> ExeResult {

}

fn try_external(args: &[&str]) -> ExeResult {
    if let Ok(output) = std::process::Command::new(&args[0])
        .args(&args[1..])
        .output() {
            std::io::stdout().write_all(&output.stdout).unwrap(); // TODO handle errors
            ExeResult::Ok(0)
        } else {
            ExeResult::Unknown
        }
}

pub fn exe_cmd(input: String, state: &ShellState) -> ExeResult {
    let parsed: Vec<&str> = input.trim().split_whitespace().collect();
    if parsed.len() == 0 {
        return ExeResult::Empty;
    }
    let cmd_name = &parsed[0];
    let args = &parsed[1..];
    match state.cmd_map.get(cmd_name) {
        Some(cmd) => {
            if cmd.nargs == args.len() {
                (cmd.handler)(&args)
            } else {
                gshell_cmd_wrong_nargs(&cmd_name, &cmd, args.len())
            }
        }
        None => try_external(&parsed)
    }
}

