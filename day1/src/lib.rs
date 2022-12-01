use std::io::BufRead;
use std::io::BufReader;
use std::string::String;
use std::vec::Vec;

use std::fs;

pub fn do_the_thing() -> Result<String, String> {
    let raw_input = read_input_file();
    let counted_input = count_calories(&raw_input);
    println!(
        "the most any of the elves have is {:?} calories",
        counted_input.iter().max()
    );

    println!(
        "top 3 elves have total {:?} calories",
        get_top_three(&counted_input)
    );

    return Ok("finished successfully".to_string());
}

fn read_input_file() -> Vec<String> {
    let input_file = fs::File::open("input").expect("Could not open input file");
    let input_buf_read = BufReader::new(input_file);
    let mut output_vector = Vec::<String>::new();

    for line in input_buf_read.lines() {
        output_vector.push(line.expect("Error when reading a line from the input"));
    }

    return output_vector;
}

fn count_calories(input: &Vec<String>) -> Vec<i64> {
    let mut counted_calories = Vec::<i64>::from([0]);

    for string_of_cals in input {
        if string_of_cals.len() == 0 {
            counted_calories.push(0);
        } else {
            let new_item_cals = string_of_cals
                .parse::<i64>()
                .expect("Error when parsing a line from input");
            *counted_calories.last_mut().unwrap() += new_item_cals;
        }
    }

    return counted_calories;
}

fn get_top_three(counted: &Vec<i64>) -> i64 {
    if counted.len() <= 3 {
        return counted.iter().sum();
    }

    let mut top_three = [0, 0, 0];

    for elf in counted {
        if *elf > top_three[0] {
            top_three[0] = *elf;
            top_three.sort();
        }
    }
    top_three.iter().sum()
}
