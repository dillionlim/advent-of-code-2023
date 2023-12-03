use log::{info, error};
use std::fs;

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

    let lines: Vec<String> = contents.lines().map(String::from).collect();
    lines
}

fn solveline_part_a(current_string: &String) -> i32 {
    let (mut startdigit, mut enddigit) = (-1, -1);
    for c in current_string.chars() {
        if c.is_numeric() == true {
            startdigit = c as i32 - '0' as i32;
            break;
        }
    }
    for c in (current_string.chars()).rev() {
        if c.is_numeric() == true {
            enddigit = c as i32 - '0' as i32;
            break;
        }
    }
    assert!(startdigit >= 0);
    assert!(enddigit >= 0);
    startdigit * 10 + enddigit
}

fn solveline_part_b(current_string: &String) -> i32 {
    let (mut startdigit, mut enddigit) = (-1, -1);
    let (mut startdigit_pos, mut enddigit_pos) = (i32::MAX, i32::MAX);
    let current_string_rev = current_string.chars().rev().collect::<String>();
    for (i, c) in current_string.chars().enumerate() {
        if c.is_numeric() == true {
            startdigit = c as i32 - '0' as i32;
            startdigit_pos = i as i32;
            break;
        }
    }
    for (i, c) in current_string_rev.chars().enumerate() {
        if c.is_numeric() == true {
            enddigit = c as i32 - '0' as i32;
            enddigit_pos = i as i32;
            break;
        }
    }
    let tomatch = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut tomatch_rev = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for number in tomatch_rev.iter_mut() {
        let reversed: String = number.chars().rev().collect();
        *number = Box::leak(reversed.into_boxed_str());
    }

    for (i, number) in tomatch.iter().enumerate() {
        match current_string.find(number) {
            Some(pos) if (pos as i32) < startdigit_pos => {
                startdigit = (i + 1) as i32;
                startdigit_pos = pos as i32;
            }
            _ => {}
        }
    }

    for (i, number) in tomatch_rev.iter().enumerate() {
        match current_string_rev.find(number) {
            Some(pos) if (pos as i32) < enddigit_pos => {
                enddigit = (i + 1) as i32;
                enddigit_pos = pos as i32;
            }
            _ => {}
        }
    }

    assert!(startdigit >= 0);
    assert!(enddigit >= 0);
    startdigit * 10 + enddigit
}

fn main() {
    let file_path = "./input/puzzle-1.txt";
    let input_lines = parse_input(file_path);
    let mut ans = 0;
    let mut ans2 = 0;
    for string in input_lines.iter() {
        ans += solveline_part_a(&string);
        ans2 += solveline_part_b(&string);
    }
    println!("Answer (Part a): {}", ans);
    println!("Answer (Part b): {}", ans2);
}