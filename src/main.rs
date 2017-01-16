use std::io;
use std::io::Write;
use std::process::Command;

fn main() {
    println!("Welcome to sksh");
    looper();
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

    let s = &trimmed_args[..];


    let output = Command::new(c)
        .args(s)
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
