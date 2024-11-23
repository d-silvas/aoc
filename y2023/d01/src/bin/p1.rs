use regex::Regex;
use std::io;
use std::time::{Duration, Instant};

fn main() {
    let start: Instant = Instant::now();

    let input_text: &str = include_str!("input/input.txt");
    let result: i64 = run(input_text).expect("Should have worked");
    println!("Result: {}", result);

    let duration: Duration = start.elapsed();
    println!("Execution time: {:.2?}", duration);
}

fn run(input_text: &str) -> io::Result<i64> {
    let re = Regex::new(r#"\d{1}"#).unwrap();
    let mut total_sum: i64 = 0;

    for line in input_text.lines() {
        let numbers: Vec<i64> = find_numbers(&re, line);
        let first: &i64 = numbers.first().unwrap();
        let last: &i64 = numbers.last().unwrap();

        total_sum += 10 * first + last;
    }

    Ok(total_sum)
}

/**
 * Finds single digits in a string
 *
 * See https://stackoverflow.com/questions/58010114/capture-all-regex-matches-into-a-vector
 */
fn find_numbers(re: &Regex, s: &str) -> Vec<i64> {
    // Iterate over all matches
    re.find_iter(s)
        // Try to parse the string matches as i64 (inferred from fn type signature)
        // and filter out the matches that can't be parsed (e.g. if there are too many digits to store in an i64).
        .filter_map(|digits| digits.as_str().parse().ok())
        // Collect the results in to a Vec<i64> (inferred from fn type signature)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let test_text: &str = include_str!("input/test1.txt");
        assert_eq!(142i64, run(test_text).unwrap());
    }
}
