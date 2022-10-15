#![windows_subsystem = "console"]

use virtual_pet;
use std::process::exit;
use std::{thread, time};

use colored::*;

fn main() {
    virtual_pet::clear_screen();
    virtual_pet::title_sequence(" Virtuapet ", None);
    thread::sleep(time::Duration::from_millis(1500));
    println!("Welcome to Virtual Pet Simulator.\u{1F44B}");

    let mut player_name = String::new();

    while player_name.len() < 1 {
        player_name = virtual_pet::question_prompt("What's your name?".to_string());
    }
    println!("Hello {}!", player_name);

    let mut pet_name = String::new();
    while pet_name.len() < 1 {
        pet_name = virtual_pet::question_prompt("What's the name of your pet?".to_string());
    }

    let mut main_pet = virtual_pet::Pet::new(pet_name);

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
    virtual_pet::clear_screen();

    let mut status = format!("Pet {} created.", main_pet.name);

    while main_pet.is_alive() {
        thread::sleep(time::Duration::from_millis(500));
        virtual_pet::clear_screen();
        println!("{}", status);
        virtual_pet::print_choices(&choices);
        let response = virtual_pet::question_prompt(format!("{} says what's your action?", main_pet.name));
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
                println!("Goodbye!\u{1F44B}");
                exit(0);
            }
        }
    }
    virtual_pet::title_sequence(" RIP", Some(true));
    println!(
        "\u{1FAA6}  {} died! Sorry {} \u{1FAA6}",
        main_pet.name, player_name,
    );
    println!("{} {}", "Stats are".red(), main_pet.status().red())
}
