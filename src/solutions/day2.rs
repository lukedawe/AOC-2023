use crate::utils::{file_iterator, string_to_int};
use core::panic;
use regex::Regex;
use std::vec;

const DATA_PATH: &str = "./src/solutions/day2/input.txt";

pub fn solve() {
    let mut possible_games: Vec<u32> = vec![];
    let mut game_number: u32 = 1;
    let rgb_vals: [u32; 3] = [12, 13, 14];
    if let Ok(lines) = file_iterator::read_lines(DATA_PATH) {
        for line in lines {
            if let Ok(game) = line {
                // there are red, green and blue cubes in the bag
                let greatest_vals_rgb = get_rgb_max(&game);
                for (index, val) in rgb_vals.into_iter().enumerate() {
                    if greatest_vals_rgb[index] > val {
                        break;
                    }
                    if index == 2 {
                        possible_games.push(game_number);
                    }
                }
            }
            game_number += 1;
        }
    }
    let mut sum: u32 = 0;
    for game in possible_games.into_iter() {
        sum += game;
        println!("game: {}", game);
    }
    println!("Total: {}",sum);
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
