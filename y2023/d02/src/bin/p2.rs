use regex::Regex;
use std::io;
use std::time::{Duration, Instant};

fn main() {
    let input_text = include_str!("input/input.txt");

    let start: Instant = Instant::now();
    let result: i32 = run(input_text).expect("Should have worked");
    let duration: Duration = start.elapsed();

    println!("Result: {}", result);
    println!("Execution time: {:.2?}", duration);
}

#[derive(Debug)]
struct Draw {
    game_id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    draws: Vec<Draw>,
}

trait CubeCount {
    fn req_blue(&self) -> i32;
    fn req_red(&self) -> i32;
    fn req_green(&self) -> i32;
    fn power(&self) -> i32;
}

impl CubeCount for Game {
    // This is quite inefficient because we are going to loop over the list of draws
    // 3 times instead of just 1. But the code is simple.
    fn req_blue(&self) -> i32 {
        self.draws.iter().map(|d| d.blue).max().unwrap_or(0)
    }

    fn req_red(&self) -> i32 {
        self.draws.iter().map(|d| d.red).max().unwrap_or(0)
    }

    fn req_green(&self) -> i32 {
        self.draws.iter().map(|d| d.green).max().unwrap_or(0)
    }

    fn power(&self) -> i32 {
        self.req_blue() * self.req_red() * self.req_green()
    }
}

fn run(input_text: &str) -> io::Result<i32> {
    let mut games: Vec<Game> = Vec::new();
    for (current_index, line) in input_text.lines().enumerate() {
        let game_id = current_index as i32 + 1;
        let game = Game {
            draws: draws_from_string(line, game_id),
        };
        games.push(game);
    }
    Ok(games.iter().map(|g| g.power()).sum())
}

fn draws_from_string(draws_str: &str, game_id: i32) -> Vec<Draw> {
    let index_regex: Regex = Regex::new(r#"Game \d+:"#).unwrap();
    let blue_regex = Regex::new(r#"(?<blue>\d+) blue"#).unwrap();
    let red_regex = Regex::new(r#"(?<red>\d+) red"#).unwrap();
    let green_regex = Regex::new(r#"(?<green>\d+) green"#).unwrap();
    // We could have used the row index that comes from the file,
    // but we are using the loop index instead
    let str = index_regex.replace_all(draws_str, "");
    let draws_str = str.split(';');
    let mut draws_vec: Vec<Draw> = Vec::new();

    for g in draws_str {
        let blue = match blue_regex.captures(g) {
            Some(caps) => caps["blue"].parse::<i32>().unwrap(),
            None => 0_i32,
        };
        let red = match red_regex.captures(g) {
            Some(caps) => caps["red"].parse::<i32>().unwrap(),
            None => 0_i32,
        };
        let green = match green_regex.captures(g) {
            Some(caps) => caps["green"].parse::<i32>().unwrap(),
            None => 0_i32,
        };
        let draw = Draw {
            game_id,
            blue,
            red,
            green,
        };
        draws_vec.push(draw);
    }
    draws_vec
}
