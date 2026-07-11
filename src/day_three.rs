use std::cmp::Reverse;

pub fn process_day_three_puzzle_one(input: String) -> u32 {
    input.lines().map(|line| process_bank(line.to_string())).sum()
}

fn process_bank(s: String) -> u32 {
    // get left digit
    let mut chars = s.chars();
    chars.next_back(); // left digit cannot be the last
    let left_digit_arr: Vec<u32> = chars.map(|c| c.to_digit(10).expect("inputs must be parseable")).collect();
    let (index, left_digit) = left_digit_arr
        .iter()
        .enumerate()
        .max_by_key(|(i, value)| (*value, Reverse(*i)))
        .unwrap();
    let right_digit_slice = &s[(index + 1)..];
    let right_digit = right_digit_slice.chars().map(|c| c.to_digit(10).expect("inputs must be parseable")).max().unwrap();
    let string_num = left_digit.to_string() + &right_digit.to_string();
    string_num.parse().expect("should compose two u8s as strings")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_bank() {
        assert_eq!(98, process_bank("987654321111111".to_string()));
        assert_eq!(89, process_bank("811111111111119".to_string()));
        assert_eq!(78, process_bank("234234234234278".to_string()));
        assert_eq!(92, process_bank("818181911112111".to_string()));
    }
}
