use std::io::BufRead;
use std::io::BufReader;

use std::vec::Vec;

use std::fs;

fn main() {
    let mut boat = Cargo::from_input_file();

    boat.use_crane_mover_9001();

    println!("{:?}", boat.get_top_ones());
}

struct Cargo {
    pub cargo: Vec<Vec<char>>,
    pub instructions: Vec<[i8; 3]>,
}

impl Cargo {
    fn from_input_file() -> Cargo {
        let mut new_cargo = Cargo {
            cargo: [].to_vec(),
            instructions: [].to_vec(),
        };

        let input_file = fs::File::open("input").expect("Could not open input file");
        let input_lines = BufReader::new(input_file).lines();

        let mut initial_state = Vec::<String>::new();
        let mut reading_initial_state = true;

        for wrapped_line in input_lines {
            let line = wrapped_line.expect("Error when reading a line from the input");
            if line.is_empty() {
                reading_initial_state = false;
                continue;
            };

            if reading_initial_state {
                initial_state.push(line);
            } else {
                new_cargo.parse_instruction(&line);
            }
        }

        new_cargo.parse_initial_state(&initial_state);

        new_cargo
    }

    fn parse_instruction(&mut self, input: &str) {
        let mut commands: [i8; 3] = [0; 3];

        let input_comms: Vec<&str> = input.split_ascii_whitespace().collect();

        for i in 0..3 {
            commands[i] = input_comms[i * 2 + 1].parse::<i8>().unwrap();
        }

        self.instructions.push(commands);
    }

    fn parse_initial_state(&mut self, input: &[String]) {
        let mut layers = input.iter().rev();
        layers
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .for_each(|_| self.cargo.push([].to_vec()));

        let mut parse_layer = |crates: &str| {
            for i in 0..self.cargo.len() {
                let crate_name = crates.chars().nth(1 + i * 4).unwrap();
                if !crate_name.is_whitespace() {
                    self.cargo[i].push(crate_name);
                }
            }
        };

        layers.for_each(|layer| parse_layer(layer));
    }

    fn _use_crane_mover_9000(&mut self) {
        for commands in self.instructions.iter() {
            for _ in 0..commands[0] {
                let a_crate = self.cargo[commands[1] as usize - 1].pop().unwrap();
                self.cargo[commands[2] as usize - 1].push(a_crate);
            }
        }
    }

    fn use_crane_mover_9001(&mut self) {
        for commands in self.instructions.iter() {
            let mut temp = Vec::<char>::new();
            for _ in 0..commands[0] {
                temp.push(self.cargo[commands[1] as usize - 1].pop().unwrap());
            }
            temp.reverse();
            self.cargo[commands[2] as usize - 1].append(&mut temp);
        }
    }

    fn get_top_ones(&self) -> String {
        let mut output = "".to_string();

        self.cargo
            .iter()
            .for_each(|stack| output.push(*stack.last().unwrap()));

        output
    }
}
