// #![allow(dead_code, unused_variables)]

use std::collections::{HashMap, VecDeque};

use std::fs::File;

use std::io::Read;

fn main() {
    let mut input_string = String::new();
    File::open("input")
        .expect("Could not open input file")
        .read_to_string(&mut input_string)
        .expect("could not read file to a string");

    let mut packet_marker = UniqueBuffer::new();

    let (packet_index, _) = input_string
        .chars()
        .enumerate()
        .find(|(_, ch)| {
            packet_marker.insert_new(*ch);
            packet_marker.is_valid_marker()
        })
        .unwrap();

    let mut message_marker = UniqueBuffer::with_size(14);

    let (msg_index, _) = input_string
        .chars()
        .enumerate()
        .find(|(_, ch)| {
            message_marker.insert_new(*ch);
            message_marker.is_valid_marker()
        })
        .unwrap();

    println!(
        "Packet starts at: {}\nMessage starts at: {}",
        packet_index + 1,
        msg_index + 1
    );
}

struct UniqueBuffer {
    size: usize,
    contents: VecDeque<char>,
}

impl UniqueBuffer {
    fn new() -> UniqueBuffer {
        UniqueBuffer {
            size: 4,
            contents: VecDeque::<char>::new(),
        }
    }

    fn with_size(size: usize) -> UniqueBuffer {
        UniqueBuffer {
            size,
            contents: VecDeque::<char>::new(),
        }
    }

    fn is_valid_marker(&self) -> bool {
        self.check_uniqueness() && self.contents.len() == self.size
    }

    fn check_uniqueness(&self) -> bool {
        let mut contents_count = HashMap::<char, usize>::new();
        for byte in &self.contents {
            contents_count
                .entry(*byte)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        for count in contents_count.into_values() {
            if count > 1 {
                return false;
            };
        }
        true
    }

    fn insert_new(&mut self, input: char) -> Option<char> {
        self.contents.push_back(input);
        if self.contents.len() > self.size {
            Some(self.contents.pop_front().unwrap())
        } else {
            None
        }
    }
}
