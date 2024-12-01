use regex::Regex;
use std::io;
use std::time::{Duration, Instant};

fn main() {
    let input_text: &str = include_str!("input/input.txt");

    let start: Instant = Instant::now();
    let result: i32 = run(input_text).expect("Should have worked");
    let duration: Duration = start.elapsed();

    println!("Result: {}", result);
    println!("Execution time: {:.2?}", duration);
}

fn run(input_text: &str) -> io::Result<i32> {
    let first_num_regex = Regex::new(r#"^\d+"#).unwrap();
    let last_num_regex = Regex::new(r#"\d+$"#).unwrap();
    let mut all_first_nums: Vec<i32> = vec![];
    let mut all_last_nums: Vec<i32> = vec![];

    for line in input_text.lines() {
        let first_num = first_num_regex
            .find(line)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        let last_num = last_num_regex
            .find(line)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap();
        all_first_nums.push(first_num);
        all_last_nums.push(last_num);
    }

    all_first_nums.sort();
    all_last_nums.sort();

    assert_eq!(all_first_nums.len(), all_last_nums.len());

    let mut differences: i32 = 0;
    for index in 0..all_first_nums.len() {
        differences += (all_last_nums[index] - all_first_nums[index]).abs();
    }

    Ok(differences)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let test_text: &str = include_str!("input/test.txt");
        assert_eq!(11i32, run(test_text).unwrap());
    }
}
