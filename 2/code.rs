use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut count: i32 = 0;

    for line in reader.lines() {
        let str = line.expect("Failed reading line");
        let levels: Vec<i32> = str.split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
        
        let mut is_safe: bool = false;

        for i in 0..levels.len() {
            let mut v: Vec<i32> = levels.clone();
            v.remove(i);

            if safe(&mut v) {is_safe = true};
        }
        if is_safe {println!("Safe"); count += 1;} else {println!("Unsafe")};
    }
    println!("{}", count);

    Ok(())
}

fn safe(nums: &mut Vec<i32>) -> bool {
    let mut prev: i32 = -1;
    let mut mode: i32 = -1;
    for int in nums.iter_mut() {
        if !safe {continue};

        if prev == -1 {
            prev = *int;
            continue;
        }

        let val = *int - prev;
        if val.abs() == 0 || val.abs() > 3 {
            return false;
        }

        if mode == -1 {
            mode = if *int > prev {1} else {0};
        }
        
        if mode != check(&*int, &prev) {
            return false;
        }

        prev = *int;
    }
    return true;
}

fn check(a: &i32, b: &i32) -> i32 {
    if a > b {return 1} else {return 0};
}
