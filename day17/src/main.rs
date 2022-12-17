#![allow(dead_code)]
use std::iter::Cycle;
use std::vec::IntoIter;

const GAME_WIDTH: usize = 7;
const ROCK_COUNT: usize = 2022;

#[derive(Copy, Clone)]
enum RockKind {
    Line,
    Plus,
    RevEl,
    StraightLine,
    Square,
}

enum HDirection {
    Left,
    Right,
}

struct Tall {
    chamber: Vec<Vec<bool>>,

    falling_rock: Vec<(usize, usize)>,

    rocks: Cycle<IntoIter<RockKind>>,
}

impl Tall {
    fn new() -> Self {
        let rocks = vec![
            RockKind::Line,
            RockKind::Plus,
            RockKind::RevEl,
            RockKind::StraightLine,
            RockKind::Square,
        ];

        Self {
            chamber: Vec::new(),
            falling_rock: Vec::new(),

            rocks: rocks.into_iter().cycle(),
        }
    }

    fn add_rock(&mut self) {
        self.chamber.push(vec![false; GAME_WIDTH]);
        self.chamber.push(vec![false; GAME_WIDTH]);

        match self.rocks.next() {
            Some(RockKind::Line) => {
                vec![
                    //    0      1      2      3     4       5     6
                    vec![false, false, true, true, true, true, false], // | . . # # # # . |
                ]
                .into_iter()
                .for_each(|line| self.chamber.push(line));

                let current_height = self.chamber.len() - 1;
                self.falling_rock = vec![
                    (2, current_height),
                    (3, current_height),
                    (4, current_height),
                    (5, current_height),
                ];
            }
            Some(RockKind::Plus) => {
                vec![
                    //    0      1      2      3     4       5     6
                    vec![false, false, false, true, false, false, false], // | . . . # . . . |
                    vec![false, false, true, true, true, false, false],   // | . . # # # . . |
                    vec![false, false, false, true, false, false, false], // | . . . # . . . |
                ]
                .into_iter()
                .for_each(|line| self.chamber.push(line));

                let current_height = self.chamber.len() - 1;
                self.falling_rock = vec![
                    (3, current_height),
                    (2, current_height - 1),
                    (3, current_height - 1),
                    (4, current_height - 1),
                    (3, current_height - 2),
                ];
            }
            Some(RockKind::RevEl) => {
                vec![
                    //    0      1      2      3     4       5     6
                    vec![false, false, false, false, true, false, false], // | . . . . # . . |
                    vec![false, false, false, false, true, false, false], // | . . . . # . . |
                    vec![false, false, true, true, true, false, false],   // | . . # # # . . |
                ]
                .into_iter()
                // We are inserting into the tall one-by-one
                // so we need to push the bottom lines first,
                // which means reverse ordering of the drawing.
                .rev()
                .for_each(|line| self.chamber.push(line));

                let current_height = self.chamber.len() - 1;
                self.falling_rock = vec![
                    (4, current_height),
                    (4, current_height - 1),
                    (2, current_height - 2),
                    (3, current_height - 2),
                    (4, current_height - 2),
                ];
            }
            Some(RockKind::StraightLine) => {
                vec![
                    //    0      1      2      3     4       5     6
                    vec![false, false, true, false, false, false, false], // | . . # . . . . |
                    vec![false, false, true, false, false, false, false], // | . . # . . . . |
                    vec![false, false, true, false, false, false, false], // | . . # . . . . |
                    vec![false, false, true, false, false, false, false], // | . . # . . . . |
                ]
                .into_iter()
                .for_each(|line| self.chamber.push(line));

                let current_height = self.chamber.len() - 1;
                self.falling_rock = vec![
                    (2, current_height),
                    (2, current_height - 1),
                    (2, current_height - 2),
                    (2, current_height - 3),
                ];
            }
            Some(RockKind::Square) => {
                vec![
                    //    0      1      2      3     4       5     6
                    vec![false, false, true, true, false, false, false], // | . . # # . . . |
                    vec![false, false, true, true, false, false, false], // | . . # # . . . |
                ]
                .into_iter()
                .for_each(|line| self.chamber.push(line));

                let current_height = self.chamber.len() - 1;
                self.falling_rock = vec![
                    (2, current_height),
                    (3, current_height),
                    (2, current_height - 1),
                    (3, current_height - 1),
                ];
            }
            _ => {}
        }
    }

    fn move_to_side(&mut self, direction: HDirection) {
        let new_position: Vec<(i32, i32)> = self
            .falling_rock
            .clone()
            .iter()
            .map(|&(a, b)| {
                let increase = match direction {
                    HDirection::Left => 1,
                    HDirection::Right => -1,
                };
                (a as i32 + increase, b as i32)
            })
            .collect();

        let is_legal = new_position.iter().all(|&(a, b)| {
            a >= 0
                && b >= 0
                && a < GAME_WIDTH as i32
                && b < GAME_WIDTH as i32
                && *self
                    .chamber
                    .get(b as usize)
                    .unwrap()
                    .get(a as usize)
                    .unwrap()
        });

        if is_legal {
            self.falling_rock = new_position
                .iter()
                .map(|&(a, b)| (a as usize, b as usize))
                .collect();
        }
    }

    fn move_down(&mut self) -> bool {
        let new_position: Vec<(i32, i32)> = self
            .falling_rock
            .clone()
            .iter()
            .map(|&(a, b)| (a as i32, b as i32 + 1))
            .collect();
        let is_legal = new_position
            .iter()
            .map(|&(a, b)| {
                a >= 0
                    && b >= 0
                    && a < GAME_WIDTH as i32
                    && b < GAME_WIDTH as i32
                    && *self
                        .chamber
                        .get(b as usize)
                        .unwrap()
                        .get(a as usize)
                        .unwrap()
            })
            .all(|b| b);

        if is_legal {
            self.falling_rock = new_position
                .iter()
                .map(|&(a, b)| (a as usize, b as usize))
                .collect();

            true
        } else {
            false
        }
    }
}

fn main() {
    println!("Hello, world!");
}
