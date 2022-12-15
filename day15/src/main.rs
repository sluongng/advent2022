use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash)]
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
    let mut my_sensors = Vec::new();

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

        my_sensors.push(Sensor {
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

    let min_x = my_sensors
        .iter()
        .map(|s| s.position.x - s.distance)
        .min()
        .unwrap();
    let max_x = my_sensors
        .iter()
        .map(|s| s.position.x + s.distance)
        .max()
        .unwrap();

    let mut not_beacon = HashSet::new();
    for x in min_x..=max_x {
        for s in my_sensors.iter() {
            let p = Position { x, y: 2_000_000 };
            match s.is_beacon(&p) {
                Some(b) => {
                    if b {
                        not_beacon.insert(p);
                    }
                }
                None => {}
            };
        }
    }

    println!("count is {}", not_beacon.len());
}
