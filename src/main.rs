use std::io;
use std::env;
use std::io::Write;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("Welcome to sksh");
    looper();
}

fn cd(args: &[&str]) {
    let path = match args.len() {
        0 => env::home_dir(),
        _ => Some(Path::new(args[0]).to_path_buf()),
    };

    let dir = path.unwrap();
    match env::set_current_dir(&dir) {
        Err(err) => {
            println!("cd: {}", err);
        },
        _ => {}
    }
}

fn execute(c: &str, args: &[&str]) {
    if c.len() == 0 {
        println!("");
        return;
    }

    let mut trimmed_args = Vec::new();

    for i in args {
        trimmed_args.push(i.trim());
    }

    let clean_args = &trimmed_args[..];

    if c == "cd" {
        return cd(clean_args);
    }

    let output = Command::new(c)
        .args(clean_args)
        .output()
        .expect("failed to execute process.");

    if output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
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
