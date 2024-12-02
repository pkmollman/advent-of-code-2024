use std::path::PathBuf;

use crate::tools::tools;

pub fn part1() {
    let lines = tools::get_file_lines(PathBuf::from("./input/day01-part1.txt"));

    let mut col_1: Vec<i32> = tools::lines_as_number_columns(&lines, 0);
    let mut col_2: Vec<i32> = tools::lines_as_number_columns(&lines, 1);

    col_1.sort();
    col_2.sort();

    let mut distance = 0;

    for i in 0..col_1.len() {
        let mut packed_values = Vec::from([col_1[i], col_2[i]]);
        packed_values.sort();
        distance += packed_values[1] - packed_values[0]
    }

    println!("Day 1 Part 1: {}", distance)
}

pub fn part2() {
    let lines = tools::get_file_lines(PathBuf::from("./input/day01-part1.txt"));

    let mut col_1: Vec<i32> = tools::lines_as_number_columns(&lines, 0);
    let mut col_2: Vec<i32> = tools::lines_as_number_columns(&lines, 1);

    col_1.sort();
    col_2.sort();

    let mut sim_score = 0;

    for i in 0..col_1.len() {
        sim_score += col_1[i] * col_2.iter().filter(|v| v == &&col_1[i]).count() as i32;
    }

    println!("Day 1 Part 2: {}", sim_score)
}
