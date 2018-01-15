use std::io;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::process::{Command, Stdio};
use std::process::exit;

fn main() {
    println!("Welcome to sksh");
    looper();
}

fn cd(args: &[&str]) -> Option<i32> {
    let path = match args.len() {
        0 => std::env::home_dir(),
        _ => Some(Path::new(args[0]).to_path_buf()),
    };

    let dir = path.unwrap();
    match std::env::set_current_dir(&dir) {
        Err(err) => {
            println!("cd: {}", err);
            return Some(1);
        },
        _ => return Some(0)
    }
}

fn execute(c: &str, args: &[&str]) -> Option<i32> {
    if c.len() == 0 {
        println!("");
        return Some(0);
    }

    let trimmed_args: Vec<&str> = args.into_iter().map(|arg| arg.trim()).collect();

    let clean_args = &trimmed_args[..];

    match c {
        "cd" => return cd(clean_args),
        "exit" => {
            let code = match clean_args.len() {
                0 => 0,
                1 => clean_args[0].parse::<i32>().unwrap(),
                _ => 1
            };

            return exit(code);
        },
        _ => {}
    }

    let mut process = Command::new(c)
        .args(clean_args)
        .stderr(Stdio::inherit())
        .stdout(Stdio::inherit())
        .spawn()
        .unwrap();

    let status = process.wait().expect("failed to wait.");

    return status.code();
}

fn split_line(line: &str) -> Vec<&str> {
    let v: Vec<&str> = line.split(' ').collect();
    return v;
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line");
    return line;
}

fn looper() {
    loop {
        print!("$ ");
        let line: String = read_line();
        let parsed: Vec<&str> = split_line(&line);
        let (command, args) = match parsed.split_first() {
            Some((first, elements)) => (first.trim(), elements),
            None => {
                return ();
            }
        };

        execute(command, args);
    }
}
