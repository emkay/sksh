use std::io;
use std::io::Write;
use std::process::Command;

fn main() {
    println!("Welcome to sksh");
    loop {
        let line: String = read_line();
        let parsed: Vec<&str> = split_line(&line);
        execute(&parsed);
    }
}

fn execute(commands: &Vec<&str>) {
    if commands.len() > 0 {
        let command = commands[0].trim();

        Command::new(command)
            .spawn()
            .expect("failed to execute process.");
        println!("");
    }
}

fn split_line(line: &str) -> Vec<&str> {
    let v: Vec<&str> = line.split(' ').collect();
    return v;
}

fn read_line() -> String {
    print!("$ ");
    let mut line = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line");
    return line;
}
