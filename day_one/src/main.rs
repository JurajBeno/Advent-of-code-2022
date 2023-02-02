use std::fs::File;
use std::io::{self, BufRead, BufReader };
fn main() {
    print!("{}", calculate_elfs_caring());
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

fn calculate_elfs_caring() -> i32 {
    let lines = read_lines("./input_data".to_string());
    let mut max_cal = 0;
    let mut collect_cal = 0;
    for line in lines {
        if line.as_ref().unwrap().len() > 1 {
            collect_cal += line.unwrap().parse::<i32>().unwrap();
        } else {
            if max_cal < collect_cal {
                max_cal = collect_cal;
            }
            collect_cal = 0;
        }
    }
    return max_cal;
}