use regex::Regex;

use crate::utils::{
    file_iterator,
    string_to_int::string_to_int_u64,
};

const DATA_PATH: &str = "./src/solutions/day5/input.txt";

#[derive(Clone)]
struct Map {
    name: String,
    ranges: Vec<Range>,
}

#[derive(Copy, Clone)]
struct Range {
    destination_range_start: u64,
    source_range_start: u64,
    range_length: u64,
}

pub fn solve() {
    let mut result = 0;
    // get the two sets of numbers for each line
    let find_numbers = Regex::new(r"(\d+)").unwrap();
    let find_string = Regex::new(r"(?P<map_name>.*) map:").unwrap();
    let find_values = Regex::new(r"(?P<drs>\d+) (?P<srs>\d+) (?P<rl>\d+)").unwrap();
    let mut seeds: Vec<u64> = vec![];
    let mut maps: Vec<Map> = vec![];

    if let Ok(lines) = file_iterator::read_lines(DATA_PATH) {
        let mut current_map = Map {
            name: "".to_string(),
            ranges: vec![],
        };
        for (line_no, line) in lines.enumerate() {
            if let Ok(line_ok) = line {
                // setup the seeds
                if line_no == 0 {
                    for num in find_numbers.find_iter(&line_ok).map(|m| m.as_str()) {
                        seeds.push(string_to_int_u64(&num.to_string()));
                    }
                } else if line_ok.contains("map:") {
                    let caps = find_string.captures(&line_ok).unwrap();
                    let map_name = &caps["map_name"];
                    current_map.name = map_name.to_string();
                } else if let Some(values) = find_values.captures(&line_ok) {
                    current_map.ranges.push(Range {
                        destination_range_start: string_to_int_u64(&values["drs"].to_string()),
                        source_range_start: string_to_int_u64(&values["srs"].to_string()),
                        range_length: string_to_int_u64(&values["rl"].to_string()),
                    });
                } else if line_ok == "" && line_no != 1 {
                    println!("map found:{}, number of ranges: {}", current_map.name, current_map.ranges.len());
                    maps.push(current_map);
                    current_map = Map {
                        name: "".to_string(),
                        ranges: vec![],
                    };
                } else if line_ok != "" {
                    panic!("Something went wrong: {}", line_ok);
                }
            }
        }
    }

    let mut current_num: i128;

    for seed in seeds {
        current_num = i128::from(seed);
        println!("seed: {}", seed);
        for map in maps.clone() {
            for range in map.ranges {
                let source_range = i128::from(range.source_range_start)..i128::from(range.source_range_start+range.range_length);
                if source_range.contains(&current_num) {
                    current_num = i128::from(range.destination_range_start) - i128::from(range.source_range_start) + current_num;
                    break;
                }
            }
            println!("after {} map: {}", map.name, current_num);
        }

        println!("after all maps: {}", current_num);

        if current_num < result || result == 0 {
            result = current_num;
        }
    }

    println!("result: {}", result);  

}
