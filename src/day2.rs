use std::{fs, io};
use std::io::Write;

pub fn day2() {
    // TODO: refactor the puzzle selector and data reading into its own function
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
    let data = match fs::read_to_string("input/day2.txt") {
        Ok(data) => {
            println!("Read data of length {}", data.len());
            data
        },
        Err(e) => panic!("Error reading input: {}", e),
    };
    
    // common code between puzzle 1 and puzzle 2 here
    let mut reports: Vec<Vec<i32>> = Vec::new();

    for line in data.split("\n") {
        let mut report: Vec<i32> = Vec::new();
        for num in line.split(" ") {
            report.push(num.parse::<i32>().unwrap());
        }
        reports.push(report);
    }

    match puzzle {
        1 => puzzle1(reports),
        2 => puzzle2(reports),
        p => println!("Invalid puzzle: {}", p),
    }
}

fn puzzle1(reports: Vec<Vec<i32>>) {
    
    let mut safe = 0;
    
    'reports: for report in reports {
        if report[0] == report[1] { continue; }
        let safe_values = if report[0] < report[1] { -3..=-1 } else { 1..=3 };
        for nums in report.iter().zip(&report[1..]) {
            let diff = nums.0 - nums.1;
            if !safe_values.contains(&diff) { continue 'reports; }
        }
        safe += 1;
    }
    
    println!("Safe: {}", safe);
}
fn puzzle2(reports: Vec<Vec<i32>>) {
    
    let mut safe = 0;
    
    fn check_report(report: &Vec<i32>, drop: i32) -> bool {
        let mut edited_report = report.clone();
        if drop != -1 {
            edited_report.remove(drop as usize);
        }

        if edited_report[0] == edited_report[1] { return false; }
        let safe_values = if edited_report[0] < edited_report[1] { -3..=-1 } else { 1..=3 };
        for nums in edited_report.iter().zip(&edited_report[1..]) {
            let diff = nums.0 - nums.1;
            if !safe_values.contains(&diff) { return false; }
        }
        true
    }

    'reports: for report in reports {
        if check_report(&report, -1) { safe += 1; }
        else {
            for i in 0..report.len() {
                if check_report(&report, i as i32) { safe +=1; continue 'reports; }
            }
        }
    }
    
    println!("Safe: {}", safe);
}