use std::collections::HashSet;

fn main() {
    let mut sensors = HashSet::new();
    let mut beacons = HashSet::new();

    let mut no_beacon = HashSet::new();

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
        sensors.insert((sensor_x, sensor_y));

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
        beacons.insert((beacon_x, beacon_y));

        let distance = (beacon_x - sensor_x).abs() + (beacon_y - sensor_y).abs();

        let left_most = sensor_x - distance;
        let right_most = sensor_x + distance;
        // println!("Distance={distance} Horizontally running from {left_most} to {right_most}");

        for x in left_most..=right_most {
            let half = distance - (x - sensor_x).abs();
            let up_most = sensor_y - half;
            let down_most = sensor_y + half;
            // println!("i={i} x={x} Vertically running from {up_most} to {down_most}");

            for y in up_most..=down_most {
                let pos = (x, y);

                if sensors.contains(&pos) || beacons.contains(&pos) {
                    continue;
                }
                no_beacon.insert(pos);
            }
        }
    });

    let count = no_beacon
        .iter()
        .filter(|t| t.1 == 2_000_000)
        .collect::<Vec<_>>()
        .len();
    println!("count is {count}");
}
