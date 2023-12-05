use core::num;

use regex::Regex;

use crate::utils::{
    file_iterator,
    print_arrays::print_2d_vec,
    string_to_int::{self, is_int, string_to_int},
};

const DATA_PATH: &str = "./src/solutions/day4/input.txt";
const WIN_TAG: &str = "winning_numbers";
const YOUR_TAG: &str = "your_numbers";

pub fn solve() {
    let mut sum = 0;
    let mut input: Vec<Vec<char>> = vec![];
    // get the two sets of numbers for each line
    let re = Regex::new(r"(?P<winning_numbers>(\d+ *)+)\| +(?P<your_numbers>(\d+ *)+)").unwrap();
    let re_num = Regex::new(r"\d+").unwrap();
    if let Ok(lines) = file_iterator::read_lines(DATA_PATH) {
        for (index, line) in lines.enumerate() {
            if let Ok(line_ok) = line {
                let mut line_sum = 0;

                if let Some(caps) = re.captures(&line_ok) {
                    let your_numbers_string = &caps[YOUR_TAG];
                    let winning_numbers_string = &caps[WIN_TAG];

                    let mut your_numbers: Vec<u32> = vec![];
                    let mut winning_numbers: Vec<u32> = vec![];

                    for num in re_num.find_iter(your_numbers_string).map(|m| m.as_str()) {
                        your_numbers.push(string_to_int(&num.to_string()));
                    }

                    for num in re_num.find_iter(winning_numbers_string).map(|m| m.as_str()) {
                        winning_numbers.push(string_to_int(&num.to_string()));
                    }

                    for num in your_numbers.into_iter() {
                        if winning_numbers.contains(&num) {
                            if line_sum == 0 {
                                line_sum = 1;
                            } else {
                                line_sum *= 2;
                            }
                        }
                    }
                }
                else {
                    println!("Something went wrong: {}", line_ok);
                }

                sum += line_sum;
            }
        }
    }

    println!("{}", sum);
}
