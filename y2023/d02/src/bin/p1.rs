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

trait IsValid {
    fn is_valid(&self) -> bool;
}

#[derive(Debug)]
struct Draw {
    game_id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

impl IsValid for Draw {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn run(input_text: &str) -> io::Result<i32> {
    let mut total_sum: i32 = 0;

    for (current_index, line) in input_text.lines().enumerate() {
        let draws_vec: Vec<Draw> = draws_from_string(line, current_index as i32 + 1);
        if draws_vec.iter().all(|d| d.is_valid()) {
            total_sum += draws_vec.first().unwrap().game_id;
        }
    }

    Ok(total_sum)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let test_text: &str = include_str!("input/test.txt");
        assert_eq!(8i32, run(test_text).unwrap());
    }
}
