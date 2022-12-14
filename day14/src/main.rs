use std::collections::HashSet;
use std::fs;

fn main() {
    let input1 = fs::read_to_string("src/input1.txt").unwrap();
    let input2 = fs::read_to_string("src/input2.txt").unwrap();

    for input in vec![input1] {
        let cave = input
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

        let max_heights = cave
            .iter()
            .map(|line| line.iter().map(|coord| coord.1).max().unwrap())
            .max()
            .unwrap();

        let mut occupied_tiles = HashSet::new();
        cave.iter().for_each(|line| {
            line.windows(2).for_each(|pair| {
                let (a, b) = (pair[0], pair[1]);

                println!("Connecting {a:?} - {b:?}");
                if a.0 == b.0 {
                    let from = a.1.min(b.1);
                    let to = a.1.max(b.1);

                    for i in from..=to {
                        occupied_tiles.insert((a.0, i));
                    }
                } else if a.1 == b.1 {
                    let from = a.0.min(b.0);
                    let to = a.0.max(b.0);

                    for i in from..=to {
                        occupied_tiles.insert((i, a.1));
                    }
                } else {
                    panic!("2 points are not alligned {a:?} {b:?}");
                }
            })
        });

        draw_cave(occupied_tiles.clone());
    }
}

fn draw_cave(cave: HashSet<(i32, i32)>) {
    let mut lines = vec![];
    for i in 0..=9 {
        let mut line = vec![];
        for j in 494..=503 {
            if cave.contains(&(j, i)) {
                line.push("#");
            } else {
                line.push(" ");
            }
        }

        lines.push(line);
    }

    for (i, l) in lines.iter().enumerate() {
        println!("{i: >2} {:?}", l);
    }
}
