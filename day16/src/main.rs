use std::fs::File;

use std::io::{BufRead, BufReader};

use std::collections::{HashMap, HashSet};

fn main() {
    // a_sudden_fizzbuzz();
    let mut caves = read_input();

    // _print_times(&mut caves);

    let pt1 = caves.travelling_salesman_opens_valves(CaveNetwork::code_to_u16("AA"));
    println!("pt1: {pt1}");

    let pt2 = caves.me_and_my_elephant_gf(CaveNetwork::code_to_u16("AA"));
    println!("pt2: {pt2}");
}

fn _print_times(caves: &mut CaveNetwork) {
    let important_valves = caves.important_valves.clone();
    for valve1 in &important_valves {
        for valve2 in &important_valves {
            println!(
                "{} to {}: {}",
                CaveNetwork::_u16_to_code(*valve1),
                CaveNetwork::_u16_to_code(*valve2),
                caves.shortest_route_length(*valve1, *valve2)
            );
        }
    }
}

fn _print_connections(caves: &CaveNetwork) {
    let important_valves = caves.important_valves.clone();
    for valve1 in &important_valves {
        println!(
            "{} neighbors: {}",
            CaveNetwork::_u16_to_code(*valve1),
            caves.network[valve1]
                .connections
                .iter()
                .map(|hash| CaveNetwork::_u16_to_code(*hash))
                .collect::<String>()
        );
    }
}

fn _a_sudden_fizzbuzz() {
    for i in 0..100 {
        let output = match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz!".to_string(),
            (_, 0) => "Buzz?".to_string(),
            (0, _) => "Fizz...".to_string(),
            _ => format!("{i} IS NOT EVEN MY FINAL FORM"),
        };
        println!("{output}");
    }
}

fn read_input() -> CaveNetwork {
    let input_file = File::open("input").expect("Could not open input file");
    let input_buf_read = BufReader::new(input_file);
    let mut cave_network = CaveNetwork::new();

    for wrapped_line in input_buf_read.lines() {
        let line = wrapped_line.unwrap();
        let words: Vec<&str> = line.split_ascii_whitespace().collect();
        let flow_rate = words[4]
            .trim_start_matches("rate=")
            .trim_end_matches(';')
            .parse::<i64>()
            .unwrap();
        let index = CaveNetwork::code_to_u16(words[1]);
        let mut connections = Vec::<u16>::new();
        for word in words.iter().skip(9) {
            connections.push(CaveNetwork::code_to_u16(word));
        }
        cave_network.network.insert(
            index,
            Valve {
                connections,
                flow_rate,
            },
        );
    }
    cave_network.filter_important();
    cave_network.set_max_flow();

    cave_network
}

#[derive(Debug)]
struct CaveNetwork {
    network: HashMap<u16, Valve>,
    important_valves: HashSet<u16>,
    shortest_routes: HashMap<(u16, u16), i64>,
    max_flow: i64,
    time_limit: i64,
    time_limit_pt2: i64,
}

impl CaveNetwork {
    fn new() -> CaveNetwork {
        CaveNetwork {
            network: HashMap::<u16, Valve>::new(),
            important_valves: HashSet::<u16>::new(),
            shortest_routes: HashMap::<(u16, u16), i64>::new(),
            max_flow: 0,
            time_limit: 30,
            time_limit_pt2: 26,
        }
    }

    fn code_to_u16(code: &str) -> u16 {
        let mut temp = code.bytes();
        let mut index: u16 = (temp.next().unwrap() as u16) << 8;
        index += temp.next().unwrap() as u16;
        index
    }

    fn _u16_to_code(hash: u16) -> String {
        [(hash >> 8) as u8 as char, hash as u8 as char]
            .iter()
            .collect()
    }

    fn set_max_flow(&mut self) {
        self.max_flow = self
            .important_valves
            .iter()
            .map(|index| self.network[index].flow_rate)
            .sum();
    }

    fn filter_important(&mut self) {
        self.important_valves = self
            .network
            .iter()
            .filter(|(_, valve)| valve.flow_rate != 0)
            .map(|(index, _)| *index)
            .collect();
    }

    fn shortest_route_length(&mut self, from: u16, to: u16) -> i64 {
        if from == to {
            return 0;
        }
        if let Some(val) = self.shortest_routes.get(&(from, to)) {
            return *val;
        }
        if let Some(val) = self.shortest_routes.get(&(to, from)) {
            return *val;
        }

        let mut visited = HashSet::from([from]);
        let mut steps_taken = 0;
        let mut queue = HashSet::<u16>::from_iter(self.network[&from].connections.iter().copied());

        'outer: loop {
            steps_taken += 1;

            let this_step_queue: Vec<u16> = queue.drain().collect();
            for next_step in this_step_queue {
                if next_step == to {
                    break 'outer;
                }
                visited.insert(next_step);
                for new_candidate in self.network[&next_step].connections.iter().copied() {
                    if !visited.contains(&new_candidate) {
                        queue.insert(new_candidate);
                    }
                }
            }
        }

