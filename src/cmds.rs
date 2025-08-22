use std::{collections::HashMap, io::Write};

pub enum ExeResult {
    Empty,
    Quit,
    Unknown,
    Ok(isize)
}

fn gshell_cmd_error(cmd: &str, msg: &str) {
    eprintln!("gshell error: {cmd}: {msg}");
    std::io::stdout().flush().unwrap(); // TODO: handle gracefully
}

pub type CmdFn = fn(&[&str]) -> isize;
pub type CmdMap = HashMap<&'static str, CmdFn>;

fn cmd_map_new() -> CmdMap {
    let mut m: CmdMap = HashMap::new();
    m.insert("pwd", exe_pwd);
    m
}

fn exe_pwd(args: &[&str]) -> isize {
    if args.len() != 0 {
        
    }
    match std::env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            0
        }
        Err(_e) => { // TODO: think about how incorporate errors
            gshell_cmd_error("pwd", "getcwd failed");
            1
        }
    }
}

pub fn exe_cmd(input: String) -> ExeResult {
    let parsed: Vec<&str> = input.trim().split_whitespace().collect();
    if parsed.len() == 0 {
        return ExeResult::Empty;
    }
    match parsed[0] {
        "quit" => ExeResult::Quit,
        _ => ExeResult::Unknown
    }
}