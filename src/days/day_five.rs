pub fn puzzle_one(input: &str) -> usize {
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#;

    #[test]
    fn puzzle_one_example_input() {
        assert_eq!(puzzle_one(EXAMPLE_INPUT), 3);
    }
}
