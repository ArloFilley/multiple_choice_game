use std::{io,fs};

pub mod terminal;
fn main() {
    loop {
        terminal::clear_screen();
        terminal::cursor_position(0, 0);
        let mut choices = String::from("");
        read_file();
        for line in io::stdin().lines() {
            let input = line.unwrap();
            choices = read_choice(&choices, &input);
            if choices.eq("end") {
                break;
            }
        }
        if choices.eq("end") {
            break;
        }
    }
}

fn read_file() {
    let intro = fs::read_to_string("story.txt").unwrap();
    for line in intro.lines() {
        if line.to_lowercase().eq("newchoice") {
            let _pointer = line;
        } else {
            println!("{}",line);
        }
    }
}

fn read_choice(choices: &str, input: &str) -> String {
    let mut choices = String::from(choices);
    if input.len() > 0 {
        choices.push_str(&input[0..1]);
        if input[0..1].eq("q") {
            choices = String::from("end");
        }
    }
    choices
}