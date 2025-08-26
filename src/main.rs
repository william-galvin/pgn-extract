use std::env;
use std::path::PathBuf;
use std::process::Command;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    Command::new(PathBuf::from(env!("OUT_DIR")).join("pgn-extract"))
        .args(&args)
        .status()
        .expect("Failed to run binary");
}
