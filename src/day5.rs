use std::{fs, io};
use std::cmp::Ordering;
use std::io::Write;
use regex::Regex;

pub fn day5() {
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
    let data = match fs::read_to_string("input/day5.txt") {
        Ok(data) => {
            println!("Read data of length {}", data.len());
            data
        },
        Err(e) => panic!("Error reading input: {}", e),
    };

    let re: Regex = Regex::new(r"(\d+)\|(\d+)").unwrap();
    let mut first_pages: Vec<i32> = Vec::new();
    let mut second_pages: Vec<i32> = Vec::new();

    for (_, [page1, page2]) in re.captures_iter(&data).map(|c| c.extract()) {
        first_pages.push(page1.parse::<i32>().unwrap());
        second_pages.push(page2.parse::<i32>().unwrap());
    }

    let updates = data.lines().skip_while(|line| !line.is_empty()).skip(1).map(|line| { line.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>() }).collect::<Vec<Vec<i32>>>();

    match puzzle {
        1 => puzzle1(first_pages, second_pages, updates),
        2 => puzzle2(first_pages, second_pages, updates),
        p => println!("Invalid puzzle: {}", p),
    }
}

fn puzzle1(first_pages: Vec<i32>, second_pages: Vec<i32>, updates: Vec<Vec<i32>>) {

    fn is_ordered(first_pages: &Vec<i32>, second_pages: &Vec<i32>, page1: &i32, page2: &i32) -> bool {
        let rules = first_pages.iter().zip(second_pages.iter());
        if rules.clone().filter(|&(a, b)| a == page1 && b == page2).next().is_some() { true } else { false }
    }
    
    let mut count = 0;
    for update in updates {
        if update.is_sorted_by(|a, b| is_ordered(&first_pages, &second_pages, a, b)) {
            count += update[update.len() / 2];
        }
    }

    println!("Sum of middle pages: {}", count);
}
fn puzzle2(first_pages: Vec<i32>, second_pages: Vec<i32>, updates: Vec<Vec<i32>>) {
    
    fn order_rules(first_pages: &Vec<i32>, second_pages: &Vec<i32>, page1: &i32, page2: &i32) -> Ordering {
        let rules = first_pages.iter().zip(second_pages.iter());
        if rules.clone().filter(|&(a, b)| a == page1 && b == page2).next().is_some() { Ordering::Less }
        else if rules.filter(|&(a, b)| b == page1 && a == page2).next().is_some() { Ordering::Greater }
        else { Ordering::Equal }
    }
    fn is_ordered(first_pages: &Vec<i32>, second_pages: &Vec<i32>, page1: &i32, page2: &i32) -> bool {
        let rules = first_pages.iter().zip(second_pages.iter());
        if rules.clone().filter(|&(a, b)| a == page1 && b == page2).next().is_some() { true } else { false }
    }
    
    let mut count = 0;
    for mut update in updates {
        if !update.is_sorted_by(|a, b| is_ordered(&first_pages, &second_pages, a, b)) {
            update.sort_by(|a, b| order_rules(&first_pages, &second_pages, a, b));
            count += update[update.len() / 2];
        }
    }

    println!("Sum of middle pages: {}", count);
}