use std::io::Write;

use crate::{cmds::gshell_cmd_wrong_nargs, ShellState};

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

