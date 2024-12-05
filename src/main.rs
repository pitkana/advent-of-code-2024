use std::io;
use std::io::Write;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {
    println!("Advent of Code 2024");

    let implemented_days = 1..=5;

    println!("Currently implemented days:");
    for day in implemented_days.clone() { println!("{}", day); }

    let mut valid = false;
    let mut day = 0;

    while !valid {
        print!("Enter the day of puzzle: ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<i32>() {
                    Ok(n) => {
                        if implemented_days.contains(&n) {
                            day = n;
                            valid = true;
                        } else {
                            println!("Not a valid day");
                        }
                    }
                    Err(_) => { println!("Invalid number"); }
                }
            }
            Err(e) => { println!("Error reading input: {}", e); }
        }
    }

    match day {
        1 => {
            println!("Day 1");
            day1::day1();
        },
        2 => {
            println!("Day 2");
            day2::day2();
        }
        3 => {
            println!("Day 3");
            day3::day3();
        }
        4 => {
            println!("Day 4");
            day4::day4();
        }
        5 => {
            println!("Day 5");
            day5::day5();
        }
        _ => { println!("This should never be reached"); }
    }


}
