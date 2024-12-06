use std::fs::read_to_string;

fn main() {
    let mut lines: Vec<String> = read_lines("input.in");

    let (mut y, mut x, mut dir) = get_pos(&lines);
    let mut out: bool = false;

    while !out {
        let (i, j) = get_dir(&dir);

        if (y == 0 && i == -1) || (y == lines.len() - 1 && i == 1) ||
           (x == 0 && j == -1) || (x == lines[0].len() - 1 && j == 1)
        {
            lines[y].replace_range(x..(x + 1), "X");
            out = true;
            break;
        }

        if get_char(&lines[add(&y, &i)], add(&x, &j)) == '#' {
            dir = get_new_dir(&dir);
            lines[y].replace_range(x..(x + 1), &dir.to_string());
        } else {
            lines[y].replace_range(x..(x + 1), "X");
            y = add(&y, &i);
            x = add(&x, &j);

            lines[y].replace_range(x..(x + 1), &dir.to_string());
        }

    }

    let mut p1: i32 = 0;
    for line in lines {
        for i in 0..line.len() {
            if get_char(&line, i) == 'X' {
                p1 += 1;
            }
        }
    }

    let mut p2: i32 = 0;
    let mut lines2: Vec<String> = read_lines("input.in");

    let mut done: usize = 0;

    for i in 0..lines2.len() {
        for j in 0..lines2[0].len() {
            done += 1;
            if get_char(&lines2[i], j) == '^' {
                continue;
            }
            let mut test: Vec<String> = lines2.to_vec();
            test[i].replace_range(j..(j+1), "#");
            if check_loop(&mut test, 100000) {
                p2 += 1;
            }
            println!("{} / {}", done, p1);
        }
    }

    println!("{}, {}", p1, p2);
}

fn check_loop(lines: &mut Vec<String>, max: i32) -> bool {
    let (mut y, mut x, mut dir) = get_pos(&lines);
    let mut count: i32 = 0;

    while count < max {
        let (i, j) = get_dir(&dir);

        if (y == 0 && i == -1) || (y == lines.len() - 1 && i == 1) ||
           (x == 0 && j == -1) || (x == lines[0].len() - 1 && j == 1)
        {
            lines[y].replace_range(x..(x + 1), "X");
            return false;
        }

        if get_char(&lines[add(&y, &i)], add(&x, &j)) == '#' {
            dir = get_new_dir(&dir);
            lines[y].replace_range(x..(x + 1), &dir.to_string());
        } else {
            lines[y].replace_range(x..(x + 1), "X");
            y = add(&y, &i);
            x = add(&x, &j);

            lines[y].replace_range(x..(x + 1), &dir.to_string());

            count += 1;
        }

    }

    return true;
}

fn get_pos(lines: &Vec<String>) -> (usize, usize, char) {
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if get_dir(&get_char(&lines[i], j)) != (0, 0) {
                return (i, j, get_char(&lines[i], j));
            }
        }
    }
    return (0, 0, 'N');
}

fn get_dir(pos: &char) -> (i32, i32) {
    match pos {
        '^' => return (-1, 0),
        'v' => return (1, 0),
        '>' => return (0, 1),
        '<' => return (0, -1),
        _ => return (0, 0)
    };
}

fn get_new_dir(pos: &char) -> char {
    match pos {
        '^' => return '>',
        'v' => return '<',
        '>' => return 'v',
        '<' => return '^',
        _ => {
            println!("Unknown direction \"{}\"", pos);
            return 'N';
        }
    };
}

fn add(a: &usize, b: &i32) -> usize {
    match b {
        -1 => return a - 1,
        1 => return a + 1,
        _ => return *a
    };
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
