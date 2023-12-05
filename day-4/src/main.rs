use log::{info, error};
use std::{fs, collections::HashSet};

fn parse_input(file_path: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    env_logger::init();
    info!("Reading input from {}.", file_path);

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            error!("Error reading the file: {}", err);
            "Error".to_string()
        }
    };

    let result: Vec<(Vec<u32>, Vec<u32>)> = contents.lines().map(|line| {
        if let Some(cards) = line.split(':').nth(1){
            let cards_split: Vec<&str> = cards.split('|').collect();

            let winning: Vec<u32> = cards_split[0]
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap_or(0))
                .collect();

            let holding: Vec<u32> = cards_split[1]
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap_or(0))
                .collect();

            (winning, holding)
        }
        else {
            (Vec::new(), Vec::new())
        }
    })
    .collect();

    result
}

fn solve_part_a(line: &(Vec<u32>, Vec<u32>)) -> u32 {
    let winning: HashSet<u32> = HashSet::from_iter(line.0.iter().cloned());
    let cnt: u32 = line.1.iter().fold(0u32, |sum, &val| sum + winning.contains(&val).then(||1).unwrap_or(0));
    if cnt == 0 {
        return 0;
    }
    1 << (cnt - 1)
}

fn solve_part_b(lines: &Vec<(Vec<u32>, Vec<u32>)>) -> u32 {
    let card_cnt: Vec<u32> = lines.iter().map(|line| {
        let winning: HashSet<u32> = HashSet::from_iter(line.0.iter().cloned());
        let cnt: u32 = line.1.iter().fold(0u32, |sum, &val| sum + winning.contains(&val).then(||1).unwrap_or(0));
        cnt
    }).collect();

    let mut card_won: Vec<u32> = vec![1; lines.len()];

    let n = card_won.len();

    (0..n).for_each(|i| {
        let end_index = std::cmp::min(n, i + card_cnt[i] as usize + 1);
        (i + 1..end_index).for_each(|j| {
            card_won[j] += card_won[i];
        });
    });
    
    let ans: u32 = card_won.iter().sum();
    
    ans
}

fn main() {
    let file_path = "./input/puzzle-4.txt";
    let input_lines = parse_input(file_path);
    let mut ans = 0;
    for line in &input_lines {
        ans += solve_part_a(&line);
    }
    let ans2 = solve_part_b(&input_lines);
    println!("Answer (Part a): {}", ans);
    println!("Answer (Part b): {}", ans2);
}