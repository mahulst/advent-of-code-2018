use std::str::FromStr;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub left: char,
    pub right: char,
}

#[derive(Debug, PartialEq)]
pub struct NodeDeps {
    pub id: char,
    pub deps: Vec<char>,
}

type DepList = HashMap<char, HashSet<char>>;

impl FromStr for Node {
    type Err = ();

    fn from_str(input: &str) -> Result<Node, ()> {
        let re =
            Regex::new(r"Step (?P<left>.) must be finished before step (?P<right>.) .*$").unwrap();


        match re.captures(input) {
            Some(caps) => Ok(Node {
                left: caps["left"].parse().unwrap(),
                right: caps["right"].parse().unwrap(),
            }),
            None => Err(())
        }
    }
}

pub fn order_build_steps_sleigh(deps: &mut DepList) -> String {
    let mut result = String::new();
    while !deps.is_empty() {
        let next_step = find_next_available_step(&deps).unwrap();
        result.push(next_step);
        remove_step_from_all_deps(next_step, deps);
        remove_step(next_step, deps);
    }

    result
}


const CHARS: [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];


pub fn get_time_for_step(c: char, base_time: u32) -> u32 {
    let time = CHARS.iter().position(|char1| c == *char1).unwrap();

    (time as u32) + 1 + base_time
}

pub fn build_sleigh(deps: &mut DepList, workers_amount: u8, base_time: u32) -> u32 {
    let mut time = 0;
    let mut workers: Vec<(Option<char>, u32)> = vec![(None, 0); workers_amount as usize];

    while !deps.is_empty() || workers.iter().any(|(working, _)| working != &None) {
        workers
            .iter_mut()
            .for_each(|worker| {
                match worker.0 {
                    Some(c) => {
                        worker.1 -= 1;
                        if worker.1 == 0 {
                            remove_step_from_all_deps(c, deps);

                            match find_next_available_step(&deps) {
                                Some(next_step) => {
                                    remove_step(next_step, deps);
                                    let seconds_needed = get_time_for_step(next_step, base_time);
                                    worker.0 = Some(next_step);
                                    worker.1 = seconds_needed;
                                }
                                None => {
                                    worker.0 = None
                                }
                            };
                        }
                    }
                    None => {
                        match find_next_available_step(&deps) {
                            Some(next_step) => {
                                remove_step(next_step, deps);
                                let seconds_needed = get_time_for_step(next_step, base_time);
                                worker.0 = Some(next_step);
                                worker.1 = seconds_needed;
                            }
                            None => {}
                        };
                    }
                }
            });

        time += 1;
    }


    time - 1
}

pub fn find_first_char_in_set(set: &HashSet<char>) -> Option<char> {
    set
        .iter()
        .fold(None, |acc, next| {
            match acc {
                Some(c) => {
                    if c > *next {
                        Some(*next)
                    } else {
                        Some(c)
                    }
                }
                None => Some(*next)
            }
        })
}

pub fn remove_step(c: char, deps: &mut DepList) {
    deps.remove(&c);
}

pub fn remove_step_from_all_deps(c: char, deps: &mut DepList) {
    deps
        .iter_mut()
        .for_each(|(_id, set)| {
            set.remove(&c);
        })
}

pub fn find_available_steps(deps: &DepList) -> Vec<char> {
    deps
        .iter()
        .filter_map(|(id, set)| {
            if set.len() == 0 {
                Some(*id)
            } else {
                None
            }
        }).collect()
}

pub fn find_next_available_step(deps: &DepList) -> Option<char> {
    let mut steps_with_no_deps: Vec<char> = find_available_steps(&deps);

    steps_with_no_deps.sort_unstable();

    match steps_with_no_deps.first() {
        Some(c) => Some(*c),
        None => None
    }
}

pub fn to_nodes(input: &str) -> DepList {
    let nodes: Vec<Node> = input
        .lines()
        .map(|l| l.parse().unwrap())
        .collect();

    let mut deps: DepList = HashMap::new();
    nodes
        .iter()
        .for_each(|node| {
            let d = deps
                .entry(node.right)
                .or_insert(HashSet::new());

            d.insert(node.left);

            deps.entry(node.left).or_insert(HashSet::new());
        });

    deps
}

#[cfg(test)]
mod tests {
    use crate::Node;
    use crate::to_nodes;
    use crate::find_next_available_step;
    use crate::order_build_steps_sleigh;
    use crate::build_sleigh;

    #[test]
    fn it_should_parse_input() {
        // Arrange
        let input = "Step C must be finished before step A can begin.";

        // Act
        let result: Node = input.parse().unwrap();

        // Assert
        assert_eq!(result, Node { left: 'C', right: 'A' });
    }

    #[test]
    fn it_should_order_the_steps_correctly() {
        // Arrange
        let input = "Step C must be finished before step A can begin.
        Step C must be finished before step F can begin.
        Step A must be finished before step B can begin.
        Step A must be finished before step D can begin.
        Step B must be finished before step E can begin.
        Step D must be finished before step E can begin.
        Step F must be finished before step E can begin.";

        // Act
        let deps = to_nodes(input);
        let result = find_next_available_step(&deps).unwrap();

        // Assert
        assert_eq!(result, 'C');
    }


    #[test]
    fn it_should_put_steps_in_correct_order() {
        // Arrange
        let input = "Step C must be finished before step A can begin.
        Step C must be finished before step F can begin.
        Step A must be finished before step B can begin.
        Step A must be finished before step D can begin.
        Step B must be finished before step E can begin.
        Step D must be finished before step E can begin.
        Step F must be finished before step E can begin.";

        // Act
        let mut deps = to_nodes(input);
        let result = order_build_steps_sleigh(&mut deps);

        // Assert
        assert_eq!(result, "CABDFE");
    }

    #[test]
    fn it_should_calc_time_to_build_sleigh() {
        // Arrange
        let input = "Step C must be finished before step A can begin.
        Step C must be finished before step F can begin.
        Step A must be finished before step B can begin.
        Step A must be finished before step D can begin.
        Step B must be finished before step E can begin.
        Step D must be finished before step E can begin.
        Step F must be finished before step E can begin.";

        // Act
        let mut deps = to_nodes(input);
        let result = build_sleigh(&mut deps, 2, 0);

        // Assert
        assert_eq!(result, 15);
    }
}
