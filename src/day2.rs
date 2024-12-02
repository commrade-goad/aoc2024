use std::fs;
use std::io::{BufRead, BufReader};

pub fn day2_p1(file_path: &str) -> Option<usize>{
    let file = fs::File::open(file_path);
    match file {
        Err(_) => return None,
        _ => {}
    }
    let reader = BufReader::new(file.unwrap());

    let parsed: Vec<Vec<usize>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|v| v.parse::<usize>().ok())
                .collect()
        })
        .collect();

    let mut result: usize = 0;
    for line in parsed {
        let is_asc = line[0] < line[1];
        let cline = line.clone();
        let mut idx = 0;
        let mut valid:bool = true;
        while idx < cline.len() - 1 {
            let diff: usize;
            if is_asc { diff = line[idx+1] - line[idx] }
            else { diff = line[idx] - line[idx+1] }

            if is_asc && line[idx] > line[idx+1]  { valid = false }
            if !is_asc && line[idx] < line[idx+1] { valid = false }
            if diff > 3 || diff < 1               { valid = false }
            idx += 1;
        }
        if valid {
            result += 1;
        }
    }
    return Some(result);
}

pub fn day2_p2(file_path: &str) -> Option<usize> {
    let file = fs::File::open(file_path).ok()?;
    let reader = BufReader::new(file);

    let parsed: Vec<Vec<usize>> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split_whitespace()
                .filter_map(|v| v.parse::<usize>().ok())
                .collect()
        })
        .collect();

    let mut result = 0;

    for num in parsed {
        if is_safe(&num) {
            result += 1;
        } else {
            for i in 0..num.len() {
                let mut modified = num.clone();
                modified.remove(i);
                if is_safe(&modified) {
                    result += 1;
                    break;
                }
            }
        }
    }
    Some(result)
}

fn is_safe(num: &[usize]) -> bool {
    let is_desc = num[0] > num[1];
    for i in 0..num.len() - 1 {
        let diff = if is_desc {
            num[i] - num[i + 1]
        } else {
            num[i + 1] - num[i]
        };

        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true
}
