use std::fs::read_to_string;
use regex::Regex;

#[derive(Debug)]
struct Robot {
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

fn main() {
    let lines: Vec<String> = read_lines("input.in");

    let mut robots: Vec<Robot> = vec![];

    for i in 0..lines.len() {
        robots.push(parse_line(&lines[i]));
    }

    for i in 0_i32..100000_i32 {
        let mut new: Vec<Robot> = vec![];
        for robot in &robots {
            new.push(simulate(&robot, i));
        }
        if line(&new) { println!("{}", i); break; }
    }

    println!("{}", safety_value(robots));
}

fn line(robots: &Vec<Robot>) -> bool{
    let mut map: Vec<Vec<char>> = vec![vec!['.'; 101]; 103];

    for robot in robots {
        map[robot.y as usize][robot.x as usize] = '#';
    }

    let mut length = 0;
    for i in 0..101 {
        for j in 0..103 {
            if map[j][i] == '#' { length += 1; } else { length = 0; }
            if length > 20 { print(&robots); return true; }
        }
    }
    
    return false;
}

fn parse_line(line: &String) -> Robot {
    let reg = Regex::new(r"p=(?<a>\d+),(?<b>\d+) v=(?<c>-?\d+),(?<d>-?\d+)").unwrap();
    let caps = reg.captures(line).unwrap();

    let a: i32 = caps["a"].parse::<i32>().unwrap();
    let b: i32 = caps["b"].parse::<i32>().unwrap();
    let c: i32 = caps["c"].parse::<i32>().unwrap();
    let d: i32 = caps["d"].parse::<i32>().unwrap();

    return Robot { x: a, y: b, vx: c, vy: d };
}

fn simulate(robot: &Robot, i: i32) -> Robot {
    const x_size: i32 = 101_i32;
    const y_size: i32 = 103_i32;

    let mut x: i32 = robot.x + robot.vx * i;
    let mut y: i32 = robot.y + robot.vy * i;
    
    x = if x > 0 { x % x_size } else { ( x_size - x.abs() % x_size ) % x_size };
    y = if y > 0 { y % y_size } else { ( y_size - y.abs() % y_size ) % y_size };

    return Robot { x: x, y: y, vx: robot.vx, vy: robot.vy };
}

fn safety_value(robots: Vec<Robot>) -> u64 {
    const x_mid: i32 = 50_i32;
    const y_mid: i32 = 51_i32;

    let mut q1: u64 = 0;
    let mut q2: u64 = 0;
    let mut q3: u64 = 0;
    let mut q4: u64 = 0;

    for robot in robots {
        let (x, y) = (robot.x, robot.y);
        
        if x < x_mid && y < y_mid { q1 += 1 };
        if x < x_mid && y > y_mid { q2 += 1 };
        if x > x_mid && y < y_mid { q3 += 1 };
        if x > x_mid && y > y_mid { q4 += 1 };
    }

    return q1 * q2 * q3 * q4;
}

fn print(robots: &Vec<Robot>) {
    let mut map: Vec<Vec<char>> = vec![vec!['.'; 101]; 103];

    for robot in robots {
        map[robot.y as usize][robot.x as usize] = '#';
    }

    for i in 0..(map.len() - 1) {
        let s: String = map[i].clone().into_iter().collect();
        println!("{}", s);
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)  
        .collect() 
}
