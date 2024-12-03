mod day1;
mod day2;
mod day3;
// use day1::*;
// use day2::*;
use day3::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: Not enought args!");
        std::process::exit(1);
    }

    // DAY 1
    /* let result = day1_p1(&args[1]);
    if result.is_some() {
        println!("part 1 : {}", result.unwrap());
    }
    let result = day1_p2("day1.txt");
    if result.is_some() {
        println!("part 2 : {}", result.unwrap());
    } */

    // DAY 2
    /* let result = day2_p1(&args[1]);
    if result.is_some() {
        println!("part 1 : {}", result.unwrap());
    }
    let result = day2_p2(&args[1]);
    if result.is_some() {
        println!("part 2 : {}", result.unwrap());
    } */

    // DAY 3
    let result = day3_p1(&args[1]);
    if result.is_some() {
        println!("part 1 : {}", result.unwrap());
    }
    let result = day3_p2(&args[1]);
    if result.is_some() {
        println!("part 2 : {}", result.unwrap());
    }
}
