use std::collections::VecDeque;
use std::fs;

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn neighbors(&self, width: usize, height: usize) -> Vec<Point> {
        let mut result = Vec::new();
        if self.x > 0 {
            result.push(Point {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.x + 1 < width {
            result.push(Point {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            result.push(Point {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.y + 1 < height {
            result.push(Point {
                x: self.x,
                y: self.y + 1,
            });
        }
        return result;
    }
}

type Grid = Vec<Vec<u8>>;

fn part1_search(grid: &Grid) -> u32 {
    let height = grid.len();
    let width = grid[0].len();

    return grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &value)| (x, y, value)))
        .filter(|&(_, _, value)| value == 0)
        .map(|(x, y, _)| {
            let start = Point { x, y };
            let mut checked = vec![vec![false; width]; height];
            let mut queue = VecDeque::new();
            queue.push_back(start);

            let mut nines_reached = 0;

            while let Some(current) = queue.pop_front() {
                if checked[current.y][current.x] {
                    continue;
                }
                checked[current.y][current.x] = true;

                let current_value = grid[current.y][current.x];

                if current_value == 9 {
                    nines_reached += 1;
                    continue;
                }

                for neighbor in current.neighbors(width, height) {
                    if !checked[neighbor.y][neighbor.x]
                        && grid[neighbor.y][neighbor.x] == current_value + 1
                    {
                        queue.push_back(neighbor);
                    }
                }
            }

            nines_reached
        })
        .sum();
}

pub fn part1(fpath: &str) -> Option<u32> {
    let content: String = fs::read_to_string(fpath).ok()?;
    let matrix: Grid = content
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .map(|(_, v)| v.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect();

    let result: u32 = part1_search(&matrix);
    return Some(result);
}

type Ratings = Vec<Vec<u32>>;

fn part2_dynamic(grid: &Grid) -> u32 {
    let height = grid.len();
    let width = grid[0].len();

    let mut ratings: Ratings = vec![vec![0; width]; height];

    // All 9s can be reached one way
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 9 {
                ratings[y][x] = 1;
            }
        }
    }

    // For each height, sum the ratings of all points one higher
    for current_height in (0..=8).rev() {
        for y in 0..height {
            for x in 0..width {
                if grid[y][x] == current_height {
                    let point = Point { x, y };
                    ratings[y][x] = point
                        .neighbors(width, height)
                        .iter()
                        .filter_map(|neighbor| {
                            if grid[neighbor.y][neighbor.x] == current_height + 1 {
                                Some(ratings[neighbor.y][neighbor.x])
                            } else {
                                None
                            }
                        })
                        .sum();
                }
            }
        }
    }

    // Sum the ratings of the 0s
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, &value)| (x, y, value)))
        .filter(|&(_, _, value)| value == 0)
        .map(|(x, y, _)| ratings[y][x])
        .sum()
}

pub fn part2(fpath: &str) -> Option<u32> {
    let content: String = fs::read_to_string(fpath).ok()?;
    let matrix: Grid = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let result = part2_dynamic(&matrix);
    // please convert the code i give you before and plug it here
    return Some(result)
}
