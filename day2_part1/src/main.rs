use std::fs::File;
use std::io::{self, BufRead, BufReader };
fn main() {
    println!("{}", calculate_score());
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

fn calculate_score() -> i32 {
    let lines = read_lines("./input_data".to_string());
    let mut result = 0;
    for line in lines {
        result += get_score(&line.unwrap());
    }
    return result;
}

fn get_score(line_unwr: &str) -> i32 {
    if line_unwr.chars().nth(0).unwrap() == line_unwr.chars().nth(2).unwrap() {
        return 3;
    } else if line_unwr.chars().nth(0).unwrap() == 'A' {
        if line_unwr.chars().nth(2).unwrap() == 'Y' {
            return 8;
        } else if line_unwr.chars().nth(2).unwrap() == 'X' {
            return 4;
        } else {
            return 3;
        }
    } else if line_unwr.chars().nth(0).unwrap() == 'B' {
        if line_unwr.chars().nth(2).unwrap() == 'Y' {
            return 5;
        } else if line_unwr.chars().nth(2).unwrap() == 'X' {
            return 1;
        } else {
            return 9;
        }
    } else if line_unwr.chars().nth(0).unwrap() == 'C' {
        if line_unwr.chars().nth(2).unwrap() == 'Y' {
            return 2;
        } else if line_unwr.chars().nth(2).unwrap() == 'X' {
            return 7;
        } else {
            return 6;
        }
    }
    return -1;
}