use std::process::Command;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let status = Command::new("make")
        .current_dir("pgn-extract")
        .status()
        .expect("Failed to run make");

    if !status.success() {
        panic!("make failed");
    }

    let bin_src = "pgn-extract/pgn-extract";
    let bin_dst = out_dir.join("pgn-extract");

    fs::copy(bin_src, &bin_dst).expect("Failed to copy binary");

    println!("cargo:rerun-if-changed=c-lib/Makefile");
    println!("cargo:rerun-if-changed=c-lib/main.c");
}