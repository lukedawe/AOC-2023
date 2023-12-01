use crate::utils::file_iterator;
use std::{borrow::BorrowMut, f32::RADIX, ops::Add, ptr::null};

const DATA_PATH: &str = "./src/solutions/day1/input.txt";

pub fn solve() {
    let mut total = 0;
    if let Ok(lines) = file_iterator::read_lines(DATA_PATH) {
        for line in lines {
            let mut first: Option<u32> = None;
            let mut last: Option<u32> = None;
            if let Ok(mut code) = line {
                println!("code: {}", code);

                // mutable borrow code
                replace_strings_with_char(&mut code);

                // this is the code-scope
                if code != String::from("") {
                    for (_index, c) in code.chars().enumerate() {
                        if let Some(num) = c.to_digit(10) {
                            if first.is_none() {
                                first = Some(num);
                            }
                            last = Some(num);
                        }
                    }
                }
            }
            if first.is_some() && last.is_some() {
                println!("first: {}, last: {}", first.unwrap(), last.unwrap());
                total = total + (first.unwrap() * 10) + last.unwrap();
            } else {
                println!("Something went badly wrong")
            }
        }
    }

    print!("Total: {}", total);
}

// borrows a string and replaces numbers with their character representatives
fn replace_strings_with_char(string: &mut String) {
    let mut index = 1;
    let mut slice: String = String::new();

    while index <= string.len() {
        let mut slice = string[..index].to_string();

        slice = str::replace(&slice, "one", "1e");
        slice = str::replace(&slice, "two", "2o");
        slice = str::replace(&slice, "three", "3e");
        slice = str::replace(&slice, "four", "4r");
        slice = str::replace(&slice, "five", "5e");
        slice = str::replace(&slice, "six", "6x");
        slice = str::replace(&slice, "seven", "7n");
        slice = str::replace(&slice, "eight", "8t");
        slice = str::replace(&slice, "nine", "9e");

        *string = slice.clone() + &string[index..];
        // the slice can change in size, so make sure the index reacts to this
        index = slice.len() + 1;

    }

    println!("replaced string: {}", string);
}
