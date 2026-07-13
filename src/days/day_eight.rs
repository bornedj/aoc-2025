#[derive(Debug)]
struct Coordinate {
    x: f64,
    y: f64,
    z: f64
}

trait ComputeDistance {
    fn compute_distance(&self, other: &Self) -> f64;
}

impl ComputeDistance for Coordinate {
    fn compute_distance(&self, other: &Self) -> f64 {
        let exponent: f64 = 2 as f64;
        f64::sqrt((other.x - self.x).powf(exponent) + (other.y - self.y).powf(exponent) + (other.z - self.z).powf(exponent))
    }
}

pub fn puzzle_one(input: &str) -> u32 {
    let coordinates = map_input_to_coordinates(input);
    let min_distance = coordinates[1..]
        .iter()
        .map(|coord| coord.compute_distance(&coordinates[0]) as u64)
        .min()
        .unwrap();

    println!("{coordinates:?}");
    println!("minimum distance from first coord: {min_distance}");
    todo!()
}

fn map_input_to_coordinates(input: &str) -> Vec<Coordinate> {
    input.lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<u32>().expect("Must be a number");
            let y = parts.next().unwrap().parse::<u32>().expect("Must be a number");
            let z = parts.next().unwrap().parse::<u32>().expect("Must be a number");
            Coordinate {
                x: x as f64, 
                y: y as f64,
                z: z as f64,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
"#;

    #[test]
    fn test_puzzle_one() {
        assert_eq!(40, puzzle_one(EXAMPLE_INPUT));
    }
}
