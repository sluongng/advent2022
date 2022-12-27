use std::{collections::HashMap, vec};

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

    fn from_origin(origin: &Point, distance: (isize, isize, isize)) -> Point {
        Point {
            x: origin.x + distance.0,
            y: origin.y + distance.1,
            z: origin.z + distance.2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Plane {
    points: Vec<Point>,
}

impl Plane {
    fn new(mut points: Vec<Point>) -> Plane {
        points.sort();
        Plane { points }
    }
}

#[derive(Debug, Clone)]
struct Cube {
    origin: Point,
    #[allow(dead_code)]
    points: Vec<Point>,
}

impl Cube {
    fn new(origin: Point) -> Cube {
        let mut points = vec![
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
            x: origin.x + coor.0,
            y: origin.y + coor.1,
            z: origin.z + coor.2,
        })
        .collect::<Vec<_>>();
        points.sort();

        Cube { origin, points }
    }

    fn planes(&self) -> Vec<Plane> {
        vec![
            // Planes that are attached to (0, 0, 0)
            vec![(0, 0, 0), (0, 1, 0), (0, 0, 1), (0, 1, 1)],
            vec![(0, 0, 0), (1, 0, 0), (0, 1, 0), (1, 1, 0)],
            vec![(0, 0, 0), (1, 0, 0), (0, 0, 1), (1, 0, 1)],
            // Planes that are attached to (1, 1, 1)
            vec![(0, 0, 1), (0, 1, 1), (1, 0, 1), (1, 1, 1)],
            vec![(0, 1, 0), (1, 1, 0), (0, 1, 1), (1, 1, 1)],
            vec![(1, 0, 0), (1, 1, 0), (1, 0, 1), (1, 1, 1)],
        ]
        .iter()
        .map(|v| {
            v.iter()
                .map(|&distance| Point::from_origin(&self.origin, distance))
                .collect()
        })
        .map(Plane::new)
        .collect()
    }
}

fn main() {
    let cubes: Vec<_> = include_str!("input1.txt")
        .trim()
        .lines()
        .map(|line| {
            line.split(',')
                .map(|i| i.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(Point::new)
        .map(Cube::new)
        .collect();

    let result = cubes
        .iter()
        .flat_map(|c| c.planes())
        .fold(HashMap::new(), |mut hm, plane| {
            *hm.entry(plane).or_insert(0) += 1;
            hm
        })
        .iter()
        .filter(|(_, &count)| count == 1)
        .count();

    println!("{result:?}");
}
