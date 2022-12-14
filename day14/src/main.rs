use std::fs;

fn main() {
    let input1 = fs::read_to_string("src/input1.txt").unwrap();
    let input2 = fs::read_to_string("src/input2.txt").unwrap();

    let cave = input2
        .as_str()
        .trim()
        .split('\n')
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    let mut vec = coord
                        .split(',')
                        .map(|number| number.parse::<i32>().unwrap());

                    (vec.next().unwrap(), vec.next().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let widths = cave
        .iter()
        .map(|line| {
            let horizontal_points = line.iter().map(|coord| coord.0);

            (
                horizontal_points.clone().min().unwrap(),
                horizontal_points.clone().max().unwrap(),
            )
        })
        .fold((i32::MAX, 0), |(min, max), (current_min, current_max)| {
            (min.min(current_min), max.max(current_max))
        });
    let heights = cave
        .iter()
        .map(|line| {
            let horizontal_points = line.iter().map(|coord| coord.1);

            (
                horizontal_points.clone().min().unwrap(),
                horizontal_points.clone().max().unwrap(),
            )
        })
        .fold((i32::MAX, 0), |(min, max), (current_min, current_max)| {
            (min.min(current_min), max.max(current_max))
        });

    println!("The cave widths {widths:?}");
    println!("The cave heights {heights:?}");
}
