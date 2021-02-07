use std::io;
use rand::Rng;
use std::cmp::Ordering;

const GUESS_NUMBER_UPPER_LIMIT: i32 = 101;

fn main() {
    print_program_intro();

    let num_to_be_guessed = get_random_number();

    loop_while_user_guesses_number(num_to_be_guessed);
}

fn loop_while_user_guesses_number(target_number: i32) {

    loop {
        let guess = get_integer();
        print_guess(guess);

        match guess.cmp(&target_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

fn get_random_number() -> i32 {
    return rand::thread_rng().gen_range(1, GUESS_NUMBER_UPPER_LIMIT);
}

fn print_guess(guess: i32) {
    println!("You guessed: {}", guess);
}

fn get_integer() -> i32 {
    loop {
        prompt_for_input();
        let mut user_input = String::new();
        let result = io::stdin().read_line(&mut user_input);
        result.expect("Failed to read line");

        let guess: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(msg) => {println!("Read {}.", msg); continue;},
        };

        return guess;
    }
}

fn prompt_for_input() {
    println!("Please input your guess.");
}

fn print_program_intro() {
    println!("Guess the number!");
}