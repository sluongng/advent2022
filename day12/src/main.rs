use std::fs;

use pathfinding::prelude::dijkstra;

fn main() {
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);

    let input = fs::read_to_string("src/input2.txt").unwrap();
    let map = input
        .as_str()
        .trim()
        .split("\n")
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    'S' => {
                        start_pos = (i, j);
                        'a'
                    }
                    'E' => {
                        end_pos = (i, j);
                        'z'
                    }
                    _ => c,
                })
                .map(|c| c as usize - 'a' as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let result = dijkstra(
        &start_pos,
        |&(line, row)| {
            let mut neighbors = vec![];

            if line == 0 {
                // down
                neighbors.push((line + 1, row));
            } else if line == map.len() - 1 {
                // up
                neighbors.push((line - 1, row));
            } else {
                // up
                neighbors.push((line - 1, row));
                // down
                neighbors.push((line + 1, row));
            }

            if row == 0 {
                // right
                neighbors.push((line, row + 1));
            } else if row == map[0].len() - 1 {
                // left
                neighbors.push((line, row - 1));
            } else {
                // right
                neighbors.push((line, row + 1));
                // left
                neighbors.push((line, row - 1));
            }

            neighbors
                .into_iter()
                .filter(|&(l, r)| map[l][r] as i32 - map[line][row] as i32 <= 1)
                .map(|p| (p, 1))
                .collect::<Vec<_>>()
        },
        |p| p.0 == end_pos.0 && p.1 == end_pos.1,
    )
    .unwrap();

    println!("{:?}", result);
}
