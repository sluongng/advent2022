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

    // Use for debugging
    fn render(&self) {
        self.chamber.iter().enumerate().rev().for_each(|(j, line)| {
            let line = line
                .iter()
                .enumerate()
                .map(|(i, b)| {
                    if self.falling_rock.contains(&(i, j)) {
                        return "@";
                    }

                    match b {
                        true => "#",
                        false => ".",
                    }
                })
                .fold("".into(), |acc: String, s| acc + " " + s);

            println!("{line}");
        });
        println!("");
    }

    fn tick(&mut self) -> bool {
        if self.falling_rock.len() == 0 {
            self.add_rock();
            self.rock_count += 1;
        }

        let h_move = self.future_moves.next().unwrap();
        self.move_to_side(h_move);
        self.move_down();

        if self.falling_rock.len() == 0 {
            // Trim empty lines at top because
            // we will be injecting them when we add new rocks.
            self.chamber = self
                .chamber
                .clone()
                .into_iter()
                .rev()
                .skip_while(|line| line.iter().all(|&cell| !cell))
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>();
        }
        if self.rock_count <= MAX_ROCK_COUNT {
            true
        } else {
            self.chamber = self
                .chamber
                .clone()
                .into_iter()
                .rev()
                .skip_while(|line| line.iter().all(|&cell| !cell))
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>();

            false
        }
    }

    fn add_rock(&mut self) {
        self.chamber.push(vec![false; GAME_WIDTH]);
        self.chamber.push(vec![false; GAME_WIDTH]);
        self.chamber.push(vec![false; GAME_WIDTH]);

        match self.future_rocks.next() {
            Some(RockKind::Line) => {
                self.chamber.push(vec![false; GAME_WIDTH]);

                let current_height = self.chamber.len() - 1;
                self.falling_rock = vec![
                    (2, current_height),
                    (3, current_height),
                    (4, current_height),
                    (5, current_height),
                ];
            }
            Some(RockKind::Plus) => {
                self.chamber.push(vec![false; GAME_WIDTH]);
                self.chamber.push(vec![false; GAME_WIDTH]);
                self.chamber.push(vec![false; GAME_WIDTH]);

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
                self.chamber.push(vec![false; GAME_WIDTH]);
                self.chamber.push(vec![false; GAME_WIDTH]);
                self.chamber.push(vec![false; GAME_WIDTH]);

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
                self.chamber.push(vec![false; GAME_WIDTH]);
                self.chamber.push(vec![false; GAME_WIDTH]);
                self.chamber.push(vec![false; GAME_WIDTH]);
                self.chamber.push(vec![false; GAME_WIDTH]);

                let current_height = self.chamber.len() - 1;
                self.falling_rock = vec![
                    (2, current_height),
                    (2, current_height - 1),
                    (2, current_height - 2),
                    (2, current_height - 3),
                ];
            }
            Some(RockKind::Square) => {
                self.chamber.push(vec![false; GAME_WIDTH]);
                self.chamber.push(vec![false; GAME_WIDTH]);

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
            HDirection::Left => (-1, 0),
            HDirection::Right => (1, 0),
        });

        if is_legal {
            self.falling_rock = new_position
                .iter()
                .map(|&(a, b)| (a as usize, b as usize))
                .collect();
        }
    }

    fn move_down(&mut self) {
        let (new_position, is_legal) = self.move_rock((0, -1));

        if is_legal {
            self.falling_rock = new_position
                .iter()
                .map(|&(a, b)| (a as usize, b as usize))
                .collect();
        } else {
            self.falling_rock
                .iter()
                .for_each(|&(a, b)| self.chamber[b][a] = true);

            self.falling_rock.clear();
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
        let is_legal = new_position.iter().all(|&(a, b)| {
            let legal =
                a >= 0 && b >= 0 && a < GAME_WIDTH as i32 && !self.chamber[b as usize][a as usize];

            legal
        });

        (new_position, is_legal)
    }
}

fn main() {
    let mut tall = Tall::new(include_str!("input2.txt").trim());

    while tall.tick() {
        // tick
    }

    println!("Height: {}", tall.chamber.len());
}
