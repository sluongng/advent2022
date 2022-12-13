use std::collections::VecDeque;
use std::fs;

use pathfinding::prelude::dijkstra;

fn main() {
    let mut possible_start_points = vec![];
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
                    'S' | 'a' => {
                        possible_start_points.push((i, j));
                        'a'
                    }
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

    // given a starting position, calculate the minimum distance to get to End position
    let get_cost = |start_pos: (usize, usize)| {
        // TODO: Recode this to hand roll the dijkstra while starting from End position.
        // Expect to get a map of Option<usize> distance in the end.
        // And filter for tiles with original level being 'z - a' and get min tiles.
        dijkstra(
            &start_pos,
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
                    // exclude tiles with level too high (more than 1) compare to current tiles
                    .filter(|&(l, r)| map[l][r] as i32 - map[line][row] as i32 <= 1)
                    // include weight to traverse between tiles for Dijkstra
                    // here since all tiles weighted equally, 1 should suffice
                    .map(|p| (p, 1))
                    .collect::<Vec<_>>()
            },
            |&p| p == end_pos,
        )
    };

    //
    let min_cost = possible_start_points
        .iter()
        // calculate for distance to End position
        .filter_map(|(l, r)| get_cost((*l, *r)))
        // discard the actual path
        .map(|(_, i)| i)
        // take the lowest result
        .min()
        .unwrap();

    println!("Shortest path: {}", min_cost);
}
