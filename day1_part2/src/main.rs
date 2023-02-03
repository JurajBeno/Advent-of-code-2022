use std::fs::File;
use std::io::{self, BufRead, BufReader };
fn main() {
    println!("{}", calculate_elfs_caring());
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

fn calculate_elfs_caring() -> i32 {
    let lines = read_lines("./input_data".to_string());
    let mut brought_calories = Vec::<i32>::new();
    let mut collect_cal = 0;
    for line in lines {
        
        if line.as_ref().unwrap().len() > 0 {
            collect_cal += line.unwrap().parse::<i32>().unwrap();
        } else {
            brought_calories.push(collect_cal);
            collect_cal = 0;
        }
    }
    brought_calories.push(collect_cal);
    brought_calories.sort();
    return brought_calories.iter().rev().take(3).sum();
}