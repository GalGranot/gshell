use crate::exe::ExeResult;

pub fn exe_echo(args: &[&str]) -> ExeResult {
    println!("{}", args[0]);
    ExeResult::Ok(0)
}