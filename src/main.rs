mod pet;
use figlet_rs::FIGfont;
use std::{thread, time};
use std::io::{stdin, stdout, Write};
use terminal_size::{terminal_size, Height, Width};

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    let mut player_name= String::new();

    while player_name.len() < 1 {
        player_name = question_prompt("What's your name?");
    }
    println!("Hello {}!", player_name);


    let mut pet_name = String::new();
    while pet_name.len() < 1 {
    pet_name = question_prompt("What's the name of your pet?");
    }
    println!("Aww. {} is a nice name.", pet_name);
    println!("Let's get started.");
    thread::sleep(time::Duration::from_millis(1500));

    title_sequence("Virtuapet");
}

pub fn question_prompt(question: &str) -> String {
    let mut input_string = String::new();
    print!("{} ", question);
    stdout().flush().unwrap();
    let result = stdin().read_line(&mut input_string);
    print!("\x1B[2J\x1B[1;1H");

    match result {
        Ok(_) => String::from(input_string.trim()),
        Err(error) => panic!("Something went really wrong. {:?}", error),
    }
}

pub fn title_sequence(title: &str) {
    let size = terminal_size();

    match size {
        Some((Width(w), Height(h))) => {
            print!("\x1B[2J\x1B[1;1H");

            let epic_font = FIGfont::from_file("resources/epic.flf").unwrap();
            let figure = epic_font.convert(title);

            let text = figure.unwrap();

            println!("{}", text);
            println!("Height is {}, Width is {},", h, w)
        }
        None => panic!("Unable to get terminal size"),
    }
}