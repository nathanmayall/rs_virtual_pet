mod pet;

use crate::pet::Pet;

use std::io::{stdin, stdout, Write};
use std::process::exit;
use std::{thread, time};

use figlet_rs::FIGfont;
use terminal_size::{terminal_size, Height, Width};
use colored::*;

fn main() {
    clear_screen();

    println!("Welcome to Virtual Pet Simulator. 👋🏼");

    let mut player_name = String::new();

    while player_name.len() < 1 {
        player_name = question_prompt("What's your name?".to_string());
    }
    println!("Hello {}!", player_name);

    let mut pet_name = String::new();
    while pet_name.len() < 1 {
        pet_name = question_prompt("What's the name of your pet?".to_string());
    }

    let mut main_pet = Pet {
        name: pet_name,
        age: 0,
        hunger: 0,
        fitness: 10,
        children: Vec::new(),
    };

    let choices = [
        "Feed".to_owned(),
        "Walk".to_owned(),
        "Check Up".to_owned(),
        "Grow Up".to_owned(),
        "Exit".to_owned(),
    ];

    println!("Aww. {} is a nice name.", main_pet.name);
    println!("Let's get started.");
    thread::sleep(time::Duration::from_millis(1500));

    title_sequence("Virtuapet", None);
    thread::sleep(time::Duration::from_millis(1500));
    clear_screen();

    let mut status = format!("Pet {} created.", main_pet.name);

    while main_pet.is_alive() {
        thread::sleep(time::Duration::from_millis(500));
        clear_screen();
        println!("{}", status);
        choices.iter().for_each(|choice| println!("{}", choice));
        let response = question_prompt(format!("{} says what's your action?", main_pet.name));
        match response.to_lowercase().trim().chars().nth(0).unwrap() {
            'f' => {
                main_pet.feed();
                status = format!("Feeding {}", main_pet.name);
            }
            'w' => {
                main_pet.walk();
                status = format!("Walking {}.", main_pet.name);
            }
            'c' => {
                status = format!(
                    "Checking up on {}. Status is {}",
                    main_pet.name,
                    main_pet.status()
                )
            }
            'g' => {
                main_pet.grow_up();
                status = format!(
                    "{} is growing up! Age is now {}",
                    main_pet.name, main_pet.age
                )
            }
            _ => {
                println!("Goodbye! 👋🏼");
                exit(0);
            }
        }
    }
    title_sequence("RIP", Some(true));
    println!(
        "{} died! Sorry {}. Stats are {}",
        main_pet.name,
        player_name,
        main_pet.status()
    )
}

pub fn question_prompt(question: String) -> String {
    let mut input_string = String::new();
    print!("{} ", question);
    stdout().flush().unwrap();
    let result = stdin().read_line(&mut input_string);

    match result {
        Ok(_) => String::from(input_string.trim()),
        Err(error) => panic!("Something went really wrong. {:?}", error),
    }
}

pub fn title_sequence(title: &str, red: Option<bool>) {
    let size = terminal_size();

    match size {
        Some((Width(w), Height(h))) => {
            clear_screen();

            let epic_font = FIGfont::from_file("resources/epic.flf").unwrap();
            let figure = epic_font.convert(title);

            let text = format!("{}", figure.unwrap());

            match red {
                Some(_val) => {
                    println!("{}", text.red())
                },
                None => println!("{}", text.blue().on_bright_black())
            }
            
        }
        None => panic!("Unable to get terminal size"),
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
