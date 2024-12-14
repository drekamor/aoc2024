use std::fs::read_to_string;
use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Field {
    crop: String,
    i: usize,
    j: usize,
}

fn main() {
    let lines: Vec<String> = read_lines("input.in");

    let mut regions: Vec<HashSet<Field>> = vec![];

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let region = get_region(&lines, Field { crop: lines[i][j..j + 1].to_string(), i: i, j: j });
            if !regions.contains(&region) {
                regions.push(region);
            }
        }
    }

    let mut count: usize = 0;
    
    for region in regions {
        count += get_discounted_price(&region, lines.len(), lines[0].len());
    }

    println!("{:?}", count);
}

fn get_region(map: &Vec<String>, head: Field) -> HashSet<Field> {
    let mut fields: HashSet<Field> = vec![head].into_iter().collect();

    let mut added: bool = true;
    while added {
        added = false;
        for field in &mut fields.clone().into_iter() {
            if field.i > 0 {
                if map[field.i - 1][field.j..(field.j + 1)] == field.crop {
                    if fields.insert( Field { crop: field.crop.clone(), i: field.i - 1, j: field.j } ) {
                        added = true;
                    }
                }
            }

            if field.i < map.len() - 1 {
                if map[field.i + 1][field.j..(field.j + 1)] == field.crop {
                    if fields.insert( Field { crop: field.crop.clone(), i: field.i + 1, j: field.j } ) {
                         added = true;
                    }
                }
            }

            if field.j > 0 {
                if map[field.i][(field.j - 1)..field.j] == field.crop {
                    if fields.insert( Field { crop: field.crop.clone(), i: field.i, j: field.j - 1 } ) {
                        added = true;
                    }
                }
            }

            if field.j < map[field.i].len() - 1 {
                if map[field.i][(field.j + 1)..(field.j + 2)] == field.crop {
                    if fields.insert( Field { crop: field.crop.clone(), i: field.i, j: field.j + 1 } ) {
                        added = true;
                    }
                }
            }
        }
    }

    return fields;
}

fn get_price(region: &HashSet<Field>, i_max: usize, j_max: usize) -> usize {
    let mut p: usize = 0;

    for field in region {
        if field.i == 0 || field.i == i_max { p += 1 };
        if field.j == 0 || field.j == j_max { p += 1 };
        
        if field.i != 0 && !region.contains( &Field { crop: field.crop.clone(), i: field.i - 1, j: field.j }) { p += 1 };
        if field.i != i_max && !region.contains( &Field { crop: field.crop.clone(), i: field.i + 1, j: field.j }) { p += 1 };
        if field.j != 0 && !region.contains( &Field { crop: field.crop.clone(), i: field.i, j: field.j - 1 }) { p += 1 };
        if field.j != j_max && !region.contains( &Field { crop: field.crop.clone(), i: field.i, j: field.j + 1 }) { p += 1 };
    }

    return region.len() * p;
}

fn get_discounted_price(region: &HashSet<Field>, i_max: usize, j_max: usize) -> usize {
    let mut p: usize = 0;



    for field in region {
        let north: bool = field.i != 0 && region.contains(&Field{ crop: field.crop.clone(), i: field.i - 1, j: field.j });
        let south: bool = field.i != i_max && region.contains(&Field{ crop: field.crop.clone(), i: field.i + 1, j: field.j });
        let east: bool = field.j != j_max && region.contains(&Field{ crop: field.crop.clone(), i: field.i, j: field.j + 1 });
        let west: bool = field.j != 0 && region.contains(&Field{ crop: field.crop.clone(), i: field.i, j: field.j - 1});

        let ne: bool = field.i != 0 && field.j != j_max && region.contains(&Field{ crop: field.crop.clone(), i: field.i - 1, j: field.j + 1 });
        let sw: bool = field.i != i_max && field.j != 0 && region.contains(&Field{ crop: field.crop.clone(), i: field.i + 1, j: field.j - 1 });
        let nw: bool = field.i != 0 && field.j != 0 && region.contains(&Field{ crop: field.crop.clone(), i: field.i - 1, j: field.j - 1 });
        let se: bool = field.i != i_max && field.j != j_max && region.contains(&Field{ crop: field.crop.clone(), i: field.i + 1, j: field.j + 1 });

        if (north ^ south) && (east ^ west) { p += 1 };
        if north as i8 + south as i8 + east as i8 + west as i8 == 1 { p += 2 };
        if north && east && !ne { p += 1 };
        if north && west && !nw { p += 1 };
        if south && east && !se { p += 1 };
        if south && west && !sw { p += 1 };
    }

    if region.len() == 1 { p = 4 };
    return region.len() * p;
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)  
        .collect() 
}
