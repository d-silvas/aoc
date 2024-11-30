use std::collections::HashMap;
use std::io;
use std::time::{Duration, Instant};

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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, Clone)]
struct Number {
    digits: Vec<NumberDigit>,
}

impl Number {
    pub fn new() -> Self {
        Number { digits: vec![] }
    }
}

trait NumberTrait {
    fn add_digit(&mut self, position: Position, all_chars: &Vec<Vec<char>>);
    // TODO it might be possible to transform Number to i32 by implementing the Into or From traits
    fn digits_to_int(&self) -> u32;
    fn get_all_adjacent_asterisk_positions(&self) -> Vec<Position>;
}

impl NumberTrait for Number {
    fn add_digit(&mut self, position: Position, all_chars: &Vec<Vec<char>>) {
        self.digits.push(NumberDigit::new(position, all_chars));
    }

    fn digits_to_int(&self) -> u32 {
        let mut result: u32 = 0;
        for (digit_index, digit) in self.digits.iter().rev().enumerate() {
            result += digit.digit as u32 * 10_u32.pow(digit_index as u32);
        }
        result
    }

    fn get_all_adjacent_asterisk_positions(&self) -> Vec<Position> {
        let mut result: Vec<Position> = Vec::new();
        for digit in self.digits.iter() {
            result.extend(digit.adjacent_asterisk_positions.clone())
        }
        result
    }
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

    // Now we have to reverse the assignment: for each asterisk, we need to find the numbers that
    // are surrounding it
    let mut asterisks_to_numbers: HashMap<Position, Vec<Number>> = HashMap::new();
    for number in all_numbers {
        for asterisk_position in number.get_all_adjacent_asterisk_positions() {
            if asterisks_to_numbers.contains_key(&asterisk_position) {
                if !(asterisks_to_numbers
                    .get(&asterisk_position)
                    .unwrap()
                    .contains(&&number))
                {
                    asterisks_to_numbers
                        .get_mut(&asterisk_position)
                        .unwrap()
                        .push(number.clone());
                }
            } else {
                let number_vec = vec![number.clone()];
                asterisks_to_numbers.insert(asterisk_position, number_vec);
            }
        }
    }

    let result: u32 = asterisks_to_numbers
        // We don't care about asterisk positions here so we take the values only
        // https://stackoverflow.com/questions/56724014/how-do-i-collect-the-values-of-a-hashmap-into-a-vector#comment131100540_63727456
        .into_values()
        // Only consider the cases where an asterisk had 2 adjacent numbers (those asterisks were "gears")
        .filter(|numbers: &Vec<Number>| numbers.len() == 2)
        // We get a Vec<Vec<Number>> where each inner Vec has exactly 2 Numbers
        .map(|numbers| {
            numbers
                .iter()
                // Map each Number to u32
                .map(|n| n.digits_to_int())
                // Here we have an array of 2 u32, which are the 2 numbers which are adjacent to one gear
                .collect::<Vec<u32>>()
                .iter()
                // We multiply these 2 u32
                .fold(1_u32, |acc: u32, x: &u32| acc * x)
        })
        // Here we have an array of u32. Each element of this array is the multiplication of the 2
        // part numbers that were adjacent to a gear.
        .collect::<Vec<u32>>()
        .into_iter()
        // And finally, we sum all these quantities
        .sum();

    Ok(result)
}

fn get_all_numbers(all_chars: &Vec<Vec<char>>) -> Vec<Number> {
    let mut all_numbers: Vec<Number> = Vec::new();
    for (row_index, row) in all_chars.iter().enumerate() {
        let mut is_prev_char_a_digit = false;
        for (col_index, ch) in row.iter().enumerate() {
            match ch.to_digit(10) {
                Some(_) => {
                    let curr_position = Position {
                        row: row_index,
                        col: col_index,
                    };
                    if is_prev_char_a_digit {
                        let last_num = all_numbers.last_mut().unwrap();
                        last_num.add_digit(curr_position, &all_chars);
                    } else {
                        let mut new_number = Number::new();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let test_text: &str = include_str!("input/test.txt");
        assert_eq!(467835u32, run(test_text).unwrap());
    }
}
