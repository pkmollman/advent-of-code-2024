use crate::tools::tools;

use std::path::PathBuf;

trait Report {
    fn is_in_bounds(&self) -> bool;
    fn is_unidirectional(&self) -> bool;
    fn lenient_iterations(&self) -> Vec<Vec<i32>>;
    fn is_safe(&self) -> bool;
    fn is_safe_lenient(&self) -> bool;
}

impl Report for Vec<i32> {
    fn is_in_bounds(&self) -> bool {
        for i in 1..=self.len() - 1 {
            let diff = (self[i - 1] - self[i]).abs();
            if diff < 1 || diff > 3 {
                return false;
            }
        }
        return true;
    }

    fn is_unidirectional(&self) -> bool {
        let dir = self[0] - self[1];
        for i in 2..=self.len() - 1 {
            let dir1 = self[i - 1] - self[i];
            if (dir < 0 && dir1 > 0) || (dir > 0 && dir1 < 0) {
                return false;
            }
        }
        return true;
    }

    fn lenient_iterations(&self) -> Vec<Vec<i32>> {
        let mut iterations = Vec::new();

        for i in 0..=self.len() - 1 {
            let mut self_copy = self.clone();
            self_copy.remove(i);
            iterations.push(self_copy);
        }

        return iterations;
    }

    fn is_safe(&self) -> bool {
        return self.is_in_bounds() && self.is_unidirectional();
    }

    fn is_safe_lenient(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for iteration in self.lenient_iterations() {
            if iteration.is_safe() {
                return true;
            }
        }

        return false;
    }
}

pub fn part1() {
    let lines = tools::get_file_lines(PathBuf::from("./input/day02-part1.txt"));
    let number_rows = tools::lines_as_number_rows::<i32>(&lines);

    let mut safe = 0;

    for row in number_rows {
        if row.is_safe() {
            safe += 1;
        }
    }

    println!("Day 2 Part 1: {}", safe)
}

pub fn part2() {
    let lines = tools::get_file_lines(PathBuf::from("./input/day02-part1.txt"));
    let number_rows = tools::lines_as_number_rows::<i32>(&lines);

    let mut safe = 0;

    for row in number_rows {
        if row.is_safe_lenient() {
            safe += 1;
        }
    }

    println!("Day 2 Part 2: {}", safe)
}
