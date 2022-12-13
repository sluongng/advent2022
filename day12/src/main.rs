use std::fs;

use pathfinding::prelude::dijkstra;

fn main() {
    let mut end_pos = (0, 0);

    // parse input into a map where each tile is a numeric value of the tile's level
    // with a side effect of collecting possible starting points and End position.
    let input = fs::read_to_string("src/input2.txt").unwrap();
    let map: Vec<Vec<usize>> = input
        .as_str()
        .trim()
        .split('\n')
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    'S' => 'a',
                    'E' => {
                        end_pos = (i, j);
                        'z'
                    }
                    _ => c,
                })
                // calculate level of each tile on the map
                .map(|c| c as usize - 'a' as usize)
                .collect()
        })
        .collect();

    let min_cost = dijkstra(
        &end_pos,
        |&(line, row)| {
            let mut neighbors = vec![];

            // conditionally include tiles above and below current tile
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

            // conditionally include tiles to the left and right of current tile
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
                .filter(|&(l, r)| map[line][row] as i32 <= map[l][r] as i32 + 1)
                .map(|p| (p, 1))
                .collect::<Vec<_>>()
        },
        // 0 is level of an 'a' tile on the map
        |&(line, row)| map[line][row] == 0,
    )
    .unwrap()
    .1;

    println!("Shortest path: {min_cost}");
}
