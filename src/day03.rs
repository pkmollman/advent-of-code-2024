use regex::Regex;
use std::path::PathBuf;

pub fn part1() {
    let regex = Regex::new(r"(?<multi>mul\((?<v1>\d{1,3}),(?<v2>\d{1,3})\))").unwrap();

    let file_res = std::fs::read_to_string(PathBuf::from("./input/day03-part1.txt")).unwrap();

    let mut total = 0;
    regex
        .captures_iter(file_res.as_str())
        .map(|f| total += f["v1"].parse::<i32>().unwrap() * f["v2"].parse::<i32>().unwrap())
        .for_each(drop);
    println!("Day 3 Part 1: {}", total)
}

pub fn part2() {
    let regex =
        Regex::new(r"(?<operation>mul|do|don't)\(((?<v1>\d{1,3}),(?<v2>\d{1,3}))?\)").unwrap();

    let file_res = std::fs::read_to_string(PathBuf::from("./input/day03-part1.txt")).unwrap();

    let mut total = 0;
    let mut enabled = true;
    regex
        .captures_iter(file_res.as_str())
        .map(|f| match &f["operation"] {
            "do" => enabled = true,
            "don't" => enabled = false,
            "mul" => {
                if enabled {
                    total += f["v1"].parse::<i32>().unwrap() * f["v2"].parse::<i32>().unwrap();
                }
            }
            _ => {}
        })
        .for_each(drop);
    println!("Day 3 Part 2: {}", total)
}
