use crate::exe::{CmdArgs, ExeResult};

pub fn exe_cd(args: &mut CmdArgs) -> ExeResult {
    match std::env::set_current_dir(
        std::path::Path::new(&args.args[0])) {
            Ok(_) => ExeResult::Ok(0),
            Err(_) => {
                super::gshell_cmd_error("cd", "couldn't change directory");
                ExeResult::Err
            }
        }
}
