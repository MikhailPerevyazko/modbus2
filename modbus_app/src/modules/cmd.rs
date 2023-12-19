use clap::Parser;
use std::path::PathBuf;


pub fn get_path() {
    let args = Args::parse();
    let home_dir = std::env::var("HOME").unwrap();
    let file_path = match args.file_path {
        Some(path) => path,
        None => PathBuf::from(home_dir)
            .join(".config")
            .join("simple_modbusclient")
            .jion("config.yaml"),
    };
}

pub struct PathState(PathBuf);
impl PathState {
    fn path(&self) -> PathBuf {
        self.0.to_owned()
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    file_path: Option<PathBuf>,
}