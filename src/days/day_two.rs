use std::ops::RangeInclusive;

pub fn process_day_two(input: &str) -> i64 {
    let string_ranges = parse_delimited_string(input);
    let ranges = string_ranges.into_iter().map(calculate_ranges);
    ranges.map(process_ranges).sum()
}

pub fn process_day_two_prob_two(input: &str) -> i64 {
    let string_ranges = parse_delimited_string(input);
    let ranges = string_ranges.into_iter().map(calculate_ranges);
    ranges.flatten().filter(|num| contains_any_length_repeated_substr(num.to_string())).sum()
}

fn parse_delimited_string(s: &str) -> Vec<&str> {
    s.split(',').collect()
}

fn calculate_ranges(s: &str) -> RangeInclusive<i64> {
    let mut iter = s.split('-');
    let first = iter.next().expect("Must be two numbers per comma delimination").parse().expect("All inputs should parse to ints");
    let last = iter.next().expect("Must be two numbers per comma delimination").parse().expect("All inputs should parse to ints");
    first..=last
}

fn process_ranges(range: RangeInclusive<i64>) -> i64 {
    range
        .filter(|num| contains_repeated_substr(num.to_string()))
        .sum()
}

fn contains_repeated_substr(input: String) -> bool {
    if input.len() % 2 != 0 {
        return false
    }
    let half_way_point = input.len() / 2;
    let first_half = &input[0..half_way_point];
    let second_half = &input[half_way_point..];
    first_half == second_half
}

fn contains_any_length_repeated_substr(s: String) -> bool {
    let max_substr_length = s.len() / 2; // longest substr can be two pairs
    // for a substr to be a possible match,
    // the length of the string must be divisble by the substr length
    let possible_substr_lengths: Vec<usize> = (1..=max_substr_length).filter(|num| s.len() % num == 0).collect();
    possible_substr_lengths.into_iter()
        .filter(|&substr_len| {
            let mut chunks = s.as_bytes().chunks(substr_len);
            let all_equal = match chunks.next() {
                None => false,
                Some(first) => chunks.all(|chunk| first == chunk)
            };
            all_equal
        }).count() >= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const FULL_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn parse_sample_input() {
        assert_eq!(vec!["11-22", "95-115", "998-1012", "1188511880-1188511890","222220-222224","1698522-1698528","446443-446449","38593856-38593862","565653-565659","824824821-824824827","2121212118-2121212124"], parse_delimited_string(FULL_INPUT));
    }

    #[test]
    fn test_calculate_ranges() {
        let input = "11-22";
        assert_eq!((11..=22), calculate_ranges(input));
    }

    #[test]
    fn test_count_repeated_substr() {
        assert_eq!(contains_repeated_substr("11".to_string()), true);
        assert_eq!(contains_repeated_substr("22".to_string()), true);
        assert_eq!(contains_repeated_substr("1010".to_string()), true);
        assert_eq!(contains_repeated_substr("1188511885".to_string()), true);
        assert_eq!(contains_repeated_substr("222222".to_string()), true);
        assert_eq!(contains_repeated_substr("446446".to_string()), true);
        assert_eq!(contains_repeated_substr("38593859".to_string()), true);
    }

    #[test]
    fn test_process() {
        assert_eq!(process_day_two(FULL_INPUT), 1227775554);
    }

    #[test]
    fn test_process_puzzle_two() {
        assert_eq!(process_day_two_prob_two(FULL_INPUT), 4174379265);
    }

    #[test]
    fn any_length_substr() {
        assert!(contains_any_length_repeated_substr("101101".to_string()));
        assert!(contains_any_length_repeated_substr("10001000".to_string()));
        assert!(contains_any_length_repeated_substr("111".to_string()));
        assert!(contains_any_length_repeated_substr("121212".to_string()));
    }
}
