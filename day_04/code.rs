use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("input.in");

    let mut c1: i32 = 0;
    let mut c2: i32 = 0;

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            match get_char(&i, &j, &lines) {
                'X' => c1 += p1(i, j, &lines),
                'A' => c2 += p2(i, j, &lines),
                _ => (),
            }
        }
    }

    println!("P1: {}\nP2: {}", c1, c2);
}

fn p1(i: usize, j: usize, lines: &Vec<String>) -> i32 {
    let mut count: i32 = 0;

    // N, NW, NE
    if i >= 3 {
        if xmas(&i - 1, &j + 0, &i - 2, &j + 0, &i - 3, &j + 0, &lines) {
            count += 1;
        }
        if j >= 3 {
            if xmas(&i - 1, &j - 1, &i - 2, &j - 2, &i - 3, &j - 3, &lines) {
                count += 1;
            } 
        }
        if j < lines[0].len() - 3 { 
            if xmas(&i - 1, &j + 1, &i - 2, &j + 2, &i - 3, &j + 3, &lines) {
                count += 1;
            }
        }
    }

    // S, SW, SE
    if i < lines.len() - 3 {
        if xmas(&i + 1, &j + 0, &i + 2, &j + 0, &i + 3, &j + 0, &lines) {
            count += 1;
        }
        if j >= 3 {
            if xmas(&i + 1, &j - 1, &i + 2, &j - 2, &i + 3, &j - 3, &lines) {
                count += 1;
            } 
        }
        if j < lines[0].len() - 3 { 
            if xmas(&i + 1, &j + 1, &i + 2, &j + 2, &i + 3, &j + 3, &lines) {
                count += 1;
            }
        }
    }

    // W
    if j >= 3 {
        if xmas(&i + 0, &j - 1, &i + 0, &j - 2, &i + 0, &j - 3, &lines) {
            count += 1;
        }
    }

    // E
    if j < lines[0].len() - 3 {
        if xmas(&i + 0, &j + 1, &i + 0, &j + 2, &i + 0, &j + 3, &lines) {
            count += 1;
        }
    }
    return count;
}

fn p2(i: usize, j: usize, lines: &Vec<String>) -> i32 {
    if i == 0 || j == 0 || i == lines.len() - 1 || j == lines.len() -1 {return 0};

    if x_mas(&i - 1, &j - 1, &i + 1, &j + 1, &lines) && x_mas(&i + 1, &j - 1, &i - 1, &j + 1, &lines) {return 1};

    return 0;
}

fn get_char(i: &usize, j: &usize, lines: &Vec<String>) -> char {
    return lines[*i].chars().nth(*j).unwrap();
}

fn xmas(i1: usize, j1: usize, i2: usize, j2: usize, i3: usize, j3: usize, lines: &Vec<String>) -> bool {
    if get_char(&i1, &j1, &lines) == 'M' &&  get_char(&i2, &j2, &lines) == 'A' && get_char(&i3, &j3, &lines) == 'S' {
        return true;
    }
    return false;
}

fn x_mas(i1: usize, j1: usize, i2: usize, j2: usize, lines: &Vec<String>) -> bool {
    if  (get_char(&i1, &j1, &lines) == 'M' || get_char(&i1, &j1, &lines) == 'S') &&
        (get_char(&i2, &j2, &lines) == 'M' || get_char(&i2, &j2, &lines) == 'S') &&
        (get_char(&i1, &j1, &lines) != get_char(&i2, &j2, &lines))
    {
            return true;
    }
    return false;
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)  
        .collect() 
}
