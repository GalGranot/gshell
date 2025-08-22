use std::io::Write;

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

enum ExeResult {
    Empty,
    Quit,
    Unknown,
    Ok(isize)
}

fn exe_cmd(input: String) -> ExeResult {
    let parsed: Vec<&str> = input.trim().split_whitespace().collect();
    if parsed.len() == 0 {
        return ExeResult::Empty;
    }
    match parsed[0] {
        "quit" => ExeResult::Quit,
        _ => ExeResult::Unknown
    }
}

struct ShellState {
    last_cmd_code: isize
}

impl ShellState {
    fn new() -> Self {
        Self {last_cmd_code: 0}
    }
}

fn main() {
    let mut state = ShellState::new();
    loop {
        print_prompt();
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(0) => continue, // Empty
            Ok(_) => {} // Continue to parsing
            Err(_e) => gshell_die("stdin"),
        }
        match exe_cmd(input) {
            ExeResult::Empty => continue,
            ExeResult::Quit => gshell_exit(),
            ExeResult::Ok(code) => state.last_cmd_code = code,
            ExeResult::Unknown => gshell_perror("Unknown command"),
        }
    }
}