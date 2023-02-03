use std::fs::File;
use std::io::{self, BufRead, BufReader };
fn main() {
    println!("{}", calculate_compartment_priority());
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

fn calculate_compartment_priority() -> i32 {
    let lines = read_lines("./input_data".to_string());
    let mut result = 0;
    for line in lines {
        let (comp1, comp2) = line.as_ref().unwrap().split_at(line.as_ref().unwrap().len() / 2);
        result += find_two_with_value(comp1.to_owned(), comp2);
    }
    return result;
}

fn find_two_with_value(comp1: String, comp2: &str) -> i32 {
    for item in comp1.chars() {
        if comp2.contains(item) {
            if (item as u32) < 91 {
                return (item as i32) - 38;
            } else {
                return (item as i32) - 96;
            }
        }
    }
    println!("iguess i have err");
    return -1;
}