        self.shortest_routes.insert((from, to), steps_taken);
        steps_taken
    }

    fn travelling_salesman_opens_valves(&mut self, from: u16) -> i64 {
        let closed_valves = self.important_valves.clone();
        let max_loss: i64 = closed_valves
            .iter()
            .map(|valve_index| self.network.get(valve_index).unwrap().flow_rate)
            .sum::<i64>()
            * self.time_limit;

        let min_loss = self.tsov_util(closed_valves, from, 0, 0, max_loss);

        max_loss - min_loss
    }

    fn tsov_util(
        &mut self,
        closed: HashSet<u16>,
        current_valve: u16,
        time_spent: i64,
        current_loss: i64,
        min_loss: i64,
    ) -> i64 {
        if min_loss < current_loss {
            return min_loss;
        }

        let loss_per_minute: i64 = closed
            .iter()
            .map(|valve_index| self.network.get(valve_index).unwrap().flow_rate)
            .sum();

        let none_closed = current_loss + loss_per_minute * (self.time_limit - time_spent);

        let mut best_case = min_loss;
        if none_closed < best_case {
            best_case = none_closed;
        }

        for closed_yet in closed.iter() {
            let time_to_close = self.shortest_route_length(current_valve, *closed_yet) + 1;
            if time_to_close + time_spent > self.time_limit {
                continue;
            }
            let this_loss = current_loss + loss_per_minute * time_to_close;
            let mut this_closed = closed.clone();
            this_closed.remove(closed_yet);

            let this_case = self.tsov_util(
                this_closed,
                *closed_yet,
                time_to_close + time_spent,
                this_loss,
                best_case,
            );

            if this_case < best_case {
                best_case = this_case;
            }
        }

        best_case
    }

    fn me_and_my_elephant_gf(&mut self, start: u16) -> i64 {
        let me = SalesmanState {
            goal: start,
            eta: 0,
        };
        let elephant = SalesmanState {
            goal: start,
            eta: 0,
        };
        let opened = HashSet::<u16>::new();

        self.mameg_util(opened, me, elephant, 0, 0, 0)
    }

    fn mameg_util(
        &mut self,
        opened: HashSet<u16>,
        my_state: SalesmanState,
        elephant_state: SalesmanState,
        passed_time: i64,
        accumulated_flow: i64,
        best_result: i64,
    ) -> i64 {
        if self.time_limit_pt2 == passed_time {
            return accumulated_flow;
        }
        if accumulated_flow + self.max_flow * (self.time_limit_pt2 - passed_time) < best_result {
            return best_result;
        }

        let i_reached = my_state.eta == passed_time;
        let elephant_reached = elephant_state.eta == passed_time;
        let mut local_opened = opened;

        if i_reached {
            local_opened.insert(my_state.goal);
        }

        if elephant_reached {
            local_opened.insert(elephant_state.goal);
        }

        if i_reached || elephant_reached {
            let closed: Vec<u16> = self
                .important_valves
                .iter()
                .filter(|&value| !local_opened.contains(value))
                .copied()
                .collect();
            if !closed.is_empty() {
                let mut junction_best_result = best_result;

                if i_reached && elephant_reached {
                    for closed_yet_4me in closed.iter() {
                        for closed_yet_4el in closed.iter().rev() {
                            let my_new_state = SalesmanState {
                                goal: *closed_yet_4me,
                                eta: passed_time
                                    + self.shortest_route_length(my_state.goal, *closed_yet_4me)
                                    + 1,
                            };
                            let elephant_new_state = SalesmanState {
                                goal: *closed_yet_4el,
                                eta: passed_time
                                    + self.shortest_route_length(
                                        elephant_state.goal,
                                        *closed_yet_4el,
                                    )
                                    + 1,
                            };
                            let this_result = self.mameg_util(
                                local_opened.clone(),
                                my_new_state,
                                elephant_new_state,
                                passed_time,
                                accumulated_flow,
                                junction_best_result,
                            );
                            junction_best_result = junction_best_result.max(this_result);
                        }
                    }
                } else if i_reached {
                    for closed_yet in closed {
                        let my_new_state = SalesmanState {
                            goal: closed_yet,
                            eta: passed_time
                                + self.shortest_route_length(my_state.goal, closed_yet)
                                + 1,
                        };
                        let this_result = self.mameg_util(
                            local_opened.clone(),
                            my_new_state,
                            elephant_state,
                            passed_time,
                            accumulated_flow,
                            junction_best_result,
                        );
                        junction_best_result = junction_best_result.max(this_result);
                    }
                } else {
                    // elephant reached
                    for closed_yet in closed {
                        let elephant_new_state = SalesmanState {
                            goal: closed_yet,
                            eta: passed_time
                                + self.shortest_route_length(elephant_state.goal, closed_yet)
                                + 1,
                        };
                        let this_result = self.mameg_util(
                            local_opened.clone(),
                            my_state,
                            elephant_new_state,
                            passed_time,
                            accumulated_flow,
                            junction_best_result,
                        );
                        junction_best_result = junction_best_result.max(this_result);
                    }
                }
                return junction_best_result;
            }
        }

        let flow_per_minute: i64 = local_opened
            .iter()
            .map(|index| self.network[index].flow_rate)
            .sum();

        self.mameg_util(
            local_opened,
            my_state,
            elephant_state,
            passed_time + 1,
            accumulated_flow + flow_per_minute,
            best_result,
        )
    }
}

#[derive(Clone, Copy)]
struct SalesmanState {
    goal: u16,
    eta: i64,
}

#[derive(Debug)]
struct Valve {
    connections: Vec<u16>,
    flow_rate: i64,
}
