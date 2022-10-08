mod pet;
use std::io::{stdin, Write, stdout};

fn main() {
    let answer = question_prompt("What's your name? ");
    println!("Hello {}!", answer);
}

pub fn question_prompt(question: &str) -> String {

    let mut input_string = String::new();
    print!("{}", question);
    stdout().flush().unwrap();
    let result = stdin().read_line(&mut input_string);
    
    match result {
        Ok(_) => String::from(input_string.trim()),
        Err(error) => panic!("Something went really wrong. {:?}", error),
   }
}