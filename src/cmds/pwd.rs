use crate::exe::ExeResult;

pub fn exe_pwd(_args: &[&str]) -> ExeResult {
    match std::env::current_dir() {
        Ok(path) => {
            println!("{}", path.display());
            ExeResult::Ok(0)
        }
        Err(_e) => { // TODO: think about how incorporate errors
            super::gshell_cmd_error("pwd", "getcwd failed");
            ExeResult::Err
        }
    }
}
