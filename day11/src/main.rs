#![allow(dead_code, unused_variables)]

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let mut monkies = Vec::<Monke>::new();
    read_monkies(&mut monkies);

    let mut result = Vec::<i64>::new();
    monkies.iter().for_each(|_| result.push(0));
    let worry_manager = monkies
        .iter()
        .map(|monke| monke.test_v)
        .reduce(|acc, check| acc * check)
        .unwrap();

    for _ in 0..10000 {
        let temp = do_a_round(&mut monkies, worry_manager);
        for monke_id in 0..result.len() {
            result[monke_id] += temp[monke_id];
        }
    }

    result.sort();

    println!("{:?}", get_monkey_business_value(&mut result));
}

fn get_monkey_business_value(monkey_activity: &mut Vec<i64>) -> Option<i64> {
    let first = monkey_activity.pop()?;
    let second = monkey_activity.pop()?;
    Some(first * second)
}

fn read_monkies(monkies: &mut Vec<Monke>) {
    let input_file = File::open("input").expect("Could not open input file");
    let input_buf_read = BufReader::new(input_file);
    let mut line_counter = 0;

    let last_value_in_line = |line: &String| -> usize {
        line.split_ascii_whitespace()
            .rev()
            .next()
            .unwrap()
            .parse()
            .unwrap()
    };

    let extract_items = |line: &String| -> Vec<i64> {
        line.split_ascii_whitespace()
            .rev()
            .map_while(|item| match item.trim_end_matches(',').parse::<i64>() {
                Ok(num) => Some(num),
                Err(_) => None,
            })
            .collect()
    };

    let extract_operation = |line: &String| -> (WorryType, i64) {
        let mut line_iter = line.split_ascii_whitespace().rev();
        let worry_value = match line_iter.next().unwrap() {
            "old" => return (WorryType::Polynomial, 2),
            val => val.parse::<i64>().expect("unknown worry value"),
        };
        let worry_type = match line_iter.next().unwrap() {
            "*" => WorryType::Multiplicative,
            "+" => WorryType::Additive,
            _ => panic!("unknown worry type"),
        };

        (worry_type, worry_value)
    };

    input_buf_read.lines().for_each(|wrapped_line| {
        let line = wrapped_line.unwrap();
        match line_counter % 7 {
            0 => monkies.push(Monke::new()),
            1 => monkies.last_mut().unwrap().items_worry_levels = extract_items(&line),
            2 => {
                let operation = extract_operation(&line);
                monkies.last_mut().unwrap().worry_t = operation.0;
                monkies.last_mut().unwrap().worry_v = operation.1;
            }
            3 => monkies.last_mut().unwrap().test_v = last_value_in_line(&line) as i64,
            4 => monkies.last_mut().unwrap().true_throw = last_value_in_line(&line),
            5 => monkies.last_mut().unwrap().false_throw = last_value_in_line(&line),
            _ => {}
        }
        line_counter += 1;
    });
}

fn do_a_round(lot_of_monke: &mut [Monke], worry_manager: i64) -> Vec<i64> {
    let mut inspection_count = Vec::<i64>::new();
    for monke_id in 0..lot_of_monke.len() {
        let mut item_count = 0;
        while let Some(mut item) = lot_of_monke[monke_id].items_worry_levels.pop() {
            item_count += 1;
            match lot_of_monke[monke_id].worry_t {
                WorryType::Multiplicative => item *= lot_of_monke[monke_id].worry_v,
                WorryType::Additive => item += lot_of_monke[monke_id].worry_v,
                WorryType::Polynomial => item = item.pow(lot_of_monke[monke_id].worry_v as u32),
            }
            // item /= 3;
            item %= worry_manager;
            if item % lot_of_monke[monke_id].test_v == 0 {
                lot_of_monke[lot_of_monke[monke_id].true_throw]
                    .items_worry_levels
                    .push(item);
            } else {
                lot_of_monke[lot_of_monke[monke_id].false_throw]
                    .items_worry_levels
                    .push(item);
            }
        }
        inspection_count.push(item_count);
    }
    inspection_count
}

#[derive(Clone, Debug)]
enum WorryType {
    Multiplicative,
    Additive,
    Polynomial,
}

#[derive(Clone, Debug)]
struct Monke {
    items_worry_levels: Vec<i64>,
    worry_t: WorryType,
    worry_v: i64,
    test_v: i64,
    true_throw: usize,
    false_throw: usize,
}

impl Monke {
    fn new() -> Monke {
        Monke {
            items_worry_levels: Vec::<i64>::new(),
            worry_t: WorryType::Additive,
            worry_v: 0,
            test_v: 0,
            true_throw: 0,
            false_throw: 0,
        }
    }
}
