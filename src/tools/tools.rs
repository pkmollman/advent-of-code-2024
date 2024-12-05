use std::{path::PathBuf, str::FromStr};

pub fn get_file_lines(path: PathBuf) -> Vec<String> {
    let mut lines = Vec::new();

    let file_res = std::fs::read_to_string(&path);

    match file_res {
        Ok(file_content) => file_content.lines().map(|l| lines.push(l.into())).collect(),
        Err(error) => {
            println!("unable to read file {:?}: {}", &path, error)
        }
    }

    return lines;
}

pub fn lines_as_number_columns<T: FromStr>(lines: &Vec<String>, place: usize) -> Vec<T> {
    let mut values = Vec::new();

    for line in lines {
        match line.split_whitespace().collect::<Vec<&str>>()[place].parse::<T>() {
            Ok(value) => values.push(value),
            Err(_) => println!("unable to parse {} value of \"{}\"", place, line),
        }
    }

    return values;
}

pub fn lines_as_number_rows<T: FromStr>(lines: &Vec<String>) -> Vec<Vec<T>> {
    let mut lines_with_values: Vec<Vec<T>> = Vec::new();

    for line in lines {
        let mut line_values = Vec::new();
        for raw_value in line.split_whitespace() {
            match raw_value.parse::<T>() {
                Ok(value) => line_values.push(value),
                Err(_) => {}
            }
        }

        lines_with_values.push(line_values);
    }

    return lines_with_values;
}

pub fn lines_as_columns(lines: &Vec<String>) -> Vec<Vec<char>> {
    let mut columns: Vec<Vec<char>> = Vec::new();

    for line in lines {
        if columns.len() == 0 {
            for _ in line.chars() {
                columns.push(Vec::new());
            }
        }
        for (i, char) in line.char_indices() {
            columns[i].push(char);
        }
    }

    return columns;
}
