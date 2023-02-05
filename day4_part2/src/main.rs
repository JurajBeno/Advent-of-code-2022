use std::fs::File;
use std::io::{self, BufRead, BufReader };
const RADIX: u32 = 10;
fn main() {
    println!("{}", calculate_camp_assigments());
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

fn calculate_camp_assigments() -> i32 {
    let lines = read_lines("./input_data".to_string());
    let mut result = 0;
    
    for line in lines {
        let mut numbers = Vec::new();
        let mut number= "".to_string();
        for charac in line.as_ref().unwrap().chars() {
            if charac.is_digit(RADIX) {
                number.push(charac);
            } else {
                numbers.push(number.parse::<i32>().unwrap());
                number = "".to_string();
            }
        }
        numbers.push(number.parse::<i32>().unwrap());

        if numbers.get(0) >= numbers.get(2) && numbers.get(0) <= numbers.get(3) {
            result += 1;
        } else if numbers.get(1) >= numbers.get(2) && numbers.get(1) <= numbers.get(3) {
            result += 1;
        } else if numbers.get(2) >= numbers.get(0) && numbers.get(2) <= numbers.get(1) {
            result += 1;
        } else if numbers.get(3) >= numbers.get(0) && numbers.get(3) <= numbers.get(1) {
            result += 1 ;
        }
    }
    result
}
