use std::env;

#[derive(Debug)]
pub struct CmdArgs {
    query: String,
    file_path: String
}

pub fn get_args() -> CmdArgs {
    let args: Vec<String> = env::args().collect();
    return CmdArgs{
        query: args[1].to_owned(),
        file_path: args[2].to_owned(),
    }
}