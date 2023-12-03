use log::{info, error};
use std::{cmp, fs};

struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

const MAX_MARBLE_CNT: Game = Game::new(12, 13, 14);

impl Game {
    const fn new(red_cnt: u32, green_cnt: u32, blue_cnt: u32) -> Game {
        Game { red: red_cnt, green: green_cnt, blue: blue_cnt }
    }
    fn check_validity( &self ) -> bool {
        MAX_MARBLE_CNT.red >= self.red && MAX_MARBLE_CNT.green >= self.green && MAX_MARBLE_CNT.blue >= self.blue
    }
}

fn parse_game(segment: &str) -> Game {
    let mut game = Game::new(0, 0, 0);

    let color_count_pairs: Vec<&str> = segment.split(',').collect();

    for pair in color_count_pairs {
        let parts: Vec<&str> = pair.trim().split_whitespace().collect();
        if let Some(marble_count_str) = parts.get(0) {
            if let Ok(marble_count) = marble_count_str.parse::<u32>() {
                if let Some(colour) = parts.get(1) {
                    match colour.to_lowercase().as_str() {
                        "red" => game.red += marble_count,
                        "green" => game.green += marble_count,
                        "blue" => game.blue += marble_count,
                        _ => {}
                    }
                }
            }
        }
    }

    game
}

fn parse_input(file_path: &str) -> Vec<Vec<Game>> {
    env_logger::init();
    info!("Reading input from {}.", file_path);

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            error!("Error reading the file: {}", err);
            "Error".to_string()
        }
    };

    let games: Vec<Vec<Game>> = contents.lines().map(|line| {
        line.split(':')
            .nth(1)
            .expect("Malformed input")
            .split(';')
            .map(|segment| parse_game(segment.trim()))
            .collect()
    }).collect();

    games
}

fn solveline_part_a(current_game_set: &Vec<Game>, index: u32) -> u32 {
    for game in current_game_set.iter() {
        if game.check_validity() == false {
            return 0;
        }
    }
    index+1
}

fn solveline_part_b(current_game_set: &Vec<Game>) -> u32 {
    let mut min_marble_cnt = Game::new(0, 0, 0);
    for game in current_game_set.iter() {
        min_marble_cnt.red = cmp::max(min_marble_cnt.red, game.red);
        min_marble_cnt.blue = cmp::max(min_marble_cnt.blue, game.blue);
        min_marble_cnt.green = cmp::max(min_marble_cnt.green, game.green);
    }
    min_marble_cnt.red * min_marble_cnt.blue * min_marble_cnt.green
}

fn main() {
    let file_path = "./input/puzzle-2.txt";
    let input_games = parse_input(file_path);
    let mut ans = 0u32;
    let mut ans2 = 0u32;
    for (index, game_set) in input_games.iter().enumerate() {
        ans += solveline_part_a(&game_set, index as u32);
        ans2 += solveline_part_b(&game_set);
    }
    println!("Answer (Part a): {}", ans);
    println!("Answer (Part b): {}", ans2);
}