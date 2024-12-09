use std::collections::HashSet;
use std::fs;

pub fn part1(file_path: &str) -> Option<usize> {
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // [Up, Right, Down, Left]
    let content = fs::read_to_string(file_path).ok()?;
    if content.trim().is_empty() {
        eprintln!("ERROR: File is empty.");
        return None;
    }

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut guard_pos = (0, 0); // (row, col)
    let mut guard_dir = 0; // Starting direction is "Up"

    // Parse input to initialize the map and guard's position
    for (row, line) in content.lines().enumerate() {
        let line_vec: Vec<char> = line.chars().collect();
        if let Some(col) = line_vec.iter().position(|&c| c == '^') {
            guard_pos = (row as i32, col as i32);
        }
        map.push(line_vec);
    }

    let mut visited_positions = HashSet::new();
    visited_positions.insert(guard_pos);

    loop {
        let (dx, dy) = directions[guard_dir];
        let new_pos = (guard_pos.0 + dx, guard_pos.1 + dy);

        // Check bounds
        if new_pos.0 < 0
            || new_pos.1 < 0
            || new_pos.0 >= map.len() as i32
            || new_pos.1 >= map[0].len() as i32
        {
            break;
        }

        match map[new_pos.0 as usize][new_pos.1 as usize] {
            '#' => {
                // Turn right
                guard_dir = (guard_dir + 1) % directions.len();
            }
            '.' | '^' => {
                // Move forward
                guard_pos = new_pos;
                visited_positions.insert(guard_pos);
            }
            _ => break,
        }
    }
    Some(visited_positions.len())
}

pub fn part2(file_path: &str) -> Option<usize> {
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // [Up, Right, Down, Left]
    let content = fs::read_to_string(file_path).ok()?;
    if content.trim().is_empty() {
        eprintln!("ERROR: File is empty.");
        return None;
    }

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut guard_pos = (0, 0); // (row, col)

    // Parse input to initialize the map and guard's position
    for (row, line) in content.lines().enumerate() {
        let line_vec: Vec<char> = line.chars().collect();
        if let Some(col) = line_vec.iter().position(|&c| c == '^') {
            guard_pos = (row as i32, col as i32);
        }
        map.push(line_vec);
    }

    let guard_start_pos = guard_pos;
    let mut loop_count = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == '#' || (row as i32, col as i32) == guard_start_pos {
                continue;
            }

            // Temporarily block this position
            map[row][col] = '#';
            if is_looping(&map, guard_start_pos, &directions) {
                loop_count += 1;
            }
            map[row][col] = '.';
        }
    }
    Some(loop_count)
}

fn is_looping(map: &[Vec<char>], start: (i32, i32), directions: &[(i32, i32)]) -> bool {
    let mut visited_positions = HashSet::new();
    let mut pos = start;
    let mut dir = 0; // Start direction "Up"

    loop {
        let (dx, dy) = directions[dir];
        let new_pos = (pos.0 + dx, pos.1 + dy);

        if new_pos.0 < 0 || new_pos.1 < 0 || new_pos.0 >= map.len() as i32 || new_pos.1 >= map[0].len() as i32 {
            return false; // Exited the grid
        }

        if visited_positions.contains(&(new_pos, dir)) {
            return true; // Loop detected
        }

        visited_positions.insert((new_pos, dir));

        match map[new_pos.0 as usize][new_pos.1 as usize] {
            '#' => {
                dir = (dir + 1) % directions.len(); // Turn right
            }
            '.' | '^' => {
                pos = new_pos; // Move forward
            }
            _ => break,
        }
    }
    false
}
