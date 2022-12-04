use std::io::BufRead;
use std::io::BufReader;

use std::vec::Vec;

use std::fs;

fn main() {
    let input = read_input_file();

    let completely_overlapped = input
        .iter()
        .map(|entry| {
            if is_completely_overlapped(entry) {
                1
            } else {
                0
            }
        })
        .sum::<i64>();

    let somewhat_overlapped = input
        .iter()
        .map(|entry| if is_not_overlapped(entry) { 0 } else { 1 })
        .sum::<i64>();

    println!(
        "Completely overlapped: {completely_overlapped}\nSomewhat overlapped: {somewhat_overlapped}"
    );
}

fn read_input_file() -> Vec<[u8; 4]> {
    let input_file = fs::File::open("input").expect("Could not open input file");
    let input_buf_read = BufReader::new(input_file);
    let mut output_vector = Vec::<[u8; 4]>::new();

    for wrapped_line in input_buf_read.lines() {
        let line = wrapped_line.unwrap();
        let l_dash = line.find('-').unwrap();
        let comma = line.find(',').unwrap();
        let r_dash = line.rfind('-').unwrap();

        output_vector.push(
            [
                line.get(0..l_dash).unwrap(),
                line.get(l_dash + 1..comma).unwrap(),
                line.get(comma + 1..r_dash).unwrap(),
                line.get(r_dash + 1..).unwrap(),
            ]
            .map(|value| value.parse::<u8>().unwrap()),
        );
    }

    return output_vector;
}

fn is_completely_overlapped(entry: &[u8; 4]) -> bool {
    (entry[0] >= entry[2] && entry[1] <= entry[3]) || (entry[0] <= entry[2] && entry[1] >= entry[3])
}

fn is_not_overlapped(entry: &[u8; 4]) -> bool {
    entry[2] > entry[1] || entry[0] > entry[3]
}
