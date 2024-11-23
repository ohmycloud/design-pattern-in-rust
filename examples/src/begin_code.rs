use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::process;

#[derive(PartialEq)]
enum State {
    Outside,
    Inside,
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let mut file = File::open(&args[1])?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    let mut state = State::Outside;
    let mut current_block = String::new();

    for line in input.lines() {
        match state {
            State::Outside => {
                if line.trim() == "=begin code" {
                    state = State::Inside;
                }
            }
            State::Inside => {
                if line.trim() == "=end code" {
                    println!("{}", current_block.trim());
                    current_block.clear();
                    state = State::Outside;
                } else {
                    current_block.push_str(line);
                    current_block.push('\n');
                }
            }
        }
    }

    Ok(())
}