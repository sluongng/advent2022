#![allow(dead_code)]
use std::iter::Cycle;
use std::vec::IntoIter;

const GAME_WIDTH: usize = 7;
const MAX_ROCK_COUNT: usize = 2022;

#[derive(Copy, Clone)]
enum RockKind {
    Line,
    Plus,
    RevEl,
    StraightLine,
    Square,
}

#[derive(Copy, Clone)]
enum HDirection {
    Left,
    Right,
}

struct Tall {
    chamber: Vec<Vec<bool>>,
    rock_count: usize,
    falling_rock: Vec<(usize, usize)>,

    future_rocks: Cycle<IntoIter<RockKind>>,
    future_moves: Cycle<IntoIter<HDirection>>,
}

impl Tall {
    fn new(input: &str) -> Self {
        let rocks = vec![
            RockKind::Line,
            RockKind::Plus,
            RockKind::RevEl,
            RockKind::StraightLine,
            RockKind::Square,
        ];

        Self {
            chamber: Vec::new(),
            rock_count: 0,
            falling_rock: Vec::new(),

            future_rocks: rocks.into_iter().cycle(),
            future_moves: input
                .chars()
                .map(|c| match c {
                    '<' => HDirection::Left,
                    '>' => HDirection::Right,
                    n => {
                        panic!("unexpected input: {n}");
                    }
                })
                .collect::<Vec<_>>()
                .into_iter()
                .cycle(),
        }
    }

    fn tick(&mut self) -> bool {
        if self.rock_count == 0 {
            self.add_rock();
        }

        let h_move = self.future_moves.next().unwrap();
        self.move_to_side(h_move);
        if !self.move_down() {
            self.add_rock();
            self.rock_count += 1;
        }

        if self.rock_count < MAX_ROCK_COUNT {
            // Trim empty lines at top because
            // we will be injecting them when we add new rocks.
            self.chamber = self
                .chamber
                .clone()
                .into_iter()
                .skip_while(|line| line.iter().all(|&cell| cell))
                .collect::<Vec<_>>();

            true
        } else {
            false
        }
    }

    fn add_rock(&mut self) {
        self.chamber.push(vec![false; GAME_WIDTH]);
        self.chamber.push(vec![false; GAME_WIDTH]);

        match self.future_rocks.next() {
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
        let (new_position, is_legal) = self.move_rock(match direction {
            HDirection::Left => (1, 0),
            HDirection::Right => (-1, 0),
        });

        if is_legal {
            self.falling_rock = new_position
                .iter()
                .map(|&(a, b)| (a as usize, b as usize))
                .collect();
        }
    }

    fn move_down(&mut self) -> bool {
        let (new_position, is_legal) = self.move_rock((0, -1));

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

    fn move_rock(&mut self, translation: (i32, i32)) -> (Vec<(i32, i32)>, bool) {
        let new_position: Vec<(i32, i32)> = self
            .falling_rock
            .clone()
            .iter()
            .map(|&(a, b)| (a as i32 + translation.0, b as i32 + translation.1))
            .collect();

        // collision check
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

        (new_position, is_legal)
    }
}

fn main() {
    let mut tall = Tall::new(include_str!("input1.txt").trim());

    while tall.tick() {
        // tick
    }

    println!("Height: {}", tall.chamber.len());
}
