// #![allow(dead_code, unused_variables)]

use serde_json::Value as JSONValue;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("pt1: {}", get_input_checksum());
    let mut jsons = get_input_to_vector();
    jsons.sort_by(json_partial_cmp);
    let first_json: JSONValue = serde_json::from_str("[[2]]").unwrap();
    let second_json: JSONValue = serde_json::from_str("[[6]]").unwrap();

    let first_index = jsons
        .iter()
        .position(|json| json_partial_cmp(json, &first_json) == Ordering::Greater)
        .unwrap()
        + 1;

    let second_index = jsons
        .iter()
        .position(|json| json_partial_cmp(json, &second_json) == Ordering::Greater)
        .unwrap()
        + 2;

    println!("pt2: {}", first_index * second_index);
}

fn get_input_checksum() -> i64 {
    let input_file = File::open("input").expect("Could not open input file");
    let input_buf_read = BufReader::new(input_file);
    let mut checksum: i64 = 0;

    let mut left_packet: JSONValue = JSONValue::Null;

    for (line_counter, wrapped_line) in input_buf_read.lines().enumerate() {
        match line_counter % 3 {
            0 => left_packet = serde_json::from_str(&wrapped_line.unwrap()).unwrap(),
            1 => {
                let right_packet = serde_json::from_str(&wrapped_line.unwrap()).unwrap();
                if compare_jsons(&left_packet, &right_packet).unwrap_or_else(|| {
                    panic!("two identical packets encountered: {left_packet}, {right_packet}")
                }) {
                    checksum += 1 + line_counter as i64 / 3
                }
            }
            _ => continue,
        };
    }
    checksum
}

fn get_input_to_vector() -> Vec<JSONValue> {
    let input_file = File::open("input").expect("Could not open input file");
    let input_buf_read = BufReader::new(input_file);
    let mut result = Vec::<JSONValue>::new();

    for wrapped_line in input_buf_read.lines() {
        match serde_json::from_str(&wrapped_line.unwrap()) {
            Err(_) => continue,
            Ok(not_null) => result.push(not_null),
        }
    }
    result
}

fn compare_jsons(left: &JSONValue, right: &JSONValue) -> Option<bool> {
    match (left, right) {
        (JSONValue::Number(left), JSONValue::Number(right)) => {
            if left.as_i64() == right.as_i64() {
                None
            } else {
                Some(left.as_i64() < right.as_i64())
            }
        }
        (JSONValue::Array(left), JSONValue::Array(right)) => {
            let mut left_iter = left.iter();
            let mut right_iter = right.iter();
            loop {
                let (left_item, right_item) = (left_iter.next(), right_iter.next());
                match (left_item, right_item) {
                    (None, None) => return None,
                    (None, Some(_)) => return Some(true),
                    (Some(_), None) => return Some(false),
                    (Some(left), Some(right)) => match compare_jsons(left, right) {
                        None => continue,
                        something => return something,
                    },
                }
            }
        }
        (JSONValue::Number(_), JSONValue::Array(_)) => {
            let vec_package = vec![left.clone()];
            compare_jsons(&JSONValue::Array(vec_package), right)
        }
        (JSONValue::Array(_), JSONValue::Number(_)) => {
            let vec_package = vec![right.clone()];
            compare_jsons(left, &JSONValue::Array(vec_package))
        }

        _ => panic!(
            "unexpected JSON value encountered: left: {}, right: {}",
            left, right
        ),
    }
}

fn json_partial_cmp(lhs: &JSONValue, rhs: &JSONValue) -> Ordering {
    match compare_jsons(lhs, rhs) {
        None => Ordering::Equal,
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
    }
}
