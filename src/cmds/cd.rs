use crate::exe::ExeResult;

pub fn exe_cd(args: &[&str]) -> ExeResult {
    match std::env::set_current_dir(
        std::path::Path::new(&args[0])) {
            Ok(_) => ExeResult::Ok(0),
            Err(_) => {
                super::gshell_cmd_error("cd", "couldn't change directory");
                ExeResult::Err
            }
        }
}
