use std::collections::HashMap;
use std::fs;

fn is_valid_update(update: &[usize], rules: &[(usize, usize)]) -> bool {
    let position: HashMap<usize, usize> = update
        .iter()
        .enumerate() // give index and value
        .map(|(i, &page)| (page, i)) // rotate them
        .collect();

    for &(x, y) in rules {
        if let (Some(&pos_x), Some(&pos_y)) = (position.get(&x), position.get(&y)) {
            if pos_x >= pos_y {
                return false;
            }
        }
    }
    return true;
}

fn reorder_update(update: &mut Vec<usize>, rules: &[(usize, usize)]) -> bool {
    let original_update = update.clone();

    let mut changed = true;
    while changed {
        changed = false;

        for &(x, y) in rules {
            if let (Some(pos_x), Some(pos_y)) = (
                update.iter().position(|&page| page == x),
                update.iter().position(|&page| page == y),
            ) {
                if pos_x > pos_y {
                    update.swap(pos_x, pos_y);
                    changed = true;
                }
            }
        }
    }

    original_update != *update
}

fn middle_page(update: &[usize]) -> usize {
    return update[update.len() / 2];
}

pub fn day5_p1(file_path: &str) -> Option<usize> {
    let content = fs::read_to_string(file_path).ok()?;
    if content.trim().is_empty() {
        eprintln!("ERROR : File is empty.");
    }

    let mut rules: Vec<(usize, usize)> = Vec::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();

    for line in content.lines() {
        if line.contains("|") {
            let parts: Vec<usize> = line.split('|').map(|s| s.trim().parse().unwrap()).collect();
            rules.push((parts[0], parts[1]));
        } else if line.contains(",") {
            let pages: Vec<usize> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            updates.push(pages);
        }
    }

    let mut total_middle_sum = 0;
    for update in updates {
        if is_valid_update(&update, &rules) {
            total_middle_sum += middle_page(&update);
        }
    }

    Some(total_middle_sum)
}

pub fn day5_p2(file_path: &str) -> Option<usize> {
    let content = std::fs::read_to_string(file_path).ok()?;
    if content.trim().is_empty() {
        eprintln!("ERROR : File is empty.");
    }

    let mut rules: Vec<(usize, usize)> = Vec::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();

    for line in content.lines() {
        if line.contains("|") {
            let parts: Vec<usize> = line.split('|').map(|s| s.trim().parse().unwrap()).collect();
            rules.push((parts[0], parts[1]));
        } else if line.contains(",") {
            let pages: Vec<usize> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            updates.push(pages);
        }
    }

    let mut total_middle_sum = 0;

    for update in &mut updates {
        if reorder_update(update, &rules) {
            total_middle_sum += middle_page(update);
        }
    }

    Some(total_middle_sum)
}
