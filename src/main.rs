use std::env;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = &args[1];

    match cmd.as_ref() {
        "check" => check(),
        "run" => run(),
        _ => panic!("invalid command: {}", cmd),
    }
}

fn check() {
    println!("check");
}

fn run() {
    println!("running");
    let proc = Command::new("node")
        .arg("-v")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start echo process");
    proc.wait_with_output().unwrap();
}
