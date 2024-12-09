use std::fs;

fn do_calculation1(p: &mut (usize, Vec<usize>), idx: usize, current_val: usize) -> bool {
    if current_val > p.0 { return false }
    if idx >= p.1.len() { return current_val == p.0 }
    if do_calculation1(p, idx + 1, current_val + p.1[idx]) { return true }
    if do_calculation1(p, idx + 1, current_val * p.1[idx]) { return true }
    return false
}

fn do_calculation2(p: &mut (usize, Vec<usize>), idx: usize, current_val: usize) -> bool {
    if current_val > p.0 { return false }
    if idx >= p.1.len() { return current_val == p.0 }
    if do_calculation2(p, idx + 1, current_val + p.1[idx]) { return true }
    // let tmp: String = current_val.to_string() + &p.1[idx].to_string();
    // if do_calculation2(p, idx + 1, tmp.trim().parse().unwrap()) { return true }
    let tmp = current_val * 10_usize.pow(p.1[idx].to_string().len() as u32) + p.1[idx];
    if do_calculation2(p, idx + 1, tmp) { return true }
    if do_calculation2(p, idx + 1, current_val * p.1[idx]) { return true }
    return false
}

pub fn part1(file_path: &str) -> Option<usize> {
    let content = fs::read_to_string(file_path).ok()?;
    if content.trim().is_empty() {
        eprintln!("ERROR: File is empty.");
        return None;
    }

    let mut count = 0; 
    let mut buffer: Vec<(usize, Vec<usize>)> = Vec::new();

     for line in content.lines() {
        let buff: Vec<&str> = line.split_whitespace().collect();
        let mut res: (usize, Vec<usize>) = (0, Vec::new());
        buff.iter().for_each(|e| {
            if let Some(pos) = e.find(':') {
                res.0 = e[0..pos].trim().parse().unwrap();
            } else {
                res.1.push(e.trim().parse().unwrap());
            }
        });
        buffer.push(res);
    }

    for mut line in buffer {
        if do_calculation1(&mut line, 0, 0) {
            count += line.0;
        }
    }

    return Some(count)
}
pub fn part2(file_path: &str) -> Option<usize> {
    let content = fs::read_to_string(file_path).ok()?;
    if content.trim().is_empty() {
        eprintln!("ERROR: File is empty.");
        return None;
    }

    let mut count = 0; 
    let mut buffer: Vec<(usize, Vec<usize>)> = Vec::new();

     for line in content.lines() {
        let buff: Vec<&str> = line.split_whitespace().collect();
        let mut res: (usize, Vec<usize>) = (0, Vec::new());
        buff.iter().for_each(|e| {
            if let Some(pos) = e.find(':') {
                res.0 = e[0..pos].trim().parse().unwrap();
            } else {
                res.1.push(e.trim().parse().unwrap());
            }
        });
        buffer.push(res);
    }

    for mut line in buffer {
        if do_calculation2(&mut line, 0, 0) {
            count += line.0;
        }
    }

    return Some(count)
}
