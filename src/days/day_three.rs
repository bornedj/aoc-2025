use std::cmp::Reverse;

pub fn process_day_three_puzzle_one(input: String) -> u32 {
    input.lines().map(|line| process_two_digit_bank(line.to_string())).sum()
}

pub fn process_day_three_puzzle_two(input: String) -> u64 {
    input.lines().map(|line| process_twelve_digit_bank(line.to_string())).sum()
}

fn process_two_digit_bank(s: String) -> u32 {
    // get left digit
    let mut chars = s.chars();
    chars.next_back(); // left digit cannot be the last
    let left_digit_arr: Vec<u32> = chars.map(unsafe_char_map).collect();
    let (index, left_digit) = left_digit_arr
        .iter()
        .enumerate()
        .max_by_key(|(i, value)| (*value, Reverse(*i)))
        .unwrap();
    let right_digit_slice = &s[(index + 1)..];
    let right_digit = right_digit_slice.chars().map(unsafe_char_map).max().unwrap();
    let string_num = left_digit.to_string() + &right_digit.to_string();
    string_num.parse().expect("should compose two u8s as strings")
}

fn process_twelve_digit_bank(s: String) -> u64 {
    let chars: Vec<char> = s.chars().collect();
    let mut digit_list: Vec<u32> = Vec::new();
    let mut start = 0;
    let mut remaining_digit_count = 12;
    let mut end = chars.len() - remaining_digit_count;

    while remaining_digit_count > 0 {

        let (digit_index, digit) = chars[start..=end]
            .iter()
            .copied()
            .map(unsafe_char_map)
            .enumerate()
            .max_by_key(|(i, value)| (*value, Reverse(*i)))
            .unwrap();
        digit_list.push(digit);

        start = start + digit_index + 1;
        remaining_digit_count -= 1;
        end = chars.len() - remaining_digit_count;
    }

    digit_list.iter().map(|d| d.to_string()).collect::<String>().parse().expect("should be parseable")
}

fn unsafe_char_map(c: char) -> u32 {
    c.to_digit(10).expect("inputs must be parseable")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_bank_2_digit() {
        assert_eq!(98, process_two_digit_bank("987654321111111".to_string()));
        assert_eq!(89, process_two_digit_bank("811111111111119".to_string()));
        assert_eq!(78, process_two_digit_bank("234234234234278".to_string()));
        assert_eq!(92, process_two_digit_bank("818181911112111".to_string()));
    }

    #[test]
    fn test_process_bank_12_digit() {
        assert_eq!(987654321111, process_twelve_digit_bank("987654321111111".to_string()));
        assert_eq!(811111111119, process_twelve_digit_bank("811111111111119".to_string()));
        assert_eq!(434234234278, process_twelve_digit_bank("234234234234278".to_string()));
        assert_eq!(888911112111, process_twelve_digit_bank("818181911112111".to_string()));
    }
}
