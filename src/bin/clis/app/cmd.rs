use std::{env, process};

#[derive(Debug)]
pub struct CmdArgs {
    pub query: String,
    pub file_path: String,
}

pub fn get_args() -> CmdArgs {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("not enough arguments");
        process::exit(-1)
    }
    return CmdArgs {
        query: args[1].to_owned(),
        file_path: args[2].to_owned(),
    };
}
