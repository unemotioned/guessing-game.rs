use rand::prelude::*;
use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::process::Command;

fn get_number_input(msg: &str) -> u32 {
    loop {
        print!("{msg}");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Not a number"),
        };
    }
}

fn clear_terminal() {
    // check one runtime windows | macos | linux
    // let current_os = std::env::consts::OS;
    // println!("Current OS: {current_os}");

    // cfg!() is a compile-time boolean; dead branches are eliminated
    //  checks against the target triple (os, arch, family, env)
    if cfg!(target_os = "windows") {
        // .status() lets output reach the terminal; .output() would capture it
        Command::new("clr")
            .status()
            .expect("failed to execute process");
    } else {
        Command::new("clear")
            .arg("-x") // preserve scrollback history
            .status()
            .expect("failed to execute process");
    };
}

fn main() {
    clear_terminal();

    println!("\n==============");
    println!("Guessing game!");
    println!("==============\n");

    // type inferred from function return type
    let og_left = get_number_input("Set start of range: ");
    let og_right = get_number_input("Set end of range: ");

    let mut left_edge = og_left;
    let mut right_edge = og_right;

    println!("Range: {left_edge} ~ {right_edge}");

    let secret_number = rand::rng().random_range(left_edge..=right_edge);

    let limit = get_number_input("Set limit of attempt: ");
    println!("Attempt limit: {limit}");

    let mut cnt: u32 = 0;

    loop {
        cnt += 1;
        let msg = format!("\nPlease input your guess({cnt}/{limit}): ");
        let guess: u32 = get_number_input(&msg);
        println!("Your guess: {guess}");

        if limit == cnt {
            println!("\n==============\n");
            println!("You have reached the limit!");
            println!("The secret number was: {secret_number}");
            break;
        }

        // check if guess is within original range
        if guess < og_left || guess > og_right {
            println!("The guess is out of range.");
            continue;
        }

        // declare immutable variable each iteration
        let is_too_big = match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                false // assigned to is_too_big
            }
            Ordering::Greater => {
                println!("Too big!");
                true
            }
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };

        // narrow range
        if is_too_big && (guess <= right_edge) {
            // exclude guessed number
            right_edge = guess - 1;
        } else if guess >= left_edge {
            left_edge = guess + 1;
        }

        let next_best = (left_edge + right_edge) / 2;
        println!("Next best guess is: {next_best}");
        println!("\n--------------");
    }
}
