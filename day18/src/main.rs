use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(coords: Vec<isize>) -> Point {
        Point {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Plane {
    points: Vec<Point>,
}

impl Plane {
    fn new(points: Vec<Point>) -> Plane {
        let mut ps = points.clone();
        ps.sort();

        Plane { points: ps }
    }
}

#[derive(Debug, Clone)]
struct Cube {
    points: Vec<Point>,
}

impl Cube {
    fn new(point: Point) -> Cube {
        let points = vec![
            (0, 0, 0),
            // 1 plus
            (1, 0, 0),
            (0, 1, 0),
            (0, 0, 1),
            // 2 plus
            (1, 1, 0),
            (0, 1, 1),
            (1, 0, 1),
            // opposite corners
            (1, 1, 1),
        ]
        .iter()
        .map(|coor| Point {
            x: point.x + coor.0,
            y: point.y + coor.1,
            z: point.z + coor.2,
        })
        .collect::<Vec<_>>();

        Cube { points }
    }

    fn planes(&self) -> Vec<Plane> {
        vec![
            // (0, 0, 0),
            // (0, 1, 0),
            // (0, 0, 1),
            // (0, 1, 1),
            Plane::new(vec![
                self.points[0],
                self.points[2],
                self.points[3],
                self.points[5],
            ]),
            // (0, 0, 0),
            // (1, 0, 0),
            // (0, 1, 0),
            // (1, 1, 0),
            Plane::new(vec![
                self.points[0],
                self.points[1],
                self.points[2],
                self.points[4],
            ]),
            // (0, 0, 0),
            // (1, 0, 0),
            // (0, 0, 1),
            // (1, 0, 1),
            Plane::new(vec![
                self.points[0],
                self.points[1],
                self.points[3],
                self.points[6],
            ]),
            // (0, 0, 1),
            // (0, 1, 1),
            // (1, 0, 1),
            // (1, 1, 1),
            Plane::new(vec![
                self.points[3],
                self.points[5],
                self.points[6],
                self.points[7],
            ]),
            // (0, 1, 0),
            // (1, 1, 0),
            // (0, 1, 1),
            // (1, 1, 1),
            Plane::new(vec![
                self.points[2],
                self.points[4],
                self.points[5],
                self.points[7],
            ]),
            // (1, 0, 0),
            // (1, 1, 0),
            // (1, 0, 1),
            // (1, 1, 1),
            Plane::new(vec![
                self.points[1],
                self.points[4],
                self.points[6],
                self.points[7],
            ]),
        ]
    }
}

fn main() {
    let cubes = include_str!("input2.txt")
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|i| i.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(Point::new)
        .map(Cube::new)
        .flat_map(|c| c.planes())
        .fold(HashMap::new(), |mut hm, plane| {
            *hm.entry(plane).or_insert(0) += 1;
            hm
        })
        .iter()
        .filter(|(_, &count)| count == 1)
        .count();

    println!("{:?}", cubes);
}
