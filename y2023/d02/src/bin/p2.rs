use std::io;
use std::time::{Duration, Instant};
mod common;

fn main() {
    let input_text = include_str!("input/input.txt");

    let start: Instant = Instant::now();
    let result: i32 = run(input_text).expect("Should have worked");
    let duration: Duration = start.elapsed();

    println!("Result: {}", result);
    println!("Execution time: {:.2?}", duration);
}

struct Game {
    draws: Vec<common::Draw>,
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
            draws: common::draws_from_string(line, game_id),
        };
        games.push(game);
    }
    Ok(games.iter().map(|g| g.power()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let test_text: &str = include_str!("input/test.txt");
        assert_eq!(2286i32, run(test_text).unwrap());
    }
}
