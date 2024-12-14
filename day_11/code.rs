use std::fs::read_to_string;
use std::collections::HashMap;

fn main() {
    let lines: Vec<String> = read_lines("input.in");

    let mut line: Vec<i64> = vec![];

    let mut temp: String = "".to_string();
    for i in 0..lines[0].len() {
        let char: &String = &lines[0][i..(i + 1)].to_string();

        if char == " " {
            line.push(temp.parse::<i64>().unwrap());
            temp = "".to_string();
        } else {
            temp.push_str(char);
        }

        if i == lines[0].len() - 1 {
            line.push(temp.parse::<i64>().unwrap());
        }
    }
    let mut cache: HashMap<i64, Vec<i64>> = HashMap::new();

    for _i in 0..3 {
        let mut temp: Vec<i64> = vec![];
        for j in 0..line.len() {
            if cache.contains_key(&line[j]) {
                temp.append(&mut cache.get(&line[j]).unwrap().clone());
            } else {
                let mut res: Vec<i64> = run(line[j]);
                cache.insert(line[j], res.clone());
                temp.append(&mut res);
            }
        }
        line = temp.clone();
    }
    
    let mut time_cache: HashMap<i64, usize> = HashMap::new();

    let mut count: u128 = 0;

    for j in 0..line.len() {
            if time_cache.contains_key(&line[j]) {
                count += *time_cache.get(&line[j]).unwrap() as u128;
            } else { 
                let run_j: Vec<i64> = if cache.contains_key(&line[j]) { cache.get(&line[j]).unwrap().to_vec() } else { run(line[j]) };
                let mut c: usize = 0;
                for k in 0..run_j.len() {
                    c += if cache.contains_key(&run_j[k]) { cache.get(&run_j[k]).unwrap().len() } else { run(run_j[k]).len() };
                }
                count += c as u128;
                time_cache.insert(line[j], c);
            }
        }

    println!("{}", count);
}

fn run(n: i64) -> Vec<i64> {
    let mut line: Vec<i64> = vec![n];
    let mut j: usize = 0;
    let mut len: usize = line.len();
    for _i in 0..15 {
        while j < len {
            if line[j] == 0 { line[j] = 1; j += 1; continue };

            let num = (line[j].abs() as f64 + 0.1).log10().ceil() as u32;
            if num % 2 == 0 {
                let temp = line[j];
                line[j] = temp / 10_i64.pow(num / 2);
                line.insert(j + 1_usize, temp % 10_i64.pow(num / 2));
                j += 1;
                len += 1;  
            } else {
                line[j] *= 2024_i64
            }
            j += 1;
        }
        j = 0;
        len = line.len();
    }
    return line.clone();
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)  
        .collect() 
}
