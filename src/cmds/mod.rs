use std::{collections::HashMap, io::Write};

use crate::{cmds::echo::exe_echo, exe::ExeResult};

pub mod cd;
pub mod quit;
pub mod pwd;
pub mod echo;

// To add new command:
// 1. insert it to cmd_map_new
// 2. implement it in a new cmds/<cmd>.rs file
// 3. add pub mod <cmd> at top of this file


pub type CmdFn = fn(&[&str]) -> ExeResult;
pub struct Cmd {
    pub(crate) handler: CmdFn,
    pub(crate) nargs: usize
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
    m.insert("pwd", Cmd::new(pwd::exe_pwd, 0));
    m.insert("quit", Cmd::new(quit::exe_quit, 0));
    m.insert("cd", Cmd::new(cd::exe_cd, 1));
    m.insert("echo", Cmd::new(exe_echo, 1));
    m
}

pub fn gshell_cmd_error(cmd: &str, msg: &str) {
    eprintln!("gshell error: {cmd}: {msg}");
    std::io::stdout().flush().unwrap(); // TODO: handle gracefully
}

pub fn gshell_cmd_wrong_nargs(cmd_name: &str, cmd: &Cmd, nargs: usize) -> ExeResult {
    eprintln!("gshell error: {}: expected {} args, received {} args", 
        cmd_name, cmd.nargs, nargs);
    ExeResult::BadArgs
}