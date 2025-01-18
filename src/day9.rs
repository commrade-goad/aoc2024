use std::fs;

fn readf(fpath: &str) -> String {
    let res: String = fs::read_to_string(fpath).unwrap_or("".to_string()).trim().to_string(); 
    return res;
}

#[derive(Debug, Copy, Clone)]
struct CFile{
    len: usize,
    id: i32,
}

fn pretty_print(p: &Vec<CFile>) {
    for s in p {
        for _ in 0..s.len {
            if s.id == -1 {
                print!(".");
            } else {
                print!("{}", s.id);
            }
        }
    }
    println!();
}

fn intern_part2(fparsed: &mut Vec<CFile>) -> usize{
    let mut sum: usize = 0;
    for i in 0..fparsed.len() {
        let cc: CFile = fparsed[i];
        let mut cur: usize = fparsed.len() - 1;
        if cc.id == -1 {
            while cur > i {
                let dc: CFile = fparsed[cur];
                if dc.id == -1 {
                    cur -= 1;
                    continue;
                }
                if dc.len == cc.len {
                    fparsed[i] = fparsed[cur];
                    fparsed[cur] = CFile{ len : cc.len, id : -1};
                    break;
                } else if dc.len < cc.len {
                    let diff: usize = cc.len - dc.len;
                    fparsed[i] = fparsed[cur];
                    fparsed[cur] = CFile{ len : dc.len, id : -1};
                    fparsed.insert(i + 1, CFile { len: diff, id: -1 });
                    break;
                }
                cur -= 1;
            }
        }
    }

    // do some calc id * idx
    // 00 99 2 111 777 . 44 . 333 .... 5555 . 6666 ..... 8888..
    let mut counter: usize = 0;
    for i in 0..fparsed.len() {
        for _ in 0..fparsed[i].len {
            let mut cid = fparsed[i].id;
            if cid == -1 {
                cid = 0;
            };
            sum += counter * cid  as usize;
            counter += 1;
        }
    }
    return sum;
}
pub fn part1(fpath: &str) -> Option<usize> {
    let mut fparsed: Vec<String> = Vec::new();
    let buff: String = readf(fpath);
    let bufoch: Vec<char> = buff.chars().collect();
    let mut gid: i32 = 0;
    let mut dir;
    for i in 0..bufoch.len() {
        if i % 2 == 0 {
            dir = false;
        } else {
            dir = true;
        }
        let conv: u32 = bufoch[i].to_digit(10).unwrap_or(u32::max_value());
        for j in 0..conv {
            if i % 2 == 0 {
                fparsed.push(gid.to_string());
            } else {
                fparsed.push(".".to_string());
            }
        }
        if !dir {
            gid += 1;
        }
    }

    let mut sum:usize = 0;
    let mut counter: usize = 0;
    let mut bcur: usize = fparsed.len() - 1;
    for fcur in 0..fparsed.len() {
        if fcur >= bcur {
            break;
        }
        if fparsed[fcur] == "." {
            while fparsed[bcur] == "." {
                bcur -= 1;
            }
            fparsed[fcur] = fparsed[bcur].clone();
            fparsed[bcur] = ".".to_string();
        }

    }

    for i in 0..fparsed.len() {
        if fparsed[i].trim().parse::<usize>().is_err() {
            sum += counter * 0;
            continue;
        }
        sum += counter * fparsed[i].trim().parse::<usize>().unwrap();
        counter += 1;
    }
    return Some(sum);
}

pub fn part2(fpath: &str) -> Option<usize> {
    let mut fparsed: Vec<CFile> = Vec::new();
    let buff: String = readf(fpath);
    let bufoch: Vec<char> = buff.chars().collect();
    let mut gid: i32 = 0;
    for i in 0..bufoch.len() {
        if i % 2 == 0 {
            fparsed.push(CFile { len: bufoch[i].to_string().trim().parse().unwrap(), id: gid });
            gid += 1;
        } else {
            fparsed.push(CFile { len: bufoch[i].to_string().trim().parse().unwrap(), id: -1 });
        } 
    }

    let res = intern_part2(&mut fparsed);
    return Some(res);
}
