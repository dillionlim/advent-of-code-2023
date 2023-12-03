use log::{info, error};
use std::{fs, cmp, collections::HashMap};
use regex::Regex;

fn parse_input(file_path: &str) -> Vec<String> {
    env_logger::init();
    info!("Reading input from {}.", file_path);

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            error!("Error reading the file: {}", err);
            "Error".to_string()
        }
    };

    let grid: Vec<String> = contents.lines().map(String::from).collect();

    grid
}

fn verify_symbol(grid: &Vec<String>, row: usize, col: usize, substr_len: usize) -> bool {
    let regex = Regex::new(r#"[^a-zA-Z0-9.]"#).expect("Invalid regex pattern");

    if row > 0 {
        if regex.is_match(&grid[row - 1][cmp::max(col as i32 - 1, 0) as usize .. cmp::min((col + substr_len + 1) as u32, (grid[0].len()) as u32) as usize]) {
            return true;
        }
    }
    if row + 1 < grid.len() {
        if regex.is_match(&grid[row + 1][cmp::max(col as i32 - 1, 0) as usize .. cmp::min((col + substr_len + 1) as u32, (grid[0].len()) as u32) as usize]) {
            return true;
        }
    }
    if col > 0 {
        if regex.is_match(&grid[row][col - 1 .. col]) {
            return true;
        }
    }
    if col + substr_len < grid[0].len() {
        if regex.is_match(&grid[row][col + substr_len .. col + substr_len + 1]) {
            return true;
        }
    }
    false
}

fn solve_part_a(grid: &Vec<String>) -> u32 {
    let regex = Regex::new(r"\d+").expect("Invalid regex pattern");

    let ans: u32 = grid.iter().enumerate().map(|(row, line)| {
            regex.find_iter(line)
                .map(|m| (m.start(), m.end() - m.start()))
                .collect::<Vec<(usize, usize)>>()
                .iter()
                .filter(|&&(index, length)| verify_symbol(&grid, row, index, length))
                .fold(0, |counter, &(index, length)| {
                    counter + grid[row][index..(index + length)].parse::<u32>().unwrap_or(0)
                })
        })
        .sum();

    ans
}

fn find_asterisk(grid: &Vec<String>, row: usize, col: usize, substr_len: usize) -> Option<(usize, usize)> {
    let regex = Regex::new(r#"\*"#).expect("Invalid regex pattern");

    if row > 0 {
        let mat = regex.find(&grid[row - 1][cmp::max(col as i32 - 1, 0) as usize .. cmp::min((col + substr_len + 1) as u32, (grid[0].len()) as u32) as usize]);
        if let Some(col_range) = mat {
            return Some((row - 1, cmp::max(col as i32 - 1, 0) as usize + col_range.start()));
        }
    }
    if row + 1 < grid.len() {
        let mat = regex.find(&grid[row + 1][cmp::max(col as i32 - 1, 0) as usize .. cmp::min((col + substr_len + 1) as u32, (grid[0].len()) as u32) as usize]);
        if let Some(col_range) = mat {
            return Some((row + 1, cmp::max(col as i32 - 1, 0) as usize + col_range.start()));
        }
    }
    if col > 0 {
        let mat = regex.find(&grid[row][col - 1 .. col]);
        if let Some(col_range) = mat {
            return Some((row, col - 1 + col_range.start()));
        }
    }
    if col + substr_len < grid[0].len() {
        let mat = regex.find(&grid[row][col + substr_len .. col + substr_len + 1]);
        if let Some(col_range) = mat {
            return Some((row, col + substr_len + col_range.start()));
        }
    }
    None
}

fn solve_part_b(grid: &Vec<String>) -> u32 {
    let regex = Regex::new(r"\d+").expect("Invalid regex pattern");
    
    let gear_list: Vec<(u32, usize, usize)> = grid.iter().enumerate().flat_map(|(row, line)| {
        regex.find_iter(line).filter_map(move |m| {
            let index = m.start();
            let length = m.end() - index;

            find_asterisk(grid, row, index as usize, length).map(|coords| (
                grid[row][index .. (index + length)].parse::<u32>().unwrap_or(0),
                coords.0,
                coords.1,
            ))
        })
    }).collect();

    let mut count_map: HashMap<(usize, usize), HashMap<u32, usize>> = HashMap::new();

    for (value, row, col) in &gear_list {
        let entry = count_map.entry((*row, *col)).or_insert_with(HashMap::new);
        *entry.entry(*value).or_insert(0) += 1;
    }

    let result: u32 = count_map
        .into_iter()
        .filter_map(|((_row, _col), value_counts)| {
            if value_counts.len() == 2 {
                let values: Vec<u32> = value_counts.keys().cloned().collect();
                Some(values[0] * values[1])
            } 
            else {
                None
            }
        })
        .sum();

    result
}

fn main() {
    let file_path = "./input/puzzle-3.txt";
    let input_grid = parse_input(file_path);
    let ans = solve_part_a(&input_grid);
    let ans2 = solve_part_b(&input_grid);
    println!("Answer (Part a): {}", ans);
    println!("Answer (Part b): {}", ans2);
}