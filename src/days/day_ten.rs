use std::collections::HashSet;

use rayon::prelude::*;

type WiringSchematics = Vec<Vec<usize>>;
type JoltageRequirement = Vec<usize>;

mod puzzle_one {
    #[derive(Debug, PartialEq)]
    pub struct InitProcedure {
        pub state: Vec<bool>,
        pub target: Vec<bool>,
        pub buttons: super::WiringSchematics,
        pub joltage_requirement: super::JoltageRequirement,
    }
    pub fn push_button(mut state: Vec<bool>, button: &Vec<usize>) -> Vec<bool> {
        for index in button {
            state[*index] = !state[*index];
        }
        state
    }
}

#[derive(Debug)]
struct JoltageProcedure {
    state: JoltageRequirement,
    target: JoltageRequirement,
    buttons: WiringSchematics,
}

pub fn puzzle_two(input: &str) -> usize {
    let procedures = parse_joltage_procedures(input);

    procedures
        .par_iter()
        .map(|procedure| solve_joltage(procedure))
        .sum()
}

fn solve_joltage(procedure: &JoltageProcedure) -> usize {
    let mut best = usize::MAX;

    let mut presses = vec![0usize; procedure.buttons.len()];

    search_buttons(
        procedure,
        0,
        &mut presses,
        &mut best,
    );

    best
}


fn search_buttons(
    procedure: &JoltageProcedure,
    button_index: usize,
    presses: &mut Vec<usize>,
    best: &mut usize,
) {
    let current_presses: usize = presses.iter().sum();

    // Already worse than a known solution
    if current_presses >= *best {
        return;
    }


    // All buttons assigned, test solution
    if button_index == procedure.buttons.len() {
        let mut state = procedure.state.clone();

        for (button, count) in procedure.buttons.iter().zip(presses.iter()) {
            for index in button {
                state[*index] += count;
            }
        }


        if state == procedure.target {
            *best = current_presses;
        }

        return;
    }


    /*
       A button cannot be pressed more times than the
       largest remaining joltage requirement.
    */
    let max_press = procedure
        .target
        .iter()
        .copied()
        .max()
        .unwrap();


    for count in 0..=max_press {
        presses[button_index] = count;

        search_buttons(
            procedure,
            button_index + 1,
            presses,
            best,
        );
    }

    presses[button_index] = 0;
}

pub fn puzzle_one(input: &str) -> usize {
    let init_procedures = parse_init_procedures(input);

    init_procedures
        .iter()
        .map(|procedure| {
            let mut set = HashSet::<Vec<bool>>::new();
            set.insert(procedure.state.clone());
            let mut count = 0;
            loop {
                set = set
                    .into_iter()
                    .flat_map(|state| {
                        procedure
                            .buttons
                            .iter()
                            .map(move |button| puzzle_one::push_button(state.clone(), button))
                    })
                    .collect();
                count += 1;
                if set.contains(&procedure.target) {
                    break;
                }
            }

            count
        })
        .sum()
}

fn parse_target_light_state(line: &str) -> Vec<bool> {
    let str_light_diagram = &line[..line.find(']').expect("must be light diagram ]")];
    str_light_diagram[1..]
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("unexpected light diagram char"),
        })
        .collect()
}

fn parse_wiring_schematic(line: &str) -> WiringSchematics {
    let str_wiring_schematic = &line[(line.find('(').expect("must be end of light diagram"))
        ..line
            .find('{')
            .expect("must be beginning of joltage requirement")]
        .trim();
    str_wiring_schematic
        .split(' ')
        .map(|parts| parts.chars().filter_map(|c| c.to_digit(10)).map(|u_32| u_32 as usize).collect())
        .collect()
}

fn parse_joltage(line: &str) -> JoltageRequirement {
    let str_joltage_requirement = &line[(line
        .find('{')
        .expect("must be beginning joltage requirement")
        + 1)
        ..line.find('}').expect("must be end of joltage requirement")];
    str_joltage_requirement
        .split(',')
        .map(|str| {
            str.parse::<usize>()
                .expect("should be a parseable int within joltage braces")
        })
        .collect()
}

fn parse_joltage_procedures(input: &str) -> Vec<JoltageProcedure> {
    input
        .lines()
        .map(|line| {
            let target = parse_joltage(line);
            let buttons = parse_wiring_schematic(line);
            let state = vec![0; target.len()];

            JoltageProcedure {
                state,
                target,
                buttons,
            }
        })
        .collect()
}

fn parse_init_procedures(input: &str) -> Vec<puzzle_one::InitProcedure> {
    input
        .lines()
        .map(|line| {
            let target = parse_target_light_state(line);
            let wiring_schematic = parse_wiring_schematic(line);
            let joltage_requirement = parse_joltage(line);

            puzzle_one::InitProcedure {
                state: vec![false; target.len()],
                target,
                buttons: wiring_schematic,
                joltage_requirement,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    const EXAMPLE_INPUT: &str = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn test_puzzle_one() {
        assert_eq!(7, puzzle_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_parse_input() {
        let first = puzzle_one::InitProcedure {
            state: vec![false, false, false, false],
            target: vec![false, true, true, false],
            buttons: vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1],
            ],
            joltage_requirement: vec![3, 5, 4, 7],
        };

        let second = puzzle_one::InitProcedure {
            state: vec![false, false, false, false, false],
            target: vec![false, false, false, true, false],
            buttons: vec![
                vec![0, 2, 3, 4],
                vec![2, 3],
                vec![0, 4],
                vec![0, 1, 2],
                vec![1, 2, 3, 4],
            ],
            joltage_requirement: vec![7, 5, 12, 7, 2],
        };

        assert_eq!(first, parse_init_procedures(EXAMPLE_INPUT)[0]);
        assert_eq!(second, parse_init_procedures(EXAMPLE_INPUT)[1]);
    }

    #[test]
    fn test_push_button() {
        let procedure = puzzle_one::InitProcedure {
            state: vec![false, false],
            target: vec![true, true],
            buttons: vec![vec![0, 1]],
            joltage_requirement: Vec::<usize>::new(),
        };
        let result = puzzle_one::push_button(procedure.state.clone(), &procedure.buttons[0]);
        assert_eq!(vec![true, true], result);
    }

    #[test]
    fn test_puzzle_two() {
        assert_eq!(33, puzzle_two(EXAMPLE_INPUT));
    }

    #[test]
    fn test_push_joltage_button() {
        let procedure = JoltageProcedure {
            state: vec![0; 4],
            target: vec![1; 4],
            buttons: vec![vec![0, 2]],
        };
        assert_eq!(
            vec![1, 0, 1, 0],
            push_button(procedure.state.clone(), &procedure.buttons[0])
        );
    }
}
