use crate::exe::{CmdArgs, ExeResult};

pub fn exe_history(args: &mut CmdArgs) -> ExeResult {
    args.state.history.print();
    ExeResult::Ok(0)
}