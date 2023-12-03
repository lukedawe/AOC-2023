use crate::utils::{file_iterator, string_to_int};
use core::panic;
use regex::Regex;

const DATA_PATH: &str = "./src/solutions/day2/input.txt";

pub fn solve() {
    let mut total_powers = 0;
    if let Ok(lines) = file_iterator::read_lines(DATA_PATH) {
        for line in lines {
            if let Ok(game) = line {
                // there are red, green and blue cubes in the bag
                let mut power = 1;
                let greatest_vals_rgb = get_rgb_max(&game);
                for val in greatest_vals_rgb.into_iter() {
                    power *= val;
                }
                total_powers += power;
            }
        }
    }
    println!("Total powers: {}", total_powers);
    
}

/*
This is run for each game line, so return the greatest value for each colour in the game
to see if the game is possible
*/
const VALUE_CONST: &str = "value";
const COLOUR_CONST: &str = "colour";

fn get_rgb_max(line: &String) -> [u32; 3] {
    let mut results_array: [u32; 3] = [0, 0, 0];
    // find the colours and their number
    let re = Regex::new(r"(?P<value>\d+) (?P<colour>\w+)").unwrap();
    // TODO: find the highest of each colour
    for pair in re.find_iter(line).map(|m| m.as_str()) {
        let caps = re.captures(pair).unwrap();
        // match statement them into the array
        let value = string_to_int::string_to_int(&caps[VALUE_CONST].to_string());
        match &caps[COLOUR_CONST] {
            "red" => {
                if value > u32::from(results_array[0]) {
                    results_array[0] = value;
                }
            }
            "green" => {
                if value > u32::from(results_array[1]) {
                    results_array[1] = value;
                }
            }
            "blue" => {
                if value > u32::from(results_array[2]) {
                    results_array[2] = value;
                }
            }
            _ => {
                panic!("Match not found")
            }
        }
    }
    return results_array;
}
