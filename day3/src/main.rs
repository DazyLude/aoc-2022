use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::string::String;
use std::vec::Vec;

use std::fs;

fn main() {
    let lines = read_input_file();

    let pt1_answer = lines
        .iter()
        .map(|contents| find_dupes(contents).unwrap().get_priority())
        .sum::<i64>();

    let mut pt2_answer: i64 = 0;
    for i in 0..(lines.len() / 3) {
        pt2_answer += find_dupes_among_three(&lines[i * 3], &lines[i * 3 + 1], &lines[i * 3 + 2])
            .unwrap()
            .get_priority();
    }

    println!("{}, {}", pt1_answer, pt2_answer);
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

fn find_dupes(rucksack: &String) -> Result<char, String> {
    let (first_compartment, second_compartment) = rucksack.split_at(rucksack.len() / 2);

    if first_compartment.len() != second_compartment.len() {
        return Err(format!("unequal number of items in {}", rucksack));
    }

    let mut contents_map = HashMap::<char, usize>::new();

    for item in first_compartment.chars() {
        contents_map.insert(item, 0);
    }

    let mut dupe: Option<char> = None;

    for item in second_compartment.chars() {
        if contents_map.contains_key(&item) {
            if dupe != None && dupe.unwrap() != item {
                return Err(format!("Multiple dupes found in {}", rucksack));
            }
            dupe = Some(item);
        }
    }

    if dupe == None {
        return Err("No dupes found".to_string());
    }
    return Ok(dupe.unwrap());
}

fn find_dupes_among_three(st: &String, nd: &String, rd: &String) -> Result<char, String> {
    let mut contents_map = HashMap::<char, usize>::new();

    for item in st.chars() {
        contents_map.insert(item, 0);
    }

    for item in nd.chars() {
        if contents_map.contains_key(&item) {
            contents_map.insert(item, 1);
        }
    }

    let mut dupe: Option<char> = None;

    for item in rd.chars() {
        if contents_map.contains_key(&item) && contents_map[&item] == 1 {
            if dupe != None && dupe.unwrap() != item {
                return Err(format!("Multiple dupes found in {}, {} and {}", st, nd, rd));
            }
            dupe = Some(item)
        }
    }

    return Ok(dupe.unwrap());
}

trait Prioritisable {
    fn get_priority(self: Self) -> i64;
}

impl Prioritisable for char {
    fn get_priority(self) -> i64 {
        let char_code = self as u8;
        if char_code < 97 {
            return (char_code - 65 + 27).into();
        } else {
            return (char_code - 97 + 1).into();
        }
    }
}
