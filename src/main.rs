use std::io;
use std::io::Write;
use std::process::Command;

fn main() {
    println!("Welcome to sksh");
    print!("$ ");
    loop {
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

fn execute(c: &str, args: &[&str]) {
    let mut trimmed_args = Vec::new();

    for i in args {
        trimmed_args.push(i.trim());
    }

    let s = &trimmed_args[..];

    Command::new(c)
        .args(s)
        .spawn()
        .expect("failed to execute process.");
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
