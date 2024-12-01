use std::{fs, io};
use std::io::Write;
use std::vec::Vec;

pub fn day1() {
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
    let data = match fs::read_to_string("input/day1.txt") {
        Ok(data) => {
            println!("Read data of length {}", data.len());
            data
        },
        Err(e) => panic!("Error reading input: {}", e),
    };

    let mut list1: Vec<i32> = vec![];
    let mut list2: Vec<i32> = vec![];

    for line in data.split('\n') {
        let mut iter = line.split_ascii_whitespace();
        let first = iter.next().unwrap().parse::<i32>().unwrap();
        let second = iter.next().unwrap().parse::<i32>().unwrap();
        list1.push(first);
        list2.push(second);
    }

    match puzzle {
        1 => puzzle1(list1, list2),
        2 => puzzle2(list1, list2),
        p => println!("Invalid puzzle: {}", p),
    }
}

fn puzzle1(mut list1: Vec<i32>, mut list2: Vec<i32>) {
    list1.sort();
    list2.sort();

    let mut total_distance = 0;

    for (item1, item2) in list1.iter().zip(list2.iter()) {
        total_distance += (item1 - item2).abs();
    }

    println!("Total distance: {}", total_distance);
}

fn puzzle2(mut list1: Vec<i32>, mut list2: Vec<i32>) {
    list1.sort();
    list2.sort();

    let mut i = 0;
    let mut j = 0;

    let mut current = -1;
    let mut similarity = 0;
    let mut current_similarity = 0;

    while i < list1.len() {
        if list1[i] == current {
            similarity += current_similarity;
        } else {
            if j == list2.len() { break; }
            current_similarity = 0;
            current = list1[i];
            while j < list2.len() && list2[j] <= current {
                if list2[j] == current { current_similarity += current; }
                j += 1;
            }
            similarity += current_similarity;
            i += 1;
        }
    }

    println!("Similarity: {}", similarity);
}
