pub mod puzz_one {
    pub fn puzzle_one(input: &str) -> usize {
        let coordinates = parse_input(input);
        find_greatest_area(coordinates)
    }

    fn find_greatest_area(coordinates: Vec<Coordinate>) -> usize {
        let mut greatest_area = 0;

        for coord in coordinates.iter() {
            let curr_greatest = coordinates
                .iter()
                .map(|inner_coord| inner_coord.area(coord))
                .max()
                .expect("should be computed areas");

            if curr_greatest > greatest_area {
                greatest_area = curr_greatest;
            }
        }
        greatest_area
    }
    fn parse_input(input: &str) -> Vec<Coordinate> {
        input
            .lines()
            .map(|line| {
                let mut parts = line.split(',');
                let x = parts
                    .next()
                    .expect("expect an x coordinate")
                    .parse::<isize>()
                    .expect("x should be parseable");
                let y = parts
                    .next()
                    .expect("expect an y coordinate")
                    .parse::<isize>()
                    .expect("y should be parseable");
                Coordinate { x, y }
            })
        .collect()
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
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
            let width = (self.x - other.x).unsigned_abs() + 1;
            let height = (self.y - other.y).unsigned_abs() + 1;

            width * height
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i64,
    y: i64,
}

fn parse_points(input: &str) -> Vec<Point> {
    input.lines().map(|line| {
                let mut parts = line.split(',');
                let x = parts
                    .next()
                    .expect("expect an x coordinate")
                    .parse::<i64>()
                    .expect("x should be parseable");
                let y = parts
                    .next()
                    .expect("expect an y coordinate")
                    .parse::<i64>()
                    .expect("y should be parseable");
                Point { x, y }
            }).collect()
}

fn point_in_polygon(p: (f64, f64), poly: &[Point]) -> bool {
    let (px, py) = p;
    let mut inside = false;

    for i in 0..poly.len() {
        let j = (i + 1) % poly.len();

        let x1 = poly[i].x as f64;
        let y1 = poly[i].y as f64;
        let x2 = poly[j].x as f64;
        let y2 = poly[j].y as f64;

        let crosses = ((y1 > py) != (y2 > py))
            && (px < (x2 - x1) * (py - y1) / (y2 - y1) + x1);

        if crosses {
            inside = !inside;
        }
    }

    inside
}

fn lower_bound(v: &[i64], x: i64) -> usize {
    let mut lo = 0;
    let mut hi = v.len();

    while lo < hi {
        let mid = (lo + hi) / 2;
        if v[mid] < x {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    lo
}

pub fn puzzle_two(input: &str) -> i64 {

    let points = parse_points(input);

    let mut xs: Vec<i64> = points.iter().map(|p| p.x).collect();
    let mut ys: Vec<i64> = points.iter().map(|p| p.y).collect();

    xs.sort_unstable();
    xs.dedup();

    ys.sort_unstable();
    ys.dedup();

    let w = xs.len() - 1;
    let h = ys.len() - 1;

    // outside[x][y] = 1 if the cell is outside the polygon
    let mut outside = vec![vec![0i32; h]; w];

    for xi in 0..w {
        for yi in 0..h {
            let cx = (xs[xi] + xs[xi + 1]) as f64 / 2.0;
            let cy = (ys[yi] + ys[yi + 1]) as f64 / 2.0;

            if !point_in_polygon((cx, cy), &points) {
                outside[xi][yi] = 1;
            }
        }
    }

    // 2D prefix sum of outside cells
    let mut pref = vec![vec![0i32; h + 1]; w + 1];

    for x in 0..w {
        for y in 0..h {
            pref[x + 1][y + 1] =
                outside[x][y]
                + pref[x][y + 1]
                + pref[x + 1][y]
                - pref[x][y];
        }
    }

    let rect_is_valid = |x1: i64, y1: i64, x2: i64, y2: i64| -> bool {
        let xa = lower_bound(&xs, x1);
        let xb = lower_bound(&xs, x2);

        let ya = lower_bound(&ys, y1);
        let yb = lower_bound(&ys, y2);

        // Thin rectangles lie entirely on a boundary line.
        if xa == xb || ya == yb {
            return true;
        }

        let bad =
            pref[xb][yb]
            - pref[xa][yb]
            - pref[xb][ya]
            + pref[xa][ya];

        bad == 0
    };

    let mut best = 0i64;

    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let a = points[i];
            let b = points[j];

            let area = ((a.x - b.x).abs() + 1) * ((a.y - b.y).abs() + 1);

            if area <= best {
                continue;
            }

            if rect_is_valid(a.x, a.y, b.x, b.y) {
                best = area;
            }
        }
    }

    best
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
        assert_eq!(50, puzz_one::puzzle_one(EXAMPLE_INPUT));
    }

    #[test]
    fn test_puzzle_two() {
        assert_eq!(24, puzzle_two(EXAMPLE_INPUT));
    }

}
