use std::io::Write;

use crate::{cmds::{gshell_cmd_wrong_nargs, CmdMap}, ShellState};


pub enum ExeResult {
    Empty,
    Quit,
    Unknown,
    Ok(isize),
    BadArgs,
    Err // TODO make errors more descriptive
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

pub struct CmdArgs<'a> {
    pub(crate) state: &'a mut ShellState,
    pub(crate) args: &'a [&'a str]
}

impl<'a> CmdArgs<'a> {
    pub fn new(state: &'a mut ShellState, args: &'a [&'a str]) -> Self {
        Self { state, args }
    }
}

pub fn exe_cmd(
    input: String,
    state: &mut ShellState,
    cmd_map: &CmdMap
) -> ExeResult {
    let parsed: Vec<&str> = input.trim().split_whitespace().collect();
    if parsed.len() == 0 {
        return ExeResult::Empty;
    }
    let mut input_backup = input.clone(); // TODO: use initial input
    update_history(input_backup, state);
    let cmd_name = &parsed[0];
    let mut args = CmdArgs::new(state, &parsed[1..]);
    match cmd_map.get(cmd_name) {
        Some(cmd) => {
            if cmd.nargs == args.args.len() {
                (cmd.handler)(&mut args)
            } else {
                gshell_cmd_wrong_nargs(&cmd_name, &cmd, args.args.len())
            }
        }
        None => try_external(&parsed)
    }
}

fn update_history(mut input: String, state: &mut ShellState) {
    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
    state.history.push(input);
}

