use colored::*;
use figlet_rs::FIGfont;
use std::io::{stdin, stdout, Write};
use std::{thread, time};
use terminal_size::{terminal_size, Height, Width};

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

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn car_cdr(s: &str) -> (&str, &str) {
    match s.chars().next() {
        Some(c) => s.split_at(c.len_utf8()),
        None => s.split_at(0),
    }
}

pub fn print_choices(choices: &[String]) {
    choices.iter().for_each(|choice| {
        let (first_char, remainder) = car_cdr(choice);
        print!("{}", format!("{}", first_char).bold());
        println!("{}", remainder);
    });
}

pub struct Pet {
    pub name: String,
    pub age: i8,
    pub hunger: i8,
    pub fitness: i8,
    pub children: Vec<Pet>,
}

impl Pet {
    pub fn new(name: String) -> Pet {
        Pet {
            name,
            age: 0,
            hunger: 0,
            fitness: 10,
            children: Vec::new(),
        }
    }
    pub fn is_alive(&self) -> bool {
        self.hunger < 10 && self.fitness > 0 && self.age < 30
    }

    pub fn adopt_child(&mut self, child: Pet) -> &Vec<Pet> {
        self.children.push(child);
        &self.children
    }
    pub fn status(&self) -> String {
        format!(
            "Age: {} Hunger: {} Fitness: {}",
            self.age, self.hunger, self.fitness
        )
    }
    pub fn feed(&mut self) {
        if self.hunger - 3 < 0 {
            self.hunger = 0;
        } else {
            self.hunger -= 3
        }
    }
    pub fn walk(&mut self) {
        if self.fitness + 3 > 10 {
            self.fitness = 10
        } else {
            self.fitness += 3
        }
    }
    pub fn grow_up(&mut self) {
        self.age += 1;
        self.hunger += 3;
        self.fitness -= 3;
    }
}

pub fn title_sequence(title: &str, colour: Option<bool>) {
    let size = terminal_size();

    match size {
        Some((Width(_w), Height(_h))) => {
            let mut acc = String::new();

            let mut font_value = include_str!("../resources/epic.flf");

            if colour.is_some() {
                font_value = include_str!("../resources/poison.flf");
            }

            let epic_font = FIGfont::from_content(font_value).unwrap();
            title.chars().into_iter().for_each(|c| {
                clear_screen();
                acc.push(c);

                let figure = epic_font.convert(acc.as_str());
                let mut text = format!("{}", figure.unwrap());

                match colour {
                    Some(_val) => text = format!("{}", text.red().on_black().bold()),
                    None => text = format!("{}", text.blue().bold()),
                }
                println!("{}", text);
                thread::sleep(time::Duration::from_millis(100));
            });
        }
        None => panic!("Unable to get terminal size"),
    }
}