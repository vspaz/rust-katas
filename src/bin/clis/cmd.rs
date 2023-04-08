use std::{env, process};

#[derive(Debug)]
pub struct CmdArgs {
    query: String,
    file_path: String,
}

pub fn get_args() -> CmdArgs {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("not enough arguments");
        process::exit(-1)
    }
    return CmdArgs {
        query: args[1].to_owned(),
        file_path: args[2].to_owned(),
    };
}
