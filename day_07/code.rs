use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("input.in");

    let mut c: i64 = 0;

    for line in lines {
        let (value, nums) = parse(line);
        if check(&value, nums) { c += value; }
    }

    println!("{}", c);
}

fn check(value: &i64, nums: Vec<i64>) -> bool {
    for c in 0..=u16::MAX {
        let bits: [u16; 16] = get_bits(c);

        let mut total: i64 = nums[0];
        for i in 1..nums.len() {
            if bits[i] == 0 {
                total += nums[i];
            } else {
                total *= nums[i];
            }
        }

        if total == *value {
            return true;
        }
    }
    return false;
}

fn parse(string: String) -> (i64, Vec<i64>) {
    let mut test_value: i64 = 0;
    let mut numbers: Vec<i64> = vec![];

    let mut c: String = "".to_string();
    for i in 0..string.len() {
        if (get_char(&string, i) != ' ' && get_char(&string, i) != ':') || i == string.len() - 1 {

            c.push_str(&get_char(&string, i).to_string());

            if i == string.len() - 1 {
                numbers.push(c.parse::<i64>().unwrap());
            }

        } else {

            if c == "" { continue; }

            if test_value == 0 {
                test_value = c.parse::<i64>().unwrap();
            } else {
                numbers.push(c.parse::<i64>().unwrap());
            }

            c = "".to_string();
        }
    }

    return (test_value, numbers);
}

fn get_bits(byte: u16) -> [u16; 16] {
    let mut bits = [0u16; 16];
    for i in 0..16 {
        let shifted_byte = byte >> i;
        let cur_bit = shifted_byte & 1;
        bits[15 - i] = cur_bit;
    }
    return bits;
}

fn get_char(str: &String, index: usize) -> char {
    return str.chars().nth(index).unwrap();
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)  
        .collect() 
}
