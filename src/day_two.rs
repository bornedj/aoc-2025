fn process() {}

fn parse_delimited_string(s: &str) -> Vec<&str> {
    s.split(',').collect()
}

fn compute_range(s: &str) -> Vec<u16> {
    s.split('-').map(|str_num| str_num.parse::<u16>().expect("All inputs should parse to ints")).collect()
}

fn check_doubles() {}

#[cfg(test)]
mod tests {
    use std::vec;

use crate::day_two::compute_range;

use super::parse_delimited_string;

    #[test]
    fn parse_sample_input() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(vec!["11-22", "95-115", "998-1012", "1188511880-1188511890","222220-222224","1698522-1698528","446443-446449","38593856-38593862","565653-565659","824824821-824824827","2121212118-2121212124"], parse_delimited_string(input));
    }

    #[test]
    fn test_compute_range() {
        let input = "11-22";
        assert_eq!(vec![11, 22], compute_range(input));
    }
}
