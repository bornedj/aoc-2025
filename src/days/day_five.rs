use std::ops::RangeInclusive;

pub fn puzzle_one(input: &str) -> usize {
    let (ranges, ingredients) = parse_input(input);
    ingredients
        .iter()
        .filter(|ingredient| {
            ranges.iter().any(|range| range.contains(ingredient))
        })
        .count()
}

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, Vec<u64>) {
    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();

    input.lines().map(&str::trim).for_each(|line| {
        if line.contains('-') {
            let mut parts = line.split('-');
            let start = parts
                .next()
                .expect("expected start of range")
                .parse::<u64>()
                .expect("expected number");
            let end = parts
                .next()
                .expect("expected end of range")
                .parse::<u64>()
                .expect("expected number");
            ranges.push(start..=end);
        } else if line.len() > 0 {
            let ingredient = line.parse::<u64>().expect("expected number");
            ingredients.push(ingredient);
        }
    });
    (ranges, ingredients)
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
