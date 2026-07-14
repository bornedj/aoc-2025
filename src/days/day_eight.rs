use std::collections::HashMap;

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32
}

trait ComputeDistance {
    fn compute_distance(&self, other: &Self) -> f64;
}

impl ComputeDistance for Coordinate {
    fn compute_distance(&self, other: &Self) -> f64 {
        f64::sqrt(((other.x - self.x).pow(2) + (other.y - self.y).pow(2) + (other.z - self.z).pow(2)).into())
    }
}


pub fn puzzle_one(input: &str) -> u32 {
    let coordinates = map_input_to_coordinates(input);
    let distance_map = coordinates_vec_to_distance_map(&coordinates);


    println!("{distance_map:?}");
    todo!()
}

fn coordinates_vec_to_distance_map<'a >(coordinates: &'a Vec<Coordinate>) -> HashMap<(&'a Coordinate, &'a Coordinate), f64> {
    let mut distance_map: HashMap<(&Coordinate, &Coordinate), f64> = HashMap::new();
    for i in 0..coordinates.len() {
        for coord in coordinates {
            if coord != &coordinates[i] {
                let distance: f64 = coord.compute_distance(&coordinates[i]);

                if !distance_map.contains_key(&(&coordinates[i], coord)) && 
                    !distance_map.contains_key(&(coord, &coordinates[i]))
                {
                    distance_map.insert((&coordinates[i], coord), distance);
                }
            }
        }
    }
    distance_map
}

fn map_input_to_coordinates(input: &str) -> Vec<Coordinate> {
    input.lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse::<i32>().expect("Must be a number");
            let y = parts.next().unwrap().parse::<i32>().expect("Must be a number");
            let z = parts.next().unwrap().parse::<i32>().expect("Must be a number");
            Coordinate {
                x, 
                y,
                z,
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
