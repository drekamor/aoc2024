use std::fs::read_to_string;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Node {
    val: u8,
    i: usize,
    j: usize,
}

fn main() {
    let lines: Vec<String> = read_lines("input.in");

    let mut count: u32 = 0;

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if parse_char(&lines[i], j) == 0 {
                count += count_distinct_trails(&lines, Node { val: 0, i: i, j: j});
            }
        }
    }
    println!("{}", count);
}

fn count_trails(map: &Vec<String>, head: Node) -> u32 {
    let mut nodes: Vec<Node> = vec![head];
    let mut current: u8 = 1;

    while current <= 9 {
        let mut current_nodes: HashSet<Node> = HashSet::default();
        for node in nodes.clone().into_iter().rev() {
            if node.val != current - 1 { break; }
        
            if node.i > 0 {
                if parse_char(&map[node.i - 1], node.j) == current {
                    current_nodes.insert( Node { val: current, i: node.i - 1, j: node.j } );
                }
            }

            if node.i < map.len() - 1 {
                if parse_char(&map[node.i + 1], node.j) == current {
                    current_nodes.insert( Node { val: current, i: node.i + 1, j: node.j } );
                }
            }

            if node.j > 0 {
                if parse_char(&map[node.i], node.j - 1) == current {
                    current_nodes.insert( Node { val: current, i: node.i, j: node.j - 1 } );
                }
            }

            if node.j < map[node.i].len() - 1 {
                if parse_char(&map[node.i], node.j + 1) == current {
                    current_nodes.insert( Node { val: current, i: node.i, j: node.j + 1 } );
                }
            }
        }
        nodes.append(&mut current_nodes.into_iter().collect());
        current += 1;
    }

    let mut count: u32 = 0;
    println!("{:?}", nodes);

    for node in nodes {
        if node.val == 9 {
            count += 1;
        }
    }
    println!("{}", count);
    return count;
}

fn count_distinct_trails(map: &Vec<String>, head: Node) -> u32 {
    let mut nodes: Vec<Node> = vec![head];
    let mut current: u8 = 1;

    while current <= 9 {
        let mut current_nodes: Vec<Node> = vec![];
        for node in nodes.clone().into_iter().rev() {
            if node.val != current - 1 { break; }
        
            if node.i > 0 {
                if parse_char(&map[node.i - 1], node.j) == current {
                    current_nodes.push( Node { val: current, i: node.i - 1, j: node.j } );
                }
            }

            if node.i < map.len() - 1 {
                if parse_char(&map[node.i + 1], node.j) == current {
                    current_nodes.push( Node { val: current, i: node.i + 1, j: node.j } );
                }
            }

            if node.j > 0 {
                if parse_char(&map[node.i], node.j - 1) == current {
                    current_nodes.push( Node { val: current, i: node.i, j: node.j - 1 } );
                }
            }

            if node.j < map[node.i].len() - 1 {
                if parse_char(&map[node.i], node.j + 1) == current {
                    current_nodes.push( Node { val: current, i: node.i, j: node.j + 1 } );
                }
            }
        }
        nodes.append(&mut current_nodes);
        current += 1;
    }

    let mut count: u32 = 0;
    println!("{:?}", nodes);

    for node in nodes {
        if node.val == 9 {
            count += 1;
        }
    }
    println!("{}", count);
    return count;
}

fn parse_char(str: &String, i: usize) -> u8 {
    return str[i..(i + 1)].parse::<u8>().unwrap();
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)  
        .collect() 
}
