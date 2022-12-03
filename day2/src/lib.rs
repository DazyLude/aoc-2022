use std::io::BufRead;
use std::io::BufReader;
use std::string::String;
use std::vec::Vec;

use std::fs;

pub fn do_the_thing() -> Result<String, String> {
    let match_log = read_input_file();
    let mut score: i64 = 0;
    for line in match_log {
        score += decypher_match_log(line).unwrap();
    }
    println!("{}", score);
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

#[derive(PartialEq, Copy, Clone)]
enum RPSShapes {
    Rock,
    Paper,
    Scirrors,
}

fn rps_beaten_by(what: RPSShapes) -> RPSShapes {
    match what {
        RPSShapes::Rock => RPSShapes::Scirrors,
        RPSShapes::Scirrors => RPSShapes::Paper,
        RPSShapes::Paper => RPSShapes::Rock,
    }
}

fn rps_beats(what: RPSShapes) -> RPSShapes {
    match what {
        RPSShapes::Rock => RPSShapes::Paper,
        RPSShapes::Scirrors => RPSShapes::Rock,
        RPSShapes::Paper => RPSShapes::Scirrors,
    }
}

enum MatchResult {
    Win,
    Draw,
    Loss,
}

fn decypher_match_log(rps_round: String) -> Result<i64, String> {
    let opponents_shape = match rps_round.chars().nth(0).unwrap() {
        'A' => RPSShapes::Rock,
        'B' => RPSShapes::Paper,
        'C' => RPSShapes::Scirrors,
        _ => return Err("Unknown input value for shape of opp's choice".to_string()),
    };

    let match_result = match rps_round.chars().nth(2).unwrap() {
        'X' => MatchResult::Loss,
        'Y' => MatchResult::Draw,
        'Z' => MatchResult::Win,
        _ => return Err("Unknown input value for match result".to_string()),
    };

    Ok(determine_match_score(match_result, opponents_shape))
}

fn determine_match_score(match_result: MatchResult, opps_shape: RPSShapes) -> i64 {
    let mut score = 0;

    score += match match_result {
        MatchResult::Win => 6,
        MatchResult::Loss => 0,
        MatchResult::Draw => 3,
    };

    let my_shape: RPSShapes = match match_result {
        MatchResult::Draw => opps_shape,
        MatchResult::Win => rps_beats(opps_shape),
        MatchResult::Loss => rps_beaten_by(opps_shape),
    };

    score += match my_shape {
        RPSShapes::Rock => 1,
        RPSShapes::Paper => 2,
        RPSShapes::Scirrors => 3,
    };

    score
}
