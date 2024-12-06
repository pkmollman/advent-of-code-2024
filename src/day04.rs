use crate::tools::tools;
use std::ops::{Add, Mul};
use std::path::PathBuf;

#[derive(PartialEq, Debug, Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }

    fn is_in_bounds(&self) -> bool {
        if self.x < 0 || self.y < 0 {
            return false;
        }
        return true;
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<i32> for Vec2 {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

const U: Vec2 = Vec2 { x: 0, y: -1 };
const D: Vec2 = Vec2 { x: 0, y: 1 };
const L: Vec2 = Vec2 { x: -1, y: 0 };
const R: Vec2 = Vec2 { x: 1, y: 0 };
const UR: Vec2 = Vec2 { x: 1, y: -1 };
const UL: Vec2 = Vec2 { x: -1, y: -1 };
const DR: Vec2 = Vec2 { x: 1, y: 1 };
const DL: Vec2 = Vec2 { x: -1, y: 1 };

const SCAN_DIRS: [Vec2; 8] = [U, D, L, R, UR, UL, DR, DL];
const X_SCAN_DIRS1: [Vec2; 4] = [U, D, L, R];
const X_SCAN_DIRS2: [Vec2; 4] = [UR, UL, DR, DL];

trait WordPuzzle {
    fn sample(&self, pos: &Vec2) -> Option<char>;
    fn count_occurences(&self, word: &str) -> i32;
    fn count_xmas(&self) -> i32;
}

impl WordPuzzle for Vec<Vec<char>> {
    fn sample(&self, pos: &Vec2) -> Option<char> {
        if pos.is_in_bounds() && pos.x < self.len() as i32 && pos.y < self[0].len() as i32 {
            return Some(self[pos.x as usize][pos.y as usize]);
        }
        return None;
    }

    fn count_occurences(&self, word: &str) -> i32 {
        let mut total = 0;
        let word_chars: Vec<char> = word.chars().collect();
        let word_len = word_chars.len() as i32;

        for y_i in 0..self[0].len() {
            for x_i in 0..self.len() {
                let cpos = Vec2::new(x_i as i32, y_i as i32);
                let cchar = self.sample(&cpos).unwrap();
                if &cchar == word_chars.first().unwrap() {
                    for target_dir in SCAN_DIRS {
                        let last_char_pos = target_dir * (word_len - 1) + cpos;
                        match self.sample(&last_char_pos) {
                            Some(last_char) => {
                                if &last_char == word_chars.last().unwrap() {
                                    let mut valid_word = true;
                                    for char_dis in 1..word_len - 1 {
                                        let target_pos = target_dir * char_dis + cpos;
                                        if &self.sample(&target_pos).unwrap()
                                            != &word_chars[char_dis as usize]
                                        {
                                            valid_word = false;
                                            break;
                                        }
                                    }
                                    if valid_word {
                                        total += 1;
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                }
            }
        }

        return total;
    }

    fn count_xmas(&self) -> i32 {
        let mut total = 0;
        let word_chars: Vec<char> = "MAS".chars().collect();
        let word_len = word_chars.len() as i32;

        // only odds, because fml
        if word_len % 2 != 1 {
            return total;
        }

        let arm_len = (word_len - 1) / 2;
        let center_char = word_chars[arm_len as usize];

        for y_i in 1..self[0].len() - 1 {
            for x_i in 1..self.len() - 1 {
                let cpos = Vec2::new(x_i as i32, y_i as i32);
                let cchar = self.sample(&cpos).unwrap();
                if &cchar == &center_char {
                    let sul = self.sample(&(UL + cpos)).unwrap();
                    let sdr = self.sample(&(DR + cpos)).unwrap();
                    let sur = self.sample(&(UR + cpos)).unwrap();
                    let sdl = self.sample(&(DL + cpos)).unwrap();

                    let mut matches1 = true;
                    let samples1 = [sul, sdr, sur, sdl];

                    for sample in samples1 {
                        if &sample != word_chars.first().unwrap()
                            && &sample != word_chars.last().unwrap()
                        {
                            matches1 = false;
                        }
                    }

                    if matches1 && sul != sdr && sur != sdl {
                        total += 1;
                    }
                }
            }
        }

        return total;
    }
}

pub fn part1() {
    let lines = tools::get_file_lines(PathBuf::from("./input/day04-part1.txt"));
    let columns = tools::lines_as_columns(&lines);

    println!("Day 4 Part 1: {}", columns.count_occurences("XMAS"))
}

pub fn part2() {
    let lines = tools::get_file_lines(PathBuf::from("./input/day04-part1.txt"));
    let columns = tools::lines_as_columns(&lines);

    println!("Day 4 Part 2: {}", columns.count_xmas())
}
