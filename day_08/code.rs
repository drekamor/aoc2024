use std::fs::read_to_string;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
#[derive(Eq, Hash, PartialEq)]
struct Antenna {
    i: i32,
    j: i32,
}

fn main() {
    let lines: Vec<String> = read_lines("input.in");
    let h: i32 = lines.len() as i32;
    let w: i32 = lines[0].len() as i32;

    let mut antennae: HashMap<char, Vec<Antenna>> = HashMap::new();

    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if get_char(&lines[i], j) != '.' {
                antennae.entry(get_char(&lines[i], j)).or_insert_with(Vec::new).push(
                    Antenna {
                        i: i as i32,
                        j: j as i32,
                    }
                );
            }
        }
    }

    let mut nodes_1: HashSet<Antenna> = Default::default();
    let mut nodes_2: HashSet<Antenna> = Default::default();

    for (_, value) in antennae.into_iter() {
        for i in 0..value.len() {
            for j in 0..value.len() {
                if j <= i { continue };
                for node in get_nodes_1(&value[i], &value[j], &h, &w) {
                    nodes_1.insert(node);
                }
                for node in get_nodes_2(&value[i], &value[j], &h, &w) {
                    nodes_2.insert(node);
                }
            }
        }
    }

    println!("P1: {}; P2: {}", nodes_1.len(), nodes_2.len());
}

fn get_nodes_1(a: &Antenna, b: &Antenna, h: &i32, w: &i32) -> Vec<Antenna> {
    let mut nodes: Vec<Antenna> = vec![];
    let di = a.i - b.i; // 2 - 4 = -2
    let dj = a.j - b.j; // 5 - 4 = 1

    let mut ni = a.i - 2 * di; // 2 + 4 = 6 
    let mut nj = a.j - 2 * dj; // 5 - 2 = 3

    if (ni >= 0 && ni < *h) && (nj >= 0 && nj < *w) { nodes.push(Antenna { i: ni, j: nj }) };

    ni = a.i + di; // 2 - 2 = 0
    nj = a.j + dj;

    if(ni >= 0 && ni < *h) && (nj >= 0 && nj < *w) { nodes.push(Antenna { i: ni, j: nj }) };
    
    return nodes;
}

fn get_nodes_2(a: &Antenna, b: &Antenna, h: &i32, w: &i32) -> Vec<Antenna> {
    let mut nodes: Vec<Antenna> = vec![];
    let di = a.i - b.i; // 2 - 4 = -2
    let dj = a.j - b.j; // 5 - 4 = 1

    let mut l = 0;
    while a.i + l * di >= 0 && a.i + l * di < *h && a.j + l * dj >= 0 && a.j + l * dj < *w {
        nodes.push( Antenna { i: a.i + l * di, j: a.j + l * dj } );
        l -= 1;
    }

    l = 1;
    while a.i + l * di >= 0 && a.i + l * di < *h && a.j + l * dj >= 0 && a.j + l * dj < *w {
        nodes.push( Antenna { i: a.i + l * di, j: a.j + l * dj } );
        l += 1;
    }

    return nodes;
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
