use std::{collections::HashMap, usize};

#[derive(Debug, PartialEq, Hash, Eq, Copy, Clone)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

trait ComputeDistance {
    fn compute_distance(&self, other: &Self) -> i64;
}

impl ComputeDistance for Coordinate {
    fn compute_distance(&self, other: &Self) -> i64 {
        i64::isqrt(
            (self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2),
        )
    }
}

pub fn puzzle_one(input: &str, junction_count: usize) -> u32 {
    let coordinates = map_input_to_coordinates(input);
    let sorted_coord_distance = coordinates_vec_to_sorted_distance_vec(&coordinates);
    let circuits = create_circuits(sorted_coord_distance, junction_count);

    let mut counts: Vec<usize> = circuits
        .values()
        .fold(HashMap::new(), |mut acc, &id| {
            *acc.entry(id).or_insert(0) += 1;
            acc
        })
        .into_values()
        .collect();

    counts.sort_by(|a, b| b.cmp(a));
    counts.iter().take(3).map(|&count| count as u32).product()
}

pub fn puzzle_two(input: &str) -> i64 {
    let coordinates = map_input_to_coordinates(input);
    let sorted_coord_distance = coordinates_vec_to_sorted_distance_vec(&coordinates);
    let last_junction = find_last_junction(&coordinates, sorted_coord_distance);
    last_junction.0.x * last_junction.1.x
}

fn find_last_junction<'a>(
    all_coordinates: &Vec<Coordinate>,
    distance_map: Vec<((&'a Coordinate, &'a Coordinate), i64)>,
) -> (&'a Coordinate, &'a Coordinate) {
    let mut circuit_map: HashMap<&Coordinate, usize> = HashMap::new();
    for (coords, _) in distance_map.iter() {
        let zero_index = circuit_map.get(coords.0).copied();
        let one_index = circuit_map.get(coords.1).copied();
        match (zero_index, one_index) {
            (None, None) => {
                if let Some(max_index) = circuit_map.values().max().copied() {
                    circuit_map.insert(coords.0, max_index + 1);
                    circuit_map.insert(coords.1, max_index + 1);
                } else {
                    circuit_map.insert(coords.0, 0);
                    circuit_map.insert(coords.1, 0);
                }
            }
            (Some(zero_index), None) => {
                circuit_map.insert(coords.1, zero_index);
            }
            (None, Some(one_index)) => {
                circuit_map.insert(coords.0, one_index);
            }
            (Some(zero_index), Some(one_index)) => {
                for circuit_id in circuit_map.values_mut() {
                    if circuit_id == &zero_index {
                        *circuit_id = one_index;
                    }
                }
            }
        }
        let first = circuit_map.values().next().unwrap();
        if all_coordinates.iter().all(|coord| {
            if let Some(index) = circuit_map.get(coord) {
                return index == first;
            } else {
                false
            }
        }) {
            println!("{:?}", coords);
            return *coords;
        }
    }
    panic!("didn't find last connection")
}

fn create_circuits<'a>(
    sorted_coord_distance: Vec<((&'a Coordinate, &'a Coordinate), i64)>,
    junction_count: usize,
) -> HashMap<&'a Coordinate, usize> {
    let mut circuits: HashMap<&'a Coordinate, usize> = HashMap::new();

    sorted_coord_distance
        .iter()
        .take(junction_count)
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
                }
                (Some(zero_index), None) => {
                    circuits.insert(coords.1, zero_index);
                }
                (None, Some(one_index)) => {
                    circuits.insert(coords.0, one_index);
                }
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
) -> Vec<((&Coordinate, &Coordinate), i64)> {
    let mut distance_map: HashMap<(&Coordinate, &Coordinate), i64> = HashMap::new();
    for i in 0..coordinates.len() {
        for coord in coordinates {
            if coord != &coordinates[i] {
                let distance: i64 = coord.compute_distance(&coordinates[i]);

                if !distance_map.contains_key(&(&coordinates[i], coord))
                    && !distance_map.contains_key(&(coord, &coordinates[i]))
                {
                    distance_map.insert((&coordinates[i], coord), distance);
                }
            }
        }
    }

    // sorting iter by distance
    let mut vec_map: Vec<((&Coordinate, &Coordinate), i64)> = distance_map
        .iter()
        .map(|(&coords, &distance)| (coords, distance))
        .collect();
    vec_map.sort_by(|a, b| a.1.cmp(&b.1));
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
                .parse::<i64>()
                .expect("Must be a number");
            let y = parts
                .next()
                .unwrap()
                .parse::<i64>()
                .expect("Must be a number");
            let z = parts
                .next()
                .unwrap()
                .parse::<i64>()
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
        assert_eq!(40, puzzle_one(EXAMPLE_INPUT, 10));
    }

    #[test]
    fn test_puzzle_two() {
        assert_eq!(25272i32, puzzle_two(EXAMPLE_INPUT));
    }
}
