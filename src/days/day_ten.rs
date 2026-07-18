struct InitProcedure {
    light_diagram: Vec<bool>,
    wiring_schematic: Vec<Vec<usize>>,
    joltage_requirement: [usize],
}

pub fn puzzle_one(input: &str) -> usize {
    todo!()
}

fn parse_input(input: &str) -> Vec<InitProcedure> {
    // input.lines()
    //     .map(f)
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT = r#"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"#;

    #[test]
    fn test_puzzle_one() {
        assert_eq!(7, puzzle_one(EXAMPLE_INPUT));
    }
}
