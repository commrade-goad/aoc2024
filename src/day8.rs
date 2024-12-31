use std::collections::HashMap;
use std::fs;

pub fn part1(file_path: &str) -> Option<usize> {
    let buffer = fs::read_to_string(file_path).unwrap();
    let mut map: HashMap<char, Vec<[usize; 2]>> = HashMap::new();
    let mut row_len = 0;
    let mut row_count = 0;

    for (row_index, line) in buffer.lines().enumerate() {
        row_len = line.len();
        for (col_index, c) in line.chars().enumerate() {
            if c != '.' {
                map.entry(c).or_insert_with(Vec::new).push([col_index, row_index]);
            }
        }
        row_count += 1;
    }

    let mut traversed = vec![vec![false; row_len]; row_count];
    let mut res = 0;

    for (_, positions) in map {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let dx = positions[i][0] as i32 - positions[j][0] as i32;
                let dy = positions[i][1] as i32 - positions[j][1] as i32;

                let obj1x = positions[i][0] as i32 + dx;
                let obj1y = positions[i][1] as i32 + dy;
                let obj2x = positions[j][0] as i32 - dx;
                let obj2y = positions[j][1] as i32 - dy;

                if obj1x >= 0 && obj1x < row_len as i32 && obj1y >= 0 && obj1y < row_count as i32 {
                    let obj1x = obj1x as usize;
                    let obj1y = obj1y as usize;
                    if !traversed[obj1y][obj1x] {
                        traversed[obj1y][obj1x] = true;
                        res += 1;
                    }
                }

                if obj2x >= 0 && obj2x < row_len as i32 && obj2y >= 0 && obj2y < row_count as i32 {
                    let obj2x = obj2x as usize;
                    let obj2y = obj2y as usize;
                    if !traversed[obj2y][obj2x] {
                        traversed[obj2y][obj2x] = true;
                        res += 1;
                    }
                }
            }
        }
    }
    return Some(res)
}

pub fn part2(file_path: &str) -> Option<usize> {
    let buffer = fs::read_to_string(file_path).unwrap();
    let mut map: HashMap<char, Vec<[usize; 2]>> = HashMap::new();
    let mut row_len = 0;
    let mut row_count = 0;

    for (row_index, line) in buffer.lines().enumerate() {
        row_len = line.len();
        for (col_index, c) in line.chars().enumerate() {
            if c != '.' {
                map.entry(c).or_insert_with(Vec::new).push([col_index, row_index]);
            }
        }
        row_count += 1;
    }

    let mut traversed = vec![vec![false; row_len]; row_count];
    let mut res = 0;

    for (_, positions) in map {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let dx = positions[i][0] as i32 - positions[j][0] as i32;
                let dy = positions[i][1] as i32 - positions[j][1] as i32;

                let mut objx = positions[j][0] as i32 - dx;
                let mut objy = positions[j][1] as i32 - dy;

                while objx >= 0 && objx < row_len as i32 && objy >= 0 && objy < row_count as i32 {
                    let ox:usize = objx as usize;
                    let oy:usize = objy as usize;
                    if !traversed[oy][ox] {
                        traversed[oy][ox] = true;
                        res += 1;
                    }
                    objx -= dx;
                    objy -= dy;
                }
                objx += dx;
                objy += dy;

                while objx >= 0 && objx < row_len as i32 && objy >= 0 && objy < row_count as i32 {
                    let ox:usize = objx as usize;
                    let oy:usize = objy as usize;
                    if !traversed[oy][ox] {
                        traversed[oy][ox] = true;
                        res += 1;
                    }
                    objx += dx;
                    objy += dy;
                }

            }
        }
    }
    return Some(res)
}
