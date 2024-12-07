use std::fs;

pub fn day4_p1(file_path: &str) -> Option<usize> {
    let content = fs::read_to_string(file_path).ok()?;
    if content.trim().is_empty() {
        eprintln!("ERROR : File is empty.");
    }

    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    if grid.is_empty() || grid[0].is_empty() {
        eprintln!("ERROR : Grid is empty or improperly formatted.");
    }

    let rows = grid.len();
    let cols = grid[0].len();
    for row in &grid {
        if row.len() != cols {
            eprintln!("ERROR : Inconsistent row lengths in the grid.");
        }
    }

    let target = "XMAS";
    let rev_target: String = target.chars().rev().collect();
    let target_len = target.len();

    let mut count = 0;

    // Helper to extract a diagonal (down-right or down-left)
    let extract_diag = |grid: &Vec<Vec<char>>, x: usize, y: usize, dx: isize, dy: isize| -> Option<String> {
        let mut result = String::new();
        for i in 0..target_len {
            let nx = (x as isize + i as isize * dx) as usize;
            let ny = (y as isize + i as isize * dy) as usize;
            if ny >= rows || nx >= cols {
                return None; // Out of bounds
            }
            result.push(grid[ny][nx]);
        }
        Some(result)
    };

    for y in 0..rows {
        for x in 0..cols {
            // Check horizontal (right)
            if x + target_len <= cols {
                let s: String = grid[y][x..x + target_len].iter().collect();
                if s == target || s == rev_target {
                    count += 1;
                }
            }

            // Check vertical (down)
            if y + target_len <= rows {
                let s: String = (0..target_len).map(|i| grid[y + i][x]).collect();
                if s == target || s == rev_target {
                    count += 1;
                }
            }

            // Check diagonal (down-right)
            if let Some(s) = extract_diag(&grid, x, y, 1, 1) {
                if s == target || s == rev_target {
                    count += 1;
                }
            }

            // Check diagonal (down-left)
            if let Some(s) = extract_diag(&grid, x, y, -1, 1) {
                if s == target || s == rev_target {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

/// Extracts a diagonal from a 2D grid starting at (y, x) and moving down-right (diag1).
fn diag1(grid: &Vec<Vec<char>>, y: usize, x: usize, len: usize) -> Option<String> {
    let mut s = String::new();
    for k in 0..len {
        if y + k < grid.len() && x + k < grid[0].len() {
            s.push(grid[y + k][x + k]);
        } else {
            return None; // Out of bounds
        }
    }
    Some(s)
}

/// Extracts a diagonal from a 2D grid starting at (y, x) and moving down-left (diag2).
fn diag2(grid: &Vec<Vec<char>>, y: usize, x: usize, len: usize) -> Option<String> {
    let mut s = String::new();
    for k in 0..len {
        if y + k < grid.len() && x >= k {
            s.push(grid[y + k][x - k]);
        } else {
            return None; // Out of bounds
        }
    }
    Some(s)
}

pub fn day4_p2(file_path: &str) -> Option<usize> {
    let content = fs::read_to_string(file_path).ok()?;
    if content.trim().is_empty() {
        eprintln!("ERROR : File is empty.");
    }

    let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
    if grid.is_empty() || grid[0].is_empty() {
        eprintln!("ERROR : Grid is empty or improperly formatted.");
    }

    let rows = grid.len();
    let cols = grid[0].len();
    for row in &grid {
        if row.len() != cols {
            eprintln!("ERROR : Inconsistent row lengths in the grid.");
        }
    }

    let mut count = 0;

    for y in 1..cols-1 {
        for x in 1..rows-1 {
            if grid[y][x] == 'A' {
                let s1: String = diag1(&grid, y - 1, x - 1, 3)?;
                if s1 != "MAS" && s1 != "SAM" { continue; }
                let s2: String = diag2(&grid, y - 1, x + 1, 3)?;
                if s2 == "MAS" || s2 == "SAM" { count +=1; }
            }
        }
    }
    return Some(count);
}
