use std::{fs, io};
use std::io::Write;
use regex::Regex;

pub fn day3() {
    let mut valid = false;
    let mut puzzle = 0;

    while !valid {
        print!("Enter the number of puzzle: ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<i32>() {
                    Ok(n) => {
                        if (1..=2).contains(&n) {
                            puzzle = n;
                            valid = true;
                        } else {
                            println!("Not a valid puzzle");
                        }
                    }
                    Err(_) => { println!("Invalid number"); }
                }
            }
            Err(e) => { println!("Error reading input: {}", e); }
        }
    }

    println!("Reading data...");
    let data = match fs::read_to_string("input/day3.txt") {
        Ok(data) => {
            println!("Read data of length {}", data.len());
            data
        },
        Err(e) => panic!("Error reading input: {}", e),
    };

    match puzzle {
        1 => puzzle1(data),
        2 => puzzle2(data),
        p => println!("Invalid puzzle: {}", p),
    }
}

fn puzzle1(data: String) {
    let mut sum = 0;
    
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    
    
    for (_, [mul1, mul2]) in re.captures_iter(&data).map(|c| c.extract()) { 
        sum += mul1.parse::<i32>().unwrap() * mul2.parse::<i32>().unwrap(); 
    }
    
    println!("Sum of multiplications: {}", sum);
}

fn puzzle2(data: String) {
    let mut sum = 0;
    let mut head = 0;

    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    while head < data.len() {
        let stop = &data[head..].find("don't()").unwrap_or(data.len() - head);
        let slice = &data[head..head + stop];

        for (_, [mul1, mul2]) in re.captures_iter(slice).map(|c| c.extract()) {
            sum += mul1.parse::<i32>().unwrap() * mul2.parse::<i32>().unwrap();
        }

        head += stop;
        let start = &data[head..].find("do()").unwrap_or(data.len() - head);
        head += start;
    }

    println!("Sum of multiplications: {}", sum);
}