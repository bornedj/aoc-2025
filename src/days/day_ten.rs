use std::collections::HashSet;

type WiringSchematics = Vec<Vec<u32>>;
type JoltageRequirement = Vec<u32>;
#[derive(Debug, PartialEq)]
struct InitProcedure {
    state: Vec<bool>,
    target: Vec<bool>,
    buttons: WiringSchematics,
    joltage_requirement: JoltageRequirement,
}

fn push_button(mut state: Vec<bool>, button: &Vec<u32>) -> Vec<bool> {
    for index in button {
        let i: usize = *index as usize;
        state[i] = !state[i];
    }
    state
}

pub fn puzzle_one(input: &str) -> u32 {
    let init_procedures = parse_init_procedures(input);

    init_procedures.iter().map(|procedure| {
        let mut set = HashSet::<Vec<bool>>::new();
        set.insert(procedure.state.clone());
        let mut count = 0;
        loop {
            set = set.into_iter().flat_map(|state| {
                procedure.buttons.iter().map(move |button| {
                    push_button(state.clone(), button)
                })
            }).collect();
            count += 1;
            if set.contains(&procedure.target) {
                break;
            }
        }

        count
    }).sum()

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
    let str_wiring_schematic =
        &line[(line.find('(').expect("must be end of light diagram"))
            ..line
                .find('{')
                .expect("must be beginning of joltage requirement")]
            .trim();
     str_wiring_schematic
        .split(' ')
        .map(|parts| parts.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn parse_joltage(line: &str) -> JoltageRequirement {
    let str_joltage_requirement = &line[(line
        .find('{')
        .expect("must be beginning joltage requirement") + 1)
        ..line.find('}').expect("must be end of joltage requirement")];
         str_joltage_requirement
            .split(',')
            .map(|str| str.parse::<u32>().expect("should be a parseable int within joltage braces"))
            .collect()
}

fn parse_init_procedures(input: &str) -> Vec<InitProcedure> {
    input
        .lines()
        .map(|line| {
            let target = parse_target_light_state(line);
            let wiring_schematic = parse_wiring_schematic(line);
            let joltage_requirement = parse_joltage(line);

            InitProcedure {
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
    use std::vec;

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
        let first = InitProcedure {
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

        let second = InitProcedure {
            state: vec![false, false, false, false, false],
            target: vec![false, false, false, true, false],
            buttons: vec![
                vec![0,2,3,4],
                vec![2,3],
                vec![0,4],
                vec![0,1,2,],
                vec![1,2,3,4],
            ],
            joltage_requirement: vec![7,5,12,7,2],
        };

        assert_eq!(first, parse_init_procedures(EXAMPLE_INPUT)[0]);
        assert_eq!(second, parse_init_procedures(EXAMPLE_INPUT)[1]);
    }

    #[test]
    fn test_push_button() {
        let procedure = InitProcedure {
            state: vec![false, false],
            target: vec![true, true],
            buttons: vec![vec![0, 1]],
            joltage_requirement: Vec::<u32>::new(),
        };
        let result = push_button(procedure.state.clone(), &procedure.buttons[0]);
        assert_eq!(vec![true, true], result);
    }
}
