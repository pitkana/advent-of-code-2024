use std::{fs, io};
use std::io::Write;

pub fn day4() {
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
    let data = match fs::read_to_string("input/day4.txt") {
        Ok(data) => {
            println!("Read data of length {}", data.len());
            data
        },
        Err(e) => panic!("Error reading input: {}", e),
    };
    
    // Convert data string into a 2D vector of chars
    let matrix = data.lines().map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    match puzzle {
        1 => puzzle1(matrix),
        2 => puzzle2(matrix),
        p => println!("Invalid puzzle: {}", p),
    }
}

fn puzzle1(data: Vec<Vec<char>>) {
    
    let directions: Vec<(i32, i32)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    fn find_directional(data: &Vec<Vec<char>>, x: i32, y: i32, direction: &(i32, i32), char_found: char) -> bool {
        if x < 0 || x >= data.len() as i32 || y < 0 || y >= data[0].len() as i32 {
            false
        } else {
            match (char_found, data[x as usize][y as usize]) {
                ('.', 'X') => find_directional(data, x + direction.0, y + direction.1, direction, 'X'),
                ('X', 'M') => find_directional(data, x + direction.0, y + direction.1, direction, 'M'),
                ('M', 'A') => find_directional(data, x + direction.0, y + direction.1, direction, 'A'),
                ('A', 'S') => true,
                _ => false,
            }
        }
    }
    
    let mut count = 0;
    
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == 'X' {
                let matches = directions.iter().map(|dir| find_directional(&data, i as i32, j as i32, dir, '.') ).collect::<Vec<bool>>();
                count += matches.iter().filter(|&b| *b).count();
            }
        }
    }
    
    println!("XMAS count: {}", count);
}

fn puzzle2(data: Vec<Vec<char>>) {
    
    fn check_x_mas(data: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        let (a, b) = (data[x-1][y-1], data[x+1][y+1]);
        if (a, b) == ('M', 'S') || (a, b) == ('S', 'M') {
            let (c, d) = (data[x+1][y-1], data[x-1][y+1]);
            (c, d) == ('M', 'S') || (c, d) == ('S', 'M')
        } else { false }
    }
    
    let mut count = 0;
    
    for i in 1..data.len()-1 {
        for j in 1..data[i].len()-1 {
            if data[i][j] == 'A' && check_x_mas(&data, i, j) { count += 1; }
        }
    }
    
    println!("X-MAS count: {}", count);
}