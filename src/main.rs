extern crate reqwest;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::time::Instant;

use reqwest::header::*;

mod days;

fn main() {
    loop {
        // Create a new String instance
        let mut input = String::new();
        // Read user input and assign to input variable
        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong reading the line!");

        // Try to execute the command
        match execute_command(&input) {
            Ok(_b) => continue,
            Err(e) => println!("{}", e),
        }
    }
}

fn execute_command(label: &str) -> Result<bool, Box<Error>> {
    // Split arguments and convert the iterator into a vector
    let split_iterator = label.trim().split(" ");
    let args: Vec<&str> = split_iterator.collect();

    /*
    Commands:
      solve <number of day>
    */
    if args.len() == 2 {
        if args[0] == "solve" {
            let day: u8 = args[1].parse()?;
            let function: fn(String) -> Result<(String, String), Box<Error>> = match day {
                1 => days::day1::solve,
                2 => days::day2::solve,
                _ => panic!("Not implemented yet sadly..."),
            };
            let input = get_input_for(day)?;

            let start_time = Instant::now();
            let solutions = function(input);
            let runtime = start_time.elapsed();

            match solutions {
                Ok((s1, s2)) => println!(
                    "Solutions: \n  => Part 1: {} \n  => Part 2: {} \nSolved in {} millis",
                    s1,
                    s2,
                    runtime.subsec_millis()
                ),
                Err(e) => println!("Error: {}", e),
            }

            return Ok(true);
        } else if args[0] == "retrieve" {
            // retrieve the input
        }
    }
    Err("Unknown command!".into())
}

fn get_input_for(day: u8) -> Result<String, Box<Error>> {
    println!("Reading input file for day {}", day);
    let path = format!("inputs/day{}.txt", day);
    println!("{}", path);
    let mut content = String::new();

    File::open(&path)?.read_to_string(&mut content)?;

    Ok(content)
}