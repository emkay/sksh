use std::io;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::process::exit;

fn main() {
    println!("Welcome to sksh");
    looper();
}

fn cd(args: &[&str]) {
    let path = match args.len() {
        0 => std::env::home_dir(),
        _ => Some(Path::new(args[0]).to_path_buf()),
    };

    let dir = path.unwrap();
    match std::env::set_current_dir(&dir) {
        Err(err) => {
            println!("cd: {}", err);
        },
        _ => {}
    }
}

fn terminate(code: i32) {
    return exit(code);
}

fn execute(c: &str, args: &[&str]) {
    if c.len() == 0 {
        println!("");
        return;
    }

    let trimmed_args: Vec<&str> = args.into_iter().map(|arg| arg.trim()).collect();

    let clean_args = &trimmed_args[..];

    match c {
        "cd" => return cd(clean_args),
        "exit" => return terminate(0),
        _ => {}
    }

    let run_command = Command::new(c)
        .args(clean_args)
        .output();

    if let Ok(output) = run_command {
        match output.status.success() {
            true => { println!("{}", String::from_utf8_lossy(&output.stdout)) },
            false => { println!("{}", String::from_utf8_lossy(&output.stderr)) }
        }
    } else {
        println!("sksh: {} command not found.", c);
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
