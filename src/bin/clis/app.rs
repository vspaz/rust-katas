mod cmd;
mod files;
mod search;
pub fn run() {
    let args = cmd::get_args();
    let contents = files::from_file(args.file_path);
    search::search(&args.query, &contents);
}
