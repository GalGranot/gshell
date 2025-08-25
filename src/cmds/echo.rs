use crate::exe::{CmdArgs, ExeResult};

pub fn exe_echo(args: &mut CmdArgs) -> ExeResult {
    println!("{}", args.args[0]);
    ExeResult::Ok(0)
}