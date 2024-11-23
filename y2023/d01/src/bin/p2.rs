use core::panic;
use regex::Regex;
use std::io;
use std::time::{Duration, Instant};

fn main() {
    let input_text: &str = include_str!("input/input.txt");

    let start_bad: Instant = Instant::now();
    let result_bad: i64 = run_bad(input_text).expect("Should have worked");
    let duration_bad: Duration = start_bad.elapsed();

    println!("Bad result: {}", result_bad);
    println!("Bad execution time: {:.2?}", duration_bad);

    println!();

    let start: Instant = Instant::now();
    let result: i64 = run(input_text).expect("Should have worked");
    let duration: Duration = start.elapsed();

    println!("Result: {}", result);
    println!("Execution time: {:.2?}", duration);
}

/**
 * THIS DOES NOT WORK
 *
 * See https://stackoverflow.com/questions/77587365/overlapping-matches-in-regex-rust-regex-engine
 */
fn run_bad(input_text: &str) -> io::Result<i64> {
    let re = Regex::new(r#"(\d|one|two|three|four|five|six|seven|eight|nine){1}"#).unwrap();
    let mut total_sum: i64 = 0;

    for line in input_text.lines() {
        let matches: Vec<String> = find_strings(&re, line);
        let first_calibration_digit: &str = match_calibration_values(&matches.first().unwrap());
        let last_calibration_digit: &str = match_calibration_values(&matches.last().unwrap());
        // See https://stackoverflow.com/questions/30154541/how-do-i-concatenate-strings
        let calibration_val_str: String =
            first_calibration_digit.to_owned() + last_calibration_digit;
        total_sum += calibration_val_str.parse::<i64>().unwrap();
    }

    Ok(total_sum)
}

fn find_strings(re: &Regex, s: &str) -> Vec<String> {
    re.find_iter(s)
        .filter_map(|matches| matches.as_str().parse().ok())
        .collect()
}

fn match_calibration_values(str_to_match: &String) -> &str {
    match str_to_match.as_str() {
        "1" | "one" => "1",
        "2" | "two" => "2",
        "3" | "three" => "3",
        "4" | "four" => "4",
        "5" | "five" => "5",
        "6" | "six" => "6",
        "7" | "seven" => "7",
        "8" | "eight" => "8",
        "9" | "nine" => "9",
        _ => panic!("Should have matched"),
    }
}

fn run(input_text: &str) -> io::Result<i64> {
    let mut total_sum: i64 = 0;

    for line in input_text.lines() {
        let (first_match, last_match): (Option<&str>, Option<&str>) = match_nums(line);
        let fm = first_match.unwrap().to_owned();
        let lm: String;
        if last_match.is_none() {
            lm = fm.clone();
        } else {
            lm = last_match.unwrap().to_owned();
        }
        let first_calibration_digit = match_calibration_values(&fm);
        let last_calibration_digit = match_calibration_values(&lm);
        let calibration_val_str: String =
            first_calibration_digit.to_owned() + last_calibration_digit;
        total_sum += calibration_val_str.parse::<i64>().unwrap();
    }

    Ok(total_sum)
}

/**
 * Inspiration: https://www.reddit.com/r/adventofcode/comments/1883ibu/comment/kfl143d
 */
fn match_nums(line: &str) -> (Option<&str>, Option<&str>) {
    use std::mem::swap;
    const NUMBERS: &str = "|one|two|three|four|five|six|seven|eight|nine|1|2|3|4|5|6|7|8|9|";
    const N: usize = NUMBERS.len();
    let bnumbers = NUMBERS.as_bytes();
    let mut dp1 = [usize::MAX; N];
    let mut dp2 = [usize::MAX; N];
    let mut first = None;
    let mut last = None;

    for b1 in line.bytes().chain([b'#']) {
        for (j, b2) in (1..).zip(NUMBERS.bytes()) {
            if b2 == b'|' && dp1[j - 1] != usize::MAX {
                let k = dp1[j - 1];
                if first.is_none() {
                    first = Some(&NUMBERS[k..j - 1]);
                } else {
                    last = Some(&NUMBERS[k..j - 1]);
                }
            } else if b1 == b2 {
                if bnumbers[j - 2] == b'|' {
                    dp2[j] = j - 1;
                } else {
                    dp2[j] = dp1[j - 1];
                }
            }
        }
        swap(&mut dp1, &mut dp2);
        dp2.fill(usize::MAX);
    }
    (first, last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let test_text: &str = include_str!("input/test2.txt");
        assert_eq!(281i64, run(test_text).unwrap());
    }

    #[test]
    fn test_run_bad() {
        let test_text: &str = include_str!("input/test2.txt");
        assert_eq!(281i64, run_bad(test_text).unwrap());
    }
}
