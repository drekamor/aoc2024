use std::fs::read_to_string;

fn main() {
    //part_1();
    part_2();
}

fn part_1() {
    let lines: Vec<String> = read_lines("input.in");
    let mut map: Vec<i32> = vec![];

    let mut id: i32 = 0;
    let mut added: bool = false;

    for i in 0..lines[0].len() {
        let ch: u32 = get_char(&lines[0], i).to_digit(10).unwrap();
        for _j in 0..ch {
            if i % 2 == 0 {
                map.push(id);
                added = true;
            } else {
                map.push(-1);
            }
        }
        if added {
            id += 1;
            added = false;
        }
    }

    while map[map.len() - 1] == -1_i32 {
        map.remove(map.len() - 1);
    }

    while !check(&map) {
        for i in 0..map.len() {
            if map[i] == -1 {
                map[i] = map[map.len() - 1];
                map.remove(map.len() - 1);
                break;
            }
        }
    }

    let mut checksum: i64 = 0;
    for i in 0..map.len() {
        checksum += map[i] as i64 * i as i64;
    }

    println!("P1: {}", checksum);
}

#[derive(Debug, Clone)]
struct File {
    num: u32,
    id: i32,
}

fn part_2() {
    let lines: Vec<String> = read_lines("input.in");
    let mut map: Vec<File> = vec![];

    let mut id: i32 = 0;

    for i in 0..lines[0].len() {
        let ch = get_char(&lines[0], i).to_digit(10).unwrap() as u32;
        if i % 2 == 0 {
            map.push( File { num: ch, id: id } );
            id += 1;
        } else {
            map.push( File { num: ch, id: -1 } );
        }
    }

    let mut changes: i32 = 0;
    while id > 0 {
        'outter: for i in (0..map.len()).rev() {
            if map[i].id == id {
                for j in 0..i {
                    if map[j].id == -1 && map[j].num >= map[i].num * (map[i].id.abs() as f64 + 0.1).log10().ceil() as u32 {
                        let temp: File = map[i].clone();
                        let mut size: u32 = 0;
                        if i < map.len() - 1 && map[i + 1].id == -1 {
                            size += map[i + 1].num;
                            map.remove(i + 1);
                        }

                        let mut rem: bool = false;
                        if i > 0 && map[i - 1].id == -1 {
                            size += map[i - 1].num;
                            map.remove(i - 1);
                            rem = true;
                        }

                        if rem {
                            map[i-1] = File { num: size + temp.num * (temp.id.abs() as f64 + 0.1).log10().ceil() as u32 , id: -1};
                        } else {
                            map[i] = File { num: size + temp.num * (temp.id.abs() as f64 + 0.1).log10().ceil() as u32 , id: -1}; 
                        }

                        size = map[j].num;
                        map[j] = temp.clone();

                        if size > temp.num * (temp.id.abs() as f64 + 0.1).log10().ceil() as u32 { 
                            size -= temp.num * (temp.id.abs() as f64 + 0.1).log10().ceil() as u32;
                            map.splice((j+1)..(j+1), vec![ File { num: size, id: -1 } ]);
                        }
                        break 'outter;
                    }
                }
            }
        }
        id -= 1;
    }
    
    let mut checksum: i64 = 0;

    let mut str: String = "".to_string();

    for i in &map {
        for _j in 0..i.num {
            if i.id != -1 { str.push_str(&i.id.to_string()) } else { str.push_str(&".".to_string()) };
        }
    }

    for i in 0..str.len() {
        println!("{} / {}", i, str.len());
        let ch: char = get_char(&str, i);
        if ch != '.' {
            checksum += (ch.to_digit(10).unwrap() as i64) * (i as i64)
        }
    }

    print(&map);

    println!("{}", checksum);
}

fn print(vec: &Vec<File>) {
    let mut str: String = "".to_string();

    for i in vec {
        for _j in 0..i.num {
            if i.id != -1 { str.push_str(&i.id.to_string()) } else { str.push_str(&".".to_string()) };
        }
    }

    println!("{}", str);
}

fn check(map: &Vec<i32>) -> bool {
    let mut empty: bool = false;

    for i in map {
        if *i == -1 { empty = true };

        if *i != -1 && empty { return false };
    }

    return true;
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
