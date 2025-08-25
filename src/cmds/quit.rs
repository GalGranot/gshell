use crate::exe::{CmdArgs, ExeResult};

pub fn exe_quit(_args: &mut CmdArgs) -> ExeResult {
    ExeResult::Quit
}
