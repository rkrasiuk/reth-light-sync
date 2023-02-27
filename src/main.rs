pub mod cli;
pub mod cmd;
pub mod db;
pub mod dirs;
pub mod state_sync;
pub mod uploader;

fn main() {
    if let Err(err) = cli::run() {
        eprintln!("Error: {err:?}");
        std::process::exit(1);
    }
}
