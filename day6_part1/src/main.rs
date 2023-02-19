use std::fs::File;
use std::io::{self, BufRead, BufReader };
use std::collections::VecDeque;

fn main() {
    println!("{}", find_index_sum_of_buff_end());
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

// this should be o = n * log n if i am not mistaken
fn without_duplicates(mut queue: VecDeque<char>) -> bool {
    let mut num = queue.len();
    while num > 0 {
        let chr = queue.pop_front();
        if queue.contains(&chr.unwrap()) {
            return false
        }
        num -= 1;
    }
    true
}
fn find_index_sum_of_buff_end() -> i32 {
    let lines = read_lines("./input_data".to_string());
    let mut res = 0;
    let mut deque_chars: VecDeque<char> = VecDeque::with_capacity(4);
    
    for line in lines {
        for chr in line.unwrap().chars() {
            if deque_chars.len() < 4 {
                deque_chars.push_back(chr);
                res += 1;
            } else {
                deque_chars.pop_front();
                deque_chars.push_back(chr);
                res += 1;
                if without_duplicates(deque_chars.clone()) {
                    return res;
                }
            }
        }
    }
    res
}