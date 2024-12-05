use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("input.in");
    let mut rules: Vec<Vec<i32>> = vec![];
    let mut updates: Vec<Vec<i32>> = vec![];
    for line in lines {
        if line.len() == 0 { continue; }
        if line.chars().nth(2).unwrap() == '|' {
            rules.push(get_pages(&line));
        } else {
            updates.push(get_pages(&line));
        }
    }

    let mut sum_corr: i32 = 0;
    let mut sum_incorr: i32 = 0;
    for update in updates {
        if check(&rules, &update) {
            sum_corr += update[update.len() / 2];
        } else {
            let ordered: Vec<i32> = sort(&rules, &update);
            sum_incorr += ordered[ordered.len() / 2];
        }
    }

    println!("P1: {}\nP2: {}", sum_corr, sum_incorr);
}

fn check(rules: &Vec<Vec<i32>>, update: &Vec<i32>) -> bool {
    for i in 0..update.len() {
        for rule in rules {
            if update[i] == rule[1] {
                for j in i..update.len() {
                    if update[j] == rule[0] {
                        return false;
                    }
                }
            }
        }
    }
    return true;
}

fn sort(rules: &Vec<Vec<i32>>, update: &Vec<i32>) -> Vec<i32> {
    let mut ordered: Vec<i32> = update.to_vec();
  
    while !check(&rules, &ordered) {
        for i in 0..ordered.len() {
            'rules: for rule in rules {
                if ordered[i] == rule[1] {
                    for j in i..ordered.len() {
                        if ordered[j] == rule[0] {
                            ordered.swap(i, j);
                            break 'rules;
                        }
                    }
                }
            }
        } 
    }

    return ordered;
}

fn get_pages(str: &String) -> Vec<i32> {
    let mut pages: Vec<i32> = vec![];
    for i in (0..str.len()).step_by(3) {
        pages.push(str[i..(i + 2)].parse::<i32>().unwrap());
    }

    return pages;
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap() 
        .lines() 
        .map(String::from)  
        .collect() 
}
