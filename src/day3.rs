use std::fs;

pub fn day3_p1(file_path: &str) -> Option<usize>{
    let file = fs::File::open(file_path);
    match file {
        Err(_) => return None,
        _ => {}
    }
    let mut statement: Vec<String> = Vec::new();
    let parsed: String = fs::read_to_string(file_path).unwrap().trim().to_string();
    let bufferochar: Vec<char> = parsed.chars().collect();
    for i in 0..bufferochar.len() {
        if i+12 >= bufferochar.len() {
            break;
        }
        let builder_char: Vec<char> = bufferochar[i..i+12].to_vec();
        let builded: String = builder_char.iter().cloned().collect();
        if &builded[0..4] == "mul(" {
            let mut tmp: String = String::new();
            let pos = builded.find(")");
            if pos.is_none() {
                continue;
            }
            if builded.find(",").is_none() { continue; }
            tmp.push_str(&builded[0..pos.unwrap() +1]);
            statement.push(tmp);
        }
    }

    let mut res: usize = 0;
    for state in statement {
        let first_pos = 4;
        let first_end_pos = state.find(",").unwrap();
        let first_num: &str = &state[first_pos..first_end_pos];
        let sec_num: &str = &state[first_end_pos + 1..state.len()-1];
        let pfirst = first_num.trim().parse::<usize>();
        let psecond = sec_num.trim().parse::<usize>();
        if pfirst.is_err() || psecond.is_err() {continue;}
        res += pfirst.unwrap() * psecond.unwrap();
    }

    return Some(res);
}

pub fn day3_p2(file_path: &str) -> Option<usize>{
    let file = fs::File::open(file_path);
    match file {
        Err(_) => return None,
        _ => {}
    }
    let mut statement: Vec<String> = Vec::new();
    let parsed: String = fs::read_to_string(file_path).unwrap().trim().to_string();
    let bufferochar: Vec<char> = parsed.chars().collect();
    let mut enabled_mode:bool = true;
    for i in 0..bufferochar.len() {
        if i+12 >= bufferochar.len() {
            break;
        }
        let builder_char: Vec<char> = bufferochar[i..i+12].to_vec();
        let builded: String = builder_char.iter().cloned().collect();
        if &builded[0..4] == "do()" {
            enabled_mode = true;
        }
        if &builded[0..7] == "don't()" {
            enabled_mode = false;
        }
        if &builded[0..4] == "mul(" && enabled_mode {
            let mut tmp: String = String::new();
            let pos = builded.find(")");
            if pos.is_none() {
                continue;
            }
            if builded.find(",").is_none() { continue; }
            tmp.push_str(&builded[0..pos.unwrap() +1]);
            statement.push(tmp);
        }
    }

    let mut res: usize = 0;
    for state in statement {
        let first_pos = 4;
        let first_end_pos = state.find(",").unwrap();
        let first_num: &str = &state[first_pos..first_end_pos];
        let sec_num: &str = &state[first_end_pos + 1..state.len()-1];
        let pfirst = first_num.trim().parse::<usize>();
        let psecond = sec_num.trim().parse::<usize>();
        if pfirst.is_err() || psecond.is_err() {continue;}
        res += pfirst.unwrap() * psecond.unwrap();
    }

    return Some(res);
}
