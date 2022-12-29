use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use pathfinding::prelude::bfs_reach;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn new(coords: (isize, isize, isize)) -> Point {
        Point {
            x: coords.0,
            y: coords.1,
            z: coords.2,
        }
    }

    fn from_origin(origin: &Point, distance: (isize, isize, isize)) -> Point {
        Point {
            x: origin.x + distance.0,
            y: origin.y + distance.1,
            z: origin.z + distance.2,
        }
    }

    fn dist_from(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }

    fn planes_from_edge(a: &Point, b: &Point) -> Vec<Plane> {
        let to_plane = |seed_vec: Vec<(isize, isize, isize)>| -> Vec<Plane> {
            seed_vec
                .iter()
                // creating new parallel edge by transforming current edge coordinates
                .map(|t| {
                    vec![
                        Point::new((a.x + t.0, a.y + t.1, a.z + t.2)),
                        Point::new((b.x + t.0, b.y + t.1, b.z + t.2)),
                    ]
                })
                // combine 2 parallel edges to make a plane
                .map(|mut v| {
                    v.extend(vec![a, b]);
                    Plane::new(v)
                })
                .collect_vec()
        };

        if a.x == b.x && a.y == b.y {
            return to_plane(vec![(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0)]);
        }
        if a.x == b.x && a.z == b.z {
            return to_plane(vec![(1, 0, 0), (-1, 0, 0), (0, 0, 1), (0, 0, -1)]);
        }
        if a.y == b.y && a.z == b.z {
            return to_plane(vec![(0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)]);
        }

        // should not happen
        panic!("should not happen")
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Cube {
    origin: Point,
}

impl Cube {
    fn new(origin: Point) -> Cube {
        Cube { origin }
    }

    #[allow(dead_code)]
    fn points(&self) -> Vec<Point> {
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
        .into_iter()
        .map(Point::new)
        .collect_vec();
        points.sort();

        points
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
        .collect_vec()
    }
}

fn main() {
    let cubes: Vec<_> = include_str!("input1.txt")
        .trim()
        .lines()
        .map(|line| {
            let mut it = line.split(',').map(|i| i.parse::<isize>().unwrap());

            (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
        })
        .map(Point::new)
        .map(Cube::new)
        .collect();

    let plane_counts =
        cubes
            .iter()
            .flat_map(|c| c.planes())
            .fold(HashMap::new(), |mut hm, plane| {
                *hm.entry(plane).or_insert(0) += 1;
                hm
            });

    let outer_planes = plane_counts
        .iter()
        .filter(|&(_, &count)| count == 1)
        .map(|(p, _)| p.clone())
        .collect::<HashSet<_>>();

    let min_outer_plane = cubes
        .iter()
        .min()
        .unwrap()
        .planes()
        .into_iter()
        .filter(|p| outer_planes.contains(p))
        .min()
        .unwrap();

    let reachable = bfs_reach(min_outer_plane.clone(), |p| {
        let original_plane = p.clone();

        p.clone()
            .points
            .into_iter()
            .combinations(2)
            .filter(|ps| ps[0].dist_from(&ps[1]) == 1)
            .map(|pair| Point::planes_from_edge(&pair[0], &pair[1]))
            .flat_map(|v| {
                let res = v
                    .into_iter()
                    .filter(|p| *p != original_plane)
                    .filter(|p| outer_planes.contains(p))
                    .collect_vec();

                // An edge could only have up to 4 attached planes.
                // Deducting the original plane, there could only be 3 planes attached.
                //
                // If all 3 planes and the original plan are outer_planes, the edge is
                // where 2 cubes touch. To avoid further traversal using this edge.
                if res.len() == 3 {
                    return vec![];
                }

                res
            })
            .collect_vec()
    })
    .collect_vec();

    println!("Reachable: {}", reachable.len());
    println!("\nFinal: {:?}", min_outer_plane);
}
