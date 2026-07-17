use std::{collections::HashMap, usize};

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

trait ComputeDistance {
    fn compute_distance(&self, other: &Self) -> f64;
}

impl ComputeDistance for Coordinate {
    fn compute_distance(&self, other: &Self) -> f64 {
        let floating_self_x = f64::from(self.x);
        let floating_self_y = f64::from(self.y);
        let floating_self_z = f64::from(self.z);
        let floating_other_x = f64::from(other.x);
        let floating_other_y = f64::from(other.y);
        let floating_other_z = f64::from(other.z);

        let exponent: f64 = f64::from(2);
        ((floating_self_x - floating_other_x).powf(exponent) + (floating_self_y - floating_other_y).powf(exponent) + (floating_self_z - floating_other_z).powf(exponent)).sqrt()
    }
}

pub fn puzzle_one(input: &str, junction_count: usize) -> u32 {
    let coordinates = map_input_to_coordinates(input);
    let sorted_coord_distance = coordinates_vec_to_sorted_distance_vec(&coordinates);
    let circuits = create_circuits(sorted_coord_distance);

    let mut counts: Vec<usize> = circuits.values().fold(HashMap::new(), |mut acc, &id| {
        *acc.entry(id).or_insert(0) += 1;
        acc
    })
    .into_values()
    .collect();

    counts.sort_by(|a,b| b.cmp(&a));
    counts.iter().take(junction_count).map(|&count| count as u32).product()
}

fn create_circuits<'a>(
    sorted_coord_distance: Vec<((&'a Coordinate, &'a Coordinate), f64)>,
) -> HashMap<&'a Coordinate, usize> {
    let mut circuits: HashMap<&'a Coordinate, usize> = HashMap::new();

    sorted_coord_distance
        .iter()
        .take(10)
        .for_each(|(coords, _)| {
            let zero_index = circuits.get(coords.0).copied();
            let one_index = circuits.get(coords.1).copied();
            match (zero_index, one_index) {
                (None, None) => {
                    if let Some(max_index) = circuits.values().max().copied() {
                        circuits.insert(coords.0, max_index + 1);
                        circuits.insert(coords.1, max_index + 1);
                    } else {
                        circuits.insert(coords.0, 0);
                        circuits.insert(coords.1, 0);
                    }
                },
                (Some(zero_index), None) => {
                    circuits.insert(coords.1, zero_index);
                },
                (None, Some(one_index)) => {
                    circuits.insert(coords.0, one_index);
                },
                (Some(zero_index), Some(one_index)) => {
                    for circuit_id in circuits.values_mut() {
                        if circuit_id == &zero_index {
                            *circuit_id = one_index;
                        }
                    }
                }
            }
        });
    circuits
}

fn coordinates_vec_to_sorted_distance_vec(
    coordinates: &Vec<Coordinate>,
) -> Vec<((&Coordinate, &Coordinate), f64)> {
    let mut distance_map: HashMap<(&Coordinate, &Coordinate), f64> = HashMap::new();
    for i in 0..coordinates.len() {
        for coord in coordinates {
            if coord != &coordinates[i] {
                let distance: f64 = coord.compute_distance(&coordinates[i]);

                if !distance_map.contains_key(&(&coordinates[i], coord))
                    && !distance_map.contains_key(&(coord, &coordinates[i]))
                {
                    distance_map.insert((&coordinates[i], coord), distance);
                }
            }
        }
    }

    // sorting iter by string length
    let mut vec_map: Vec<((&Coordinate, &Coordinate), f64)> = distance_map
        .iter()
        .map(|(&coords, &distance)| (coords, distance))
        .collect();
    vec_map.sort_by(|a, b| a.1.total_cmp(&b.1));
    vec_map
}

fn map_input_to_coordinates(input: &str) -> Vec<Coordinate> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts
                .next()
                .unwrap()
                .parse::<i32>()
                .expect("Must be a number");
            let y = parts
                .next()
                .unwrap()
                .parse::<i32>()
                .expect("Must be a number");
            let z = parts
                .next()
                .unwrap()
                .parse::<i32>()
                .expect("Must be a number");
            Coordinate { x, y, z }
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
        assert_eq!(40, puzzle_one(EXAMPLE_INPUT, 3));
    }
}
