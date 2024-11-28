use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::time::{Duration, Instant};

/**
 * Some changes in approach
 *  1. We are only going to consider "part numbers" the numbers that are adjacent to * symbols. We
 *      don't care if a number is adjacent to symbols like ? or $.
 */

fn main() {
    let input_text = include_str!("input/input.txt");

    let start: Instant = Instant::now();
    let result: u32 = run(input_text).expect("Should have worked");
    let duration: Duration = start.elapsed();

    println!("Result: {}", result);
    println!("Execution time: {:.2?}", duration);
}

// Hash and Eq are needed for Position to be a key of a HashMap
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
struct Position {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct NumberDigit {
    digit: u8,
    position: Position,
    adjacent_asterisk_positions: Vec<Position>,
}

impl NumberDigit {
    pub fn new(position: Position, all_chars: &Vec<Vec<char>>) -> Self {
        let row: usize = position.row;
        let col: usize = position.col;
        let curr_char: char = all_chars[row][col];
        let digit: u8 = Self::get_digit(curr_char);
        let adjacent_asterisk_positions: Vec<Position> =
            Self::get_adjacent_asterisk_positions(&all_chars, &position);
        NumberDigit {
            digit,
            position,
            adjacent_asterisk_positions,
        }
    }

    fn get_digit(char: char) -> u8 {
        match char {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            _ => panic!("You should have checked that there was a digit at this position! There was a {} instead!", char),
        }
    }

    fn get_adjacent_asterisk_positions(
        all_chars: &Vec<Vec<char>>,
        self_position: &Position,
    ) -> Vec<Position> {
        let last_row: usize = all_chars.len() - 1;
        let last_col: usize = all_chars[0].len() - 1;
        let curr_row: usize = self_position.row;
        let curr_col: usize = self_position.col;
        let mut positions_to_check: Vec<Position> = Vec::new();
        if curr_row == 0 {
            if curr_col == 0 {
                positions_to_check.push(Position { row: 0, col: 1 });
                positions_to_check.push(Position { row: 1, col: 0 });
                positions_to_check.push(Position { row: 1, col: 1 });
            } else if curr_col == last_col {
                positions_to_check.push(Position {
                    row: 0,
                    col: last_col - 1,
                });
                positions_to_check.push(Position {
                    row: 1,
                    col: last_col,
                });
                positions_to_check.push(Position {
                    row: 1,
                    col: last_col - 1,
                });
            } else {
                positions_to_check.push(Position {
                    row: 0,
                    col: curr_col - 1,
                });
                positions_to_check.push(Position {
                    row: 0,
                    col: curr_col + 1,
                });
                positions_to_check.push(Position {
                    row: 1,
                    col: curr_col - 1,
                });
                positions_to_check.push(Position {
                    row: 1,
                    col: curr_col,
                });
                positions_to_check.push(Position {
                    row: 1,
                    col: curr_col + 1,
                });
            }
        } else if curr_row == last_row {
            if curr_col == 0 {
                positions_to_check.push(Position {
                    row: last_row - 1,
                    col: 0,
                });
                positions_to_check.push(Position {
                    row: last_row - 1,
                    col: 1,
                });
                positions_to_check.push(Position {
                    row: last_row,
                    col: 1,
                });
            } else if curr_col == last_col {
                positions_to_check.push(Position {
                    row: last_row - 1,
                    col: last_col,
                });
                positions_to_check.push(Position {
                    row: last_row - 1,
                    col: last_col - 1,
                });
                positions_to_check.push(Position {
                    row: last_row,
                    col: last_col - 1,
                });
            } else {
                positions_to_check.push(Position {
                    row: last_row,
                    col: curr_col - 1,
                });
                positions_to_check.push(Position {
                    row: last_row,
                    col: curr_col + 1,
                });
                positions_to_check.push(Position {
                    row: last_row - 1,
                    col: curr_col - 1,
                });
                positions_to_check.push(Position {
                    row: last_row - 1,
                    col: curr_col,
                });
                positions_to_check.push(Position {
                    row: last_row - 1,
                    col: curr_col + 1,
                });
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if curr_col == 0 {
                positions_to_check.push(Position {
                    row: curr_row - 1,
                    col: 0,
                });
                positions_to_check.push(Position {
                    row: curr_row - 1,
                    col: 1,
                });
                positions_to_check.push(Position {
                    row: curr_row,
                    col: 1,
                });
                positions_to_check.push(Position {
                    row: curr_row + 1,
                    col: 0,
                });
                positions_to_check.push(Position {
                    row: curr_row + 1,
                    col: 1,
                });
            } else if curr_col == last_col {
                positions_to_check.push(Position {
                    row: curr_row - 1,
                    col: last_col - 1,
                });
                positions_to_check.push(Position {
                    row: curr_row - 1,
                    col: last_col,
                });
                positions_to_check.push(Position {
                    row: curr_row,
                    col: last_col - 1,
                });
                positions_to_check.push(Position {
                    row: curr_row + 1,
                    col: last_col - 1,
                });
                positions_to_check.push(Position {
                    row: curr_row + 1,
                    col: last_col,
                });
            } else {
                positions_to_check.push(Position {
                    row: curr_row - 1,
                    col: curr_col - 1,
                });
                positions_to_check.push(Position {
                    row: curr_row - 1,
                    col: curr_col,
                });
                positions_to_check.push(Position {
                    row: curr_row - 1,
                    col: curr_col + 1,
                });
                positions_to_check.push(Position {
                    row: curr_row,
                    col: curr_col - 1,
                });
                positions_to_check.push(Position {
                    row: curr_row,
                    col: curr_col + 1,
                });
                positions_to_check.push(Position {
                    row: curr_row + 1,
                    col: curr_col - 1,
                });
                positions_to_check.push(Position {
                    row: curr_row + 1,
                    col: curr_col,
                });
                positions_to_check.push(Position {
                    row: curr_row + 1,
                    col: curr_col + 1,
                });
            }
        }

        positions_to_check
            .iter()
            .filter(|&p: &&Position| all_chars[p.row][p.col] == '*')
            .map(|p| *p)
            .collect::<Vec<Position>>()
    }
}

#[derive(Debug, PartialEq)]
struct Number {
    digits: Vec<NumberDigit>,
}

trait NumberTrait {
    fn add_digit(&mut self, position: Position, all_chars: &Vec<Vec<char>>);
    // fn set_as_part_num(&mut self);
    // fn to_string(&self) -> String;
    // fn digits_to_int(&self) -> u32;
    fn get_all_adjacent_asterisk_positions(&self) -> Vec<Position>;
}

impl NumberTrait for Number {
    fn add_digit(&mut self, position: Position, all_chars: &Vec<Vec<char>>) {
        self.digits.push(NumberDigit::new(position, all_chars));
    }

