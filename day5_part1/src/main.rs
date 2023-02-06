use std::fs::File;
use std::io::{self, BufRead, BufReader };
use std::str::Chars;
const RADIX: u32 = 10;

fn main() {
    println!("{}", calculate_camp_assigments(9));
}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

#[derive(Clone)]
struct Stack {
    crates: Vec<char>
}

fn calculate_camp_assigments(mut num_stacks: i32) -> String {
    let lines = read_lines("./input_data".to_string());
    let mut stacks: Vec<Stack> = vec![];

    while num_stacks > 0 {
        let new_vec = Vec::new();
        stacks.push(Stack { crates: new_vec});
        num_stacks -=1;
    }

    for line in lines {
        let mut index = 1;
        if line.as_ref().unwrap() == "" {
            stacks = rotate_stacks(stacks.clone());
        } else if line.as_ref().unwrap().chars().nth(0).unwrap() == 'm' {
            let chars = line.as_ref().unwrap().chars();
            let nums = get_nums_from_chars(chars);
            stacks = move_crates(*nums.get(0).unwrap(),
                           (*nums.get(1).unwrap()).try_into().unwrap(),
                             (*nums.get(2).unwrap()).try_into().unwrap(),
                                 stacks.clone());
        } else {
            for cha in line.unwrap().chars() {
                if cha != '[' && cha != ']' && cha != ' ' && !cha.is_digit(RADIX) {
                    stacks.get_mut((index - 2) / 4).unwrap().crates.push(cha);
                    
                }
            index += 1;
            }
        }
    }
    build_result(stacks)
}

fn build_result(stacks: Vec<Stack>) -> String {
    let mut res = "".to_string();
        for stack in stacks {
            res.push(*stack.crates.last().unwrap());
        }
    res
}

fn get_nums_from_chars(chars: Chars) -> Vec<i32> {
    let mut res = Vec::new();
    let mut number= "".to_string();
    for charac in chars {
        if charac.is_digit(RADIX) {
            number.push(charac);
        } else if number != "".to_string() {
            res.push(number.parse::<i32>().unwrap());
            number = "".to_string();
        }
    }
    res.push(number.parse::<i32>().unwrap());
    res
}

fn move_crates(mut quantity: i32, from: usize, to: usize, mut stacks: Vec<Stack>) -> Vec<Stack> {
    while quantity > 0 {
        let crate_ = stacks.get_mut(from - 1).unwrap().crates.pop().unwrap();
        stacks.get_mut(to - 1).unwrap().crates.push(crate_);
        quantity -= 1;
    }
    stacks
}

fn rotate_stacks(mut stacks: Vec<Stack>) -> Vec<Stack> {
    let mut n_stacks = 0;
    while n_stacks < stacks.len() {
        stacks.get_mut(n_stacks).unwrap().crates.reverse();
        n_stacks += 1;
    }
    stacks
}