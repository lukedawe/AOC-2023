use core::num;
use std::string;

use crate::utils::{
    file_iterator,
    string_to_int::{self, is_int}, print_arrays::print_2d_vec,
};

const DATA_PATH: &str = "./src/solutions/day3/sample2.txt";

pub fn solve() {
    let mut part_numbers: Vec<u32> = vec![];
    let mut sum = 0;
    let mut input: Vec<Vec<char>> = vec![];
    if let Ok(lines) = file_iterator::read_lines(DATA_PATH) {
        for (index, line) in lines.enumerate() {
            if let Ok(line_ok) = line {
                input.push(vec![]);
                // read the line into a 2d array
                for c in line_ok.chars().into_iter() {
                    input[index].push(c);
                }
            }
        }
    }

    // for each vec in the vec
    let dimens = (input[0].len(), input.len());

    // we have to clone the input so we can use it later on
    for (y, line) in input.clone().into_iter().enumerate() {
        let mut num_found: bool = false;
        let mut current_num: String = String::new();
        let mut current_num_is_part_num: bool = false;
        for (x, char) in line.into_iter().enumerate() {
            // this block checks to see if anything around it is a symbol
            if is_int(&char) {
                if x > 0 {
                    if is_symbol(&input[y][x - 1]) {
                        current_num_is_part_num = true;
                    }
                    if y > 0 {
                        if is_symbol(&input[y - 1][x - 1]) {
                            current_num_is_part_num = true;
                        }
                    }
                    if y < dimens.1 - 1 {
                        if is_symbol(&input[y + 1][x - 1]) {
                            current_num_is_part_num = true;
                        }
                    }
                }
                if y > 0 {
                    if is_symbol(&input[y - 1][x]) {
                        current_num_is_part_num = true;
                    }
                    if x < dimens.0 - 1 {
                        if is_symbol(&input[y - 1][x + 1]) {
                            current_num_is_part_num = true;
                        }
                    }
                }
                if y < dimens.1 - 1 {
                    if is_symbol(&input[y + 1][x]) {
                        current_num_is_part_num = true;
                    }
                    if x < dimens.0 - 1 {
                        if is_symbol(&input[y + 1][x + 1]) {
                            current_num_is_part_num = true;
                        }
                    }
                }
                if x < dimens.0 - 1 {
                    if is_symbol(&input[y][x + 1]) {
                        current_num_is_part_num = true;
                    }
                }

                if !num_found {
                    num_found = true;
                    current_num = char.to_string();
                } else {
                    current_num += &char.to_string();
                }

            /*
            when we have found the end of the current number and number is a part number
            add that to the sum.
             */
            } else if (char == '.' || is_symbol(&char)) && current_num_is_part_num {
                sum += string_to_int::string_to_int(&current_num);
                part_numbers.push(string_to_int::string_to_int(&current_num));
                num_found = false;
                current_num_is_part_num = false;
                // print!("({}).*", current_num);
            } else {
                num_found = false;
                current_num_is_part_num = false;
            }
        }
        // now we deal with the numbers at the end of the row
        if current_num_is_part_num {
            sum += string_to_int::string_to_int(&current_num);
            part_numbers.push(string_to_int::string_to_int(&current_num));
        }
        // print!("\\n.*");
    }

    println!("Sum: {}", sum);
}

fn is_symbol(char: &char) -> bool {
    match char {
        '&' | '#' | '+' | '$' | '@' | '=' | '%' | '/' | '*' | '-' => return true,
        _ => return false,
    }
}
