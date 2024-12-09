mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day6r;
mod day7;
// use day6::*;
// use day6r::*;
use day7::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: Not enought args!");
        std::process::exit(1);
    }

    let result = part1(&args[1]);
    if result.is_some() {
        println!("part 1 : {}", result.unwrap());
    }
    let result = part2(&args[1]);
    if result.is_some() {
        println!("part 2 : {}", result.unwrap());
    }
}
