use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::time::{Duration, Instant};

fn main() {
    let input_text: &str = include_str!("input/input.txt");

    let start: Instant = Instant::now();
    let result: u32 = run(input_text).expect("Should have worked");
    let duration: Duration = start.elapsed();

    println!("Result: {}", result);
    println!("Execution time: {:.2?}", duration);
}

fn run(input_text: &str) -> io::Result<u32> {
    let first_num_regex = Regex::new(r#"^\d+"#).unwrap();
    let last_num_regex = Regex::new(r#"\d+$"#).unwrap();
    let mut all_first_nums: Vec<u32> = vec![];
    let mut all_last_nums: Vec<u32> = vec![];

    for line in input_text.lines() {
        let first_num = first_num_regex
            .find(line)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let last_num = last_num_regex
            .find(line)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        all_first_nums.push(first_num);
        all_last_nums.push(last_num);
    }

    all_first_nums.sort();
    all_last_nums.sort();

    assert_eq!(all_first_nums.len(), all_last_nums.len());

    let mut similarity_score_addends: HashMap<u32, u32> = HashMap::new();
    let mut similarity_score: u32 = 0;
    for num in all_first_nums {
        if !similarity_score_addends.contains_key(&num) {
            let times_num_appears_in_last_nums_list: u32 = all_last_nums
                .iter()
                .filter(|last_num| **last_num == num)
                .collect::<Vec<&u32>>()
                .len() as u32;
            similarity_score_addends.insert(num, num * times_num_appears_in_last_nums_list);
        }
        similarity_score += similarity_score_addends.get(&num).unwrap();
    }

    Ok(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let test_text: &str = include_str!("input/test.txt");
        assert_eq!(31u32, run(test_text).unwrap());
    }
}
