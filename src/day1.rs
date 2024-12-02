use std::fs;
use std::io::{BufRead, BufReader};

pub fn day1_p1(file_path: &str) -> Option<usize>{
    let file = fs::File::open(file_path);
    match file {
        Err(_) => return None,
        _ => {}
    }
    let reader = BufReader::new(file.unwrap());

    let mut parsed_l: Vec<usize> = Vec::new();
    let mut parsed_r: Vec<usize> = Vec::new();
    let lines: Result<Vec<String>, _> = reader.lines().collect::<Result<Vec<String>, _>>();
    for line in lines.unwrap() {
        let spl: Vec<&str> = line.split_whitespace().collect();
        parsed_l.push(spl[0].trim().parse().unwrap());
        parsed_r.push(spl[1].trim().parse().unwrap());
    }

    parsed_l.sort();
    parsed_r.sort();

    let mut result: usize = 0;
    for i in 0..parsed_l.len() {
        if parsed_r[i] > parsed_l[i] {
            result += parsed_r[i] - parsed_l[i];
            continue;
        }
        result += parsed_l[i] - parsed_r[i];
    }

    return Some(result);
}

pub fn day1_p2(file_path: &str) -> Option<usize>{
    let file = fs::File::open(file_path);
    match file {
        Err(_) => return None,
        _ => {}
    }
    let reader = BufReader::new(file.unwrap());

    let mut parsed_l: Vec<usize> = Vec::new();
    let mut parsed_r: Vec<usize> = Vec::new();
    let lines: Result<Vec<String>, _> = reader.lines().collect::<Result<Vec<String>, _>>();
    for line in lines.unwrap() {
        let spl: Vec<&str> = line.split_whitespace().collect();
        parsed_l.push(spl[0].trim().parse().unwrap());
        parsed_r.push(spl[1].trim().parse().unwrap());
    }

    let mut result: usize = 0;
    for i in 0..parsed_l.len() {
        let mut remove_counter_idx: Vec<usize> = Vec::new();
        let l_num = parsed_l[i];
        for j in 0..parsed_r.len() {
            let r_num = parsed_r[j];
            if l_num == r_num {
                remove_counter_idx.push(j);
            }
        }
        remove_counter_idx.sort_by(|a, b| b.cmp(a));
        for i in 0..remove_counter_idx.len() {
            parsed_r.remove(remove_counter_idx[i]);
        }
        result += l_num * remove_counter_idx.len();
    }

    for i in 0..parsed_r.len() {
        let mut remove_counter_idx: Vec<usize> = Vec::new();
        let r_num = parsed_r[i];
        for j in 0..parsed_l.len() {
            let l_num = parsed_l[j];
            if r_num == l_num {
                remove_counter_idx.push(j);
            }
        }
        remove_counter_idx.sort_by(|a, b| b.cmp(a));
        for i in 0..remove_counter_idx.len() {
            parsed_l.remove(remove_counter_idx[i]);
        }
        result += r_num * remove_counter_idx.len();
    }

    return Some(result);
}
