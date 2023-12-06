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
    let mut card_multipliers: Vec<u32> = vec![1];
    // get the two sets of numbers for each line
    let re = Regex::new(r"(?P<winning_numbers>(\d+ *)+)\| +(?P<your_numbers>(\d+ *)+)").unwrap();
    let re_num = Regex::new(r"\d+").unwrap();
    if let Ok(lines) = file_iterator::read_lines(DATA_PATH) {
        for (card_num, line) in lines.enumerate() {
            if let Ok(line_ok) = line {
                let mut line_sum = 0;

                if let Some(caps) = re.captures(&line_ok) {

                    if card_multipliers.len() <= card_num {
                        card_multipliers.push(1);
                    }

                    let your_numbers_string = &caps[YOUR_TAG];
                    let winning_numbers_string = &caps[WIN_TAG];

                    let mut your_numbers: Vec<u32> = vec![];
                    let mut winning_numbers: Vec<u32> = vec![];

                    // populate the your numbers array
                    for num in re_num.find_iter(your_numbers_string).map(|m| m.as_str()) {
                        your_numbers.push(string_to_int(&num.to_string()));
                    }

                    // populate the winning numbers array
                    for num in re_num.find_iter(winning_numbers_string).map(|m| m.as_str()) {
                        winning_numbers.push(string_to_int(&num.to_string()));
                    }

                    // find how many winning numbers there are
                    for num in your_numbers.into_iter() {
                        if winning_numbers.contains(&num) {
                            line_sum += 1;
                        }
                    }

                    // take care of the card multipliers
                    // 0 wins 4 cards, this means that 1,2,3,4
                    for i in card_num + 1..card_num + 1 + line_sum {
                        // make sure the array has enough room
                        while i >= card_multipliers.len() {
                            card_multipliers.push(1);
                        }
                        card_multipliers[i] += card_multipliers[card_num];
                    }

                    println!("card {} won: {} cards", card_num, line_sum);

                    print!("card_multipliers: [");
                    for i in card_multipliers.clone() {
                        print!("{},", i);
                    }
                    print!("]\n");

                    sum += card_multipliers[card_num];
                } else {
                    println!("Something went wrong: {}", line_ok);
                }
            }
        }
    }

    println!("{}", sum);
}
