use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("input2.in");

    let mut c_p1: i64 = 0;
    let mut c_p2: i64 = 0;

    for line in lines {
        let (value, nums) = parse(line);
        if check_p1(&value, &nums) { c_p1 += value; } 
        //if check_p2(&value, &nums) { c_p2 += value; }
    }

    println!("P1: {}; P2: {}", c_p1, c_p2);
}

fn check_p1(value: &i64, nums: &Vec<i64>) -> bool {
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

fn check_p2(value: &i64, nums: &Vec<i64>) -> bool {
    for c in 0..3_u64.pow(12) {
        let ter: [u64; 12] = get_ternary(c);

        let mut total: i64 = nums[0];
        for i in 1..nums.len() {
            if ter[i] == 0 {
                total += nums[i];
            } else if ter[i] == 1 {
                total *= nums[i];
            } else {
                let n = nums[i];
                let d = (n.abs() as f64 + 0.1).log10().ceil() as u32;
                total *= 10_i64.pow(d); 
                total += nums[i];
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

fn get_ternary(mut num: u64) -> [u64; 12] {
    let mut ter = [0u64; 12];
    for i in 0..12 {
        ter[i] = num % 3;
        num /= 3;
    }

    return ter;
}

fn get_bits(byte: u16) -> [u16; 16] {
    let mut bits = [0u16; 16];
    for i in 0..16 {
        let shifted_byte = byte >> i;
        let cur_bit = shifted_byte & 1;
        bits[i] = cur_bit;
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