    fn get_all_adjacent_asterisk_positions(&self) -> Vec<Position> {
        let mut result: Vec<Position> = Vec::new();
        for digit in self.digits.iter() {
            result.extend(digit.adjacent_asterisk_positions.clone())
        }
        result
    }

    // fn set_as_part_num(&mut self) {
    //     self.is_part_num = true;
    // }
    //
    // fn to_string(&self) -> String {
    //     self.digits.iter().map(|d| d.to_string()).join("") + "|" + &self.is_part_num.to_string()
    // }
    //
    // fn digits_to_int(&self) -> u32 {
    //     let mut result: u32 = 0;
    //     for (digit_index, digit) in self.digits.iter().rev().enumerate() {
    //         result += digit * 10_u32.pow(digit_index as u32);
    //     }
    //     result
    // }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.digits.eq(&other.digits)
    }
}

fn run(input_text: &str) -> io::Result<u32> {
    let mut all_chars: Vec<Vec<char>> = Vec::new();

    for line in input_text.lines() {
        let line_chars: Vec<char> = line.chars().collect();
        all_chars.push(line_chars);
    }

    // Get all numbers, and for each of them, save the position of all surrounding '*'
    let all_numbers = get_all_numbers(&all_chars);
    // .iter()
    // Discard the numbers that have no surrounding '*'
    // .filter(|&n| n.get_all_adjacent_asterisk_positions().len() > 0)
    // .map(|n| *n)
    // .collect::<Vec<Number>>();

    // Now we have to reverse the assignment: for each asterisk, we need to find the numbers that
    // are surrounding it
    let mut asterisks_to_numbers: HashMap<Position, Vec<Number>> = HashMap::new();
    for mut number in all_numbers {
        for asterisk_position in number.get_all_adjacent_asterisk_positions() {
            if asterisks_to_numbers.contains_key(&asterisk_position) {
                if !(asterisks_to_numbers
                    .get(&asterisk_position)
                    .unwrap()
                    .contains(&number))
                {
                    asterisks_to_numbers
                        .get_mut(&asterisk_position)
                        .unwrap()
                        .push(number);
                }
            }
        }
    }

    // Ok(all_numbers
    //     .iter()
    //     .filter(|n| n.is_part_num)
    //     .map(|n| n.digits_to_int())
    //     .sum())
    Ok(1u32)
}

/**
 * Please note: minimum size of all_chars is 2x2
 */
fn get_all_numbers(all_chars: &Vec<Vec<char>>) -> Vec<Number> {
    let mut all_numbers: Vec<Number> = Vec::new();
    for (row_index, row) in all_chars.iter().enumerate() {
        let mut is_prev_char_a_digit = false;
        for (col_index, ch) in row.iter().enumerate() {
            if *ch == '.' {
                is_prev_char_a_digit = false;
                continue;
            }
            match ch.to_digit(10) {
                Some(num) => {
                    let curr_position = Position {
                        row: row_index,
                        col: col_index,
                    };
                    if is_prev_char_a_digit {
                        let last_num = all_numbers.last_mut().unwrap();
                        last_num.add_digit(curr_position, &all_chars);
                        // If we have already found previously that the last Number was a part number,
                        // we don't have to make that calculation again
                        // if !last_num.is_part_num
                        //     && is_adjacent_to_symbol_num(&curr_position, &all_chars)
                        // {
                        //     last_num.set_as_part_num();
                        // }
                    } else {
                        // let is_part_num = is_adjacent_to_symbol_num(&curr_position, &all_chars);
                        // TODO constructor without digits.
                        let mut new_number = Number { digits: vec![] };
                        new_number.add_digit(curr_position, &all_chars);
                        all_numbers.push(new_number)
                    }
                    is_prev_char_a_digit = true;
                }
                None => {
                    is_prev_char_a_digit = false;
                }
            }
        }
    }
    all_numbers
}

fn is_symbol(ch: char) -> bool {
    // We are compiling the Regex on every call...
    let symbol_regex = Regex::new(r"[^\d.\s]").unwrap();
    symbol_regex.is_match(&ch.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_works_example() {
    //     assert_eq!(
    //         nums_to_string(&get_all_numbers(&vec![
    //             vec!['4', '6', '7', '.', '.', '1', '1', '4', '.', '.'],
    //             vec!['.', '.', '.', '*', '.', '.', '.', '.', '.', '.'],
    //             vec!['.', '.', '3', '5', '.', '.', '6', '3', '3', '.'],
    //             vec!['.', '.', '.', '.', '.', '.', '#', '.', '.', '.'],
    //             vec!['6', '1', '7', '*', '.', '.', '.', '.', '.', '.'],
    //             vec!['.', '.', '.', '.', '.', '+', '.', '5', '8', '.'],
    //             vec!['.', '.', '5', '9', '2', '.', '.', '.', '.', '.'],
    //             vec!['.', '.', '.', '.', '.', '.', '7', '5', '5', '.'],
    //             vec!['.', '.', '.', '$', '.', '*', '.', '.', '.', '.'],
    //             vec!['.', '6', '6', '4', '.', '5', '9', '8', '.', '.'],
    //         ])),
    //         "467|true,114|false,35|true,633|true,617|true,58|false,592|true,755|true,664|true,598|true"
    //     );
    // }

    #[test]
    fn test_run() {
        let test_text: &str = include_str!("input/test.txt");
        assert_eq!(4361u32, run(test_text).unwrap());
    }

    // fn nums_to_string(all_numbers: &Vec<Number>) -> String {
    //     all_numbers.iter().map(|n| n.to_string()).join(",")
    // }
}
