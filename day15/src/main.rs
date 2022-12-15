use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone)]
struct Position {
    x: i32,
    y: i32,
}

struct Sensor {
    position: Position,
    distance: i32,
    nearest_beacon: Position,
}

impl Sensor {
    fn is_beacon(&self, p: &Position) -> Option<bool> {
        let dis = (p.x - self.position.x).abs() + (p.y - self.position.y).abs();

        if dis > self.distance {
            // not sure
            return None;
        }

        if (p.x == self.nearest_beacon.x && p.y == self.nearest_beacon.y)
            || (p.x == self.position.x && p.y == self.position.y)
        {
            return Some(false);
        }

        Some(true)
    }
}

fn main() {
    let mut sensors = Vec::new();

    include_str!("input2.txt").lines().for_each(|line| {
        let mut halves = line.split(": ");

        let mut sensor_coordinate = halves
            .next()
            .unwrap()
            .trim_start_matches("Sensor at ")
            .split(", ")
            .map(|coord| coord.split('=').last().unwrap().parse::<i32>());
        let (sensor_x, sensor_y) = (
            sensor_coordinate.next().unwrap().unwrap(),
            sensor_coordinate.next().unwrap().unwrap(),
        );

        let mut beacon_coordinate = halves
            .next()
            .unwrap()
            .trim_start_matches("closest beacon is at ")
            .split(", ")
            .map(|coord| coord.split('=').last().unwrap().parse::<i32>());
        let (beacon_x, beacon_y) = (
            beacon_coordinate.next().unwrap().unwrap(),
            beacon_coordinate.next().unwrap().unwrap(),
        );

        let distance = (beacon_x - sensor_x).abs() + (beacon_y - sensor_y).abs();

        sensors.push(Sensor {
            position: Position {
                x: sensor_x,
                y: sensor_y,
            },
            distance,
            nearest_beacon: Position {
                x: beacon_x,
                y: beacon_y,
            },
        });
    });

    let mut border_repeat = HashMap::new();
    for s in sensors.iter() {
        for x in (s.position.x - s.distance - 1)..=(s.position.x + s.distance + 1) {
            let half = s.distance + 1 - (x - s.position.x).abs();

            let upper = Position {
                x,
                y: s.position.y - half,
            };
            let mut points = vec![upper];

            // TODO: double check these de-duplication
            if half != 0 {
                let lower = Position {
                    x,
                    y: s.position.y + half,
                };
                points.push(lower);
            }

            for p in points {
                if p.x < 0 || p.y < 0 || p.x > 4_000_000 || p.y > 4_000_000 {
                    continue;
                }

                border_repeat
                    .entry(p)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            }
        }
    }

    let mut borders = border_repeat
        .iter()
        .filter(|(_, &count)| count > 3)
        .filter(|(p, _)| {
            let mut result = false;

            for s in sensors.iter() {
                if s.is_beacon(p).is_none() {
                    result = true;
                    break;
                }
            }

            result
        })
        // .inspect(|(p, count)| println!("({} - {}) has count {}", p.x, p.y, count))
        .collect::<Vec<_>>();

    borders.sort_by(|a, b| a.1.cmp(b.1));

    for (p, count) in borders.iter() {
        println!("({} - {}) has count {}", p.x, p.y, count);
    }

    // let mut all_sensors = HashSet::new();
    // let mut all_beacons = HashSet::new();

    // for s in sensors.iter() {
    //     all_sensors.insert(s.position.clone());
    //     all_beacons.insert(s.nearest_beacon.clone());
    // }

    // for y in -2..=22 {
    //     let mut line: String = "".into();
    //     for x in -2..=25 {
    //         let p = Position { x, y };

    //         if all_sensors.contains(&p) {
    //             line += "S";
    //             continue;
    //         }
    //         if all_beacons.contains(&p) {
    //             line += "B";
    //             continue;
    //         }
    //         if not_beacon.contains(&p) {
    //             line += "#";
    //             continue;
    //         }

    //         line += ".";
    //     }
    //     println!("{}", line);
    // }

    // for nb in not_beacon {
    //     println!("count is {} - {}", nb.x, nb.y);
    // }
}
