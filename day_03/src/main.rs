use std::fs::File;
use std::io::{prelude::*, BufReader};
use regex::Regex;

//const reg = Regex::new(r"mul\(\d*,\d*\)").unwrap();

fn main() {
    let file = File::open("./resources/input.in").unwrap();
    let mut reader = BufReader::new(file);

    let mut input = String::new(); 
    reader.read_to_string(&mut input).unwrap();
   
    let reg_inst = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    // let reg_mul = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let reg_nums = Regex::new(r"(?<a>\d+),(?<b>\d+)").unwrap();
    let instructions: Vec<&str> = reg_inst.find_iter(&input).map(|m| m.as_str()).collect();

    let mut vdo: bool = true;
    let mut sum: i32 = 0;
    for instruction in instructions {
        match instruction {
            _ if instruction == "do()" => vdo = true,
            _ if instruction == "don't()" => vdo = false,
            _ => {
                let Some(caps) = reg_nums.captures(&instruction) else { println!("No match"); return; };
                if vdo {
                    sum += &caps["a"].parse::<i32>().unwrap() * &caps["b"].parse::<i32>().unwrap();
                }
            }
        }
    }

    println!("{}", sum);
}

