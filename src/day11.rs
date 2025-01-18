use std::fs;
use std::collections::HashMap;

fn enforce_rule(a: &mut Vec<usize>) {
    let mut i = 0;
    while i < a.len() {
        let cur_el: usize = a[i];
        match cur_el {
            0 => a[i] = 1,
            _ => {
                let b: &str = &cur_el.to_string();
                let half: usize = b.len() / 2;
                if b.len() % 2 == 0 {
                    a[i] = b[0..half].trim().parse().unwrap();
                    i += 1;
                    a.insert(i, b[half..b.len()].trim().parse().unwrap());
                } else {
                    a[i] *= 2024
                }
            }
        }
        i += 1;
    }
}

pub fn part1(fpath: &str) -> Option<usize> {
    let fcontent: String = fs::read_to_string(fpath).unwrap();
    let mut content_arr: Vec<usize> = fcontent
        .split_whitespace()
        .into_iter()
        .map(|v| v.trim().parse::<usize>().unwrap())
        .collect();
    const BLINK: i32 = 25;
    for _ in 0..BLINK {
        enforce_rule(&mut content_arr);
    }
    return Some(content_arr.len());
}

fn blink_count_hash(input: &[u64], count: usize) -> usize {
    let mut list1 = HashMap::new();
    let mut list2 = HashMap::new();

    for v in input {
        list1.entry(*v).and_modify(|c| *c += 1).or_insert(1);
    }

    for _ in 0..count {
        for (v, c) in list1.drain() {
            if v == 0 {
                list2.entry(1).and_modify(|c2| *c2 += c).or_insert(c);
            } else {
                let digit_count = v.ilog10() + 1;
                if digit_count % 2 == 0 {
                    let divisor = 10u64.pow(digit_count / 2);
                    list2
                        .entry(v / divisor)
                        .and_modify(|c2| *c2 += c)
                        .or_insert(c);
                    list2
                        .entry(v % divisor)
                        .and_modify(|c2| *c2 += c)
                        .or_insert(c);
                } else {
                    list2.entry(v * 2024).and_modify(|c2| *c2 += c).or_insert(c);
                }
            }
        }

        std::mem::swap(&mut list1, &mut list2);
        list2.clear();
    }

    return list1.values().sum()
}

pub fn part2(fpath: &str) -> Option<usize> {
    let fcontent: String = fs::read_to_string(fpath).unwrap();
    let content_arr: Vec<u64> = fcontent
        .split_whitespace()
        .into_iter()
        .map(|v| v.trim().parse::<u64>().unwrap())
        .collect();
    return Some(blink_count_hash(&content_arr[0..content_arr.len()], 75));
}
