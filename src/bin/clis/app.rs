mod cmd;
mod files;

pub fn run() {
    let args = cmd::get_args();
    files::from_file(args.file_path);
}
