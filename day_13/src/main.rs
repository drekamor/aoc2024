use std::fs::read_to_string;
use regex::Regex;

struct Machine {
    ax: f64,
    ay: f64,
    bx: f64,
    by: f64,
    tx: f64,
    ty: f64,
}

fn main() {
    let lines: Vec<String> = read_lines("input.in");

    let mut machines: Vec<Machine> = vec![];

    for i in (0..lines.len()).step_by(4) {
        let (ax, ay) = parse_line(&lines[i]);
        let (bx, by) = parse_line(&lines[i + 1]);
        let (tx, ty) = parse_line(&lines[i + 2]);

        machines.push(Machine { ax: ax, ay: ay, bx: bx, by: by, tx: tx, ty: ty });
    }

    let mut count: i64 = 0;
    for machine in machines {
        count += solve(machine);
    }
    println!("{}", count);
}

fn parse_line(line: &String) -> (f64,  f64) {
    let reg = Regex::new(r"X(?<s>.)(?<a>\d+), Y.(?<b>\d+)").unwrap();
    let caps = reg.captures(line).unwrap();

    let mut a: f64 = caps["a"].parse::<i64>().unwrap() as f64;
    let mut b: f64 = caps["b"].parse::<i64>().unwrap() as f64;

    if caps["s"] == "=".to_string() {
        a += 10000000000000.0;
        b += 10000000000000.0;
    }
    return (a, b);
}

fn solve(m: Machine) -> i64 {
    let a: f64 = (( m.tx * m.by - m.bx * m.ty ) / ( m.ax * m.by - m.bx * m.ay )) as f64;
    let b: f64 = (( m.ty * m.ax - m.ay * m.tx ) / ( m.ax * m.by - m.bx * m.ay )) as f64;

    if a.fract() != 0.0 || b.fract() != 0.0 { return 0 };

    println!("{}, {}", a, b);
    return (a as i64) * 3 + b as i64;
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)  
        .collect() 
}
