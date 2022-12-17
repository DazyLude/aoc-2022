use std::io::BufRead;
use std::io::BufReader;

use std::fs::File;

fn main() {
    let mut srg = SingleRegMachine::new();
    read_and_apply_input(&mut srg);
    println!("pt1: {}", srg.signal_strength_history.iter().sum::<i64>());

    srg.render_history.iter().for_each(|line| {
        let stringed: String = line
            .iter()
            .map(|input| if *input { '#' } else { '.' })
            .collect();
        println!("{}", stringed);
    });
}

fn read_and_apply_input(srg: &mut SingleRegMachine) {
    let input_file = File::open("input").expect("Could not open input file");
    let input_buf_read = BufReader::new(input_file);

    input_buf_read.lines().for_each(|wrapped_line| {
        let line = wrapped_line.unwrap();
        let mut command_iter = line.split_ascii_whitespace();
        let command = command_iter.next();
        match command {
            Some("noop") => srg.skip(),
            Some("addx") => {
                let value = command_iter.next().unwrap().parse::<i64>().unwrap();
                srg.add(value);
            }
            _ => panic!("Unknown command code"),
        };
    });
}

struct SingleRegMachine {
    reg: i64,
    cycle: i64,
    signal_strength_history: Vec<i64>,
    render_history: Vec<Vec<bool>>,
}

impl SingleRegMachine {
    fn new() -> SingleRegMachine {
        SingleRegMachine {
            reg: 1,
            cycle: 1,
            signal_strength_history: Vec::<i64>::new(),
            render_history: Vec::<Vec<bool>>::new(),
        }
    }

    fn tick(&mut self) {
        if (self.cycle - 1) % 40 == 0 {
            self.render_history.push(Vec::<bool>::new());
        }
        if ((self.cycle - 1) % 40 - self.reg).abs() <= 1 {
            self.render_history.last_mut().unwrap().push(true);
        } else {
            self.render_history.last_mut().unwrap().push(false);
        }
        self.check_signal_strength();
        self.cycle += 1;
    }

    fn skip(&mut self) {
        self.tick()
    }

    fn add(&mut self, how_much: i64) {
        self.tick();
        self.tick();
        self.reg += how_much;
    }

    fn check_signal_strength(&mut self) {
        if self.cycle % 20 == 0 && (self.cycle / 20) % 2 == 1 {
            self.signal_strength_history.push(self.cycle * self.reg);
        }
    }
}
