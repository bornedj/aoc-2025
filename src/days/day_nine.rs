#[derive(Debug)]
struct Coordinate {
    x: isize,
    y: isize,
}

trait Area {
    fn area(&self, other: &Self) -> usize;
}

impl Area for Coordinate {
    fn area(&self, other: &Self) -> usize {
        // he inverts the coords to be a grid that indexes like a matrix
        // which makes the width and height inclusive
        let width = (self.x - other.x).abs() as usize + 1;
        let height = (self.y - other.y).abs() as usize + 1;

        width * height
    }
}

pub fn puzzle_one(input: &str) -> usize {
    let coordinates = parse_input(input);
    find_greatest_area(coordinates) as usize
}

fn find_greatest_area(coordinates: Vec<Coordinate>) -> usize {
    let mut greatest_area = 0;

    for coord in coordinates.iter() {
        let curr_greatest = coordinates.iter().map(|inner_coord| {
            inner_coord.area(coord)
            
        }).max().expect("should be computed areas");

        if curr_greatest > greatest_area {
            greatest_area = curr_greatest;
        }
    }
    greatest_area
}

fn parse_input(input: &str) -> Vec<Coordinate> {
    input.lines().map(|line| {
        let mut parts = line.split(',');
        let x = parts.next().expect("expect an x coordinate").parse::<isize>().expect("x should be parseable");
        let y = parts.next().expect("expect an y coordinate").parse::<isize>().expect("y should be parseable");
        Coordinate  {
            x, y
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#;

    #[test]
    fn test_puzzle_one() {
        assert_eq!(50, puzzle_one(EXAMPLE_INPUT));
    }
}
