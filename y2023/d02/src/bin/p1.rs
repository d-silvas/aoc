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

trait IsValid {
    fn is_valid(&self) -> bool;
}

impl IsValid for common::Draw {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

fn run(input_text: &str) -> io::Result<i32> {
    let mut total_sum: i32 = 0;

    for (current_index, line) in input_text.lines().enumerate() {
        let draws_vec: Vec<common::Draw> =
            common::draws_from_string(line, current_index as i32 + 1);
        if draws_vec.iter().all(|d| d.is_valid()) {
            total_sum += draws_vec.first().unwrap().game_id;
        }
    }

    Ok(total_sum)
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
