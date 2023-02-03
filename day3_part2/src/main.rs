use std::fs::File;
use std::io::{self, BufRead, BufReader };
fn main() {
    println!("{}", find_value_of_priorities());
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

fn find_value_of_priorities() -> i32 {
    let lines = read_lines("./input_data".to_string());
    let mut result = 0;
    let mut index = 0;
    let mut three_str = Vec::new();
    for line in lines {
        index += 1;
        three_str.push(line.unwrap().to_owned());
        if index % 3 == 0 {
            result += find_common_char_as_val(three_str);
            three_str = Vec::new();

        }
    }
    return result;
}

fn find_common_char_as_val(lines: Vec<String>) -> i32 {
    for item in lines.get(0).unwrap().chars() {
        if lines.get(1).unwrap().contains(item) && lines.get(2).unwrap().contains(item) {
            if (item as u32) < 91 {
                return (item as i32) - 38;
            } else {
                return (item as i32) - 96;
            }
        }
    }
    println!("there is something wrong");
    return -1;
}