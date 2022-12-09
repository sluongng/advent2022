use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Knot {
    x: i32,
    y: i32,
}

impl Knot {
    fn new(x: i32, y: i32) -> Self {
        Knot { x, y }
    }

    fn move_with_direction(&mut self, direction: &str) {
        match direction {
            "U" => {
                self.y += 1;
            }
            "D" => {
                self.y -= 1;
            }
            "L" => {
                self.x -= 1;
            }
            "R" => {
                self.x += 1;
            }
            _ => (),
        }
    }

    fn distance(&self, other: &Self) -> (i32, i32) {
        (self.x - other.x, self.y - other.y)
    }
}

// used during debug to visualize the movement of the knots
#[allow(dead_code)]
fn visualize_knots(knots: &Vec<Knot>) {
    let min_x = -20;
    let max_x = 20;
    let min_y = -20;
    let max_y = 20;

    println!("");
    for y in (min_y..=max_y).rev() {
        let mut line = format!("{: >3}", y);
        for x in min_x..=max_x {
            let mut add = ".".to_owned();

            for k in 0..knots.len() {
                if x == knots[k].x && y == knots[k].y {
                    if k == 0 {
                        add = "H".to_owned();
                        break;
                    }

                    add = k.to_string();
                }

                if x == 0 && y == 0 {
                    add = "s".to_owned();
                }
            }

            line += &add;
        }

        println!("{}", line)
    }
}

fn main() {
    let input = vec![
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        "U 2
D 2
R 2
U 2
D 1
L 2
R 2
D 1
R 2
D 1
R 2
L 1
U 1
L 2
R 2
L 2
U 2
R 2
L 1
U 1
R 2
U 1
D 2
L 1
U 2
D 1
L 1
R 1
L 1
D 1
R 1
D 2
L 1
U 2
L 1
U 1
D 2
U 1
R 2
U 1
R 2
D 1
U 1
D 2
L 2
U 2
L 2
R 1
L 1
U 2
D 2
U 1
R 2
L 2
U 1
R 1
L 2
D 1
L 1
U 2
D 1
R 1
U 2
R 2
U 2
R 2
D 1
L 2
R 2
U 2
D 2
U 2
L 1
U 1
L 2
U 1
D 1
L 1
D 1
L 1
R 1
L 1
R 2
D 1
L 1
D 2
R 2
L 1
U 2
L 1
U 2
L 1
U 1
D 2
U 1
L 1
R 1
D 1
L 2
U 1
L 1
R 2
D 1
L 2
U 1
L 2
D 2
R 2
D 2
U 1
D 2
R 3
U 2
L 2
R 3
L 2
U 3
L 3
D 1
R 2
L 1
R 2
U 3
R 3
L 1
D 2
R 3
L 2
D 3
L 3
U 1
R 1
D 2
R 2
D 3
L 3
U 3
R 3
D 2
U 1
D 3
L 2
R 1
D 3
L 3
D 3
L 2
D 3
L 1
D 2
R 2
L 3
U 2
R 2
L 1
U 1
D 3
R 3
U 2
R 3
D 1
R 1
U 3
D 2
L 2
D 2
L 3
U 2
R 3
D 1
L 3
U 1
R 3
U 1
D 1
U 2
D 1
R 2
L 2
D 1
R 1
D 2
L 1
U 1
D 1
R 2
D 1
L 2
R 1
D 3
U 1
D 3
U 2
L 1
U 3
D 2
L 3
U 2
D 3
R 3
D 3
U 3
R 1
U 2
L 3
D 1
R 3
D 2
U 2
D 2
L 3
U 3
D 3
R 3
L 2
D 3
U 3
D 3
U 2
R 2
L 3
R 3
U 1
D 1
U 2
L 4
D 1
U 1
L 2
D 4
U 3
L 1
U 1
D 1
L 4
R 4
L 4
R 2
U 4
L 2
R 1
L 3
U 3
D 2
L 3
U 4
R 3
L 4
D 2
U 3
D 2
U 2
L 4
D 2
L 1
R 4
U 4
D 2
U 4
L 4
D 2
L 3
U 3
D 1
R 1
U 1
D 4
U 3
L 4
D 1
L 4
U 1
L 2
U 3
D 2
L 3
U 4
D 2
U 1
D 1
U 1
R 2
L 2
D 3
U 2
R 1
U 1
L 4
U 1
R 2
D 3
R 1
D 1
R 3
D 3
L 1
U 3
L 3
D 4
L 3
D 4
L 1
D 1
R 1
D 1
U 4
L 2
U 3
D 4
R 4
D 1
U 2
D 4
U 1
R 4
L 2
D 3
U 4
R 3
D 1
L 3
D 4
R 4
L 3
D 2
L 3
U 3
R 3
D 1
R 1
L 2
D 2
U 4
R 1
L 1
D 2
U 3
D 3
L 3
D 5
L 1
U 2
L 5
U 3
R 3
L 2
U 2
L 1
U 5
L 4
D 1
L 3
D 5
U 3
R 5
D 1
L 5
R 5
L 5
D 4
U 4
L 5
U 5
R 5
U 2
D 5
L 5
U 3
L 3
U 4
L 5
U 3
D 4
R 3
U 1
L 2
U 5
L 1
U 2
D 1
U 1
D 1
R 4
L 1
R 2
D 2
R 3
L 5
R 2
D 5
U 4
R 5
D 3
L 1
D 3
R 3
L 1
R 4
D 3
L 5
D 1
R 1
U 4
D 2
R 3
U 4
D 3
U 1
D 1
R 1
D 4
U 1
L 5
R 1
U 5
L 4
R 1
L 1
R 4
D 3
U 3
D 3
U 4
L 2
U 1
R 2
U 1
D 4
U 4
R 1
U 4
R 3
L 4
R 2
L 4
D 5
R 3
U 1
D 1
L 3
R 2
U 1
D 3
U 1
D 1
R 3
U 2
L 6
R 1
D 1
R 5
L 5
U 4
D 1
L 6
D 5
L 3
D 6
R 1
U 3
D 3
R 5
U 1
R 5
D 1
L 1
D 4
L 1
D 4
R 1
D 5
U 6
L 5
R 2
U 2
D 6
R 3
U 6
L 2
D 5
U 3
R 1
D 3
L 1
D 3
U 1
D 3
U 4
R 2
D 3
R 2
D 2
L 3
U 2
R 6
D 4
R 4
D 1
U 3
D 1
L 3
U 6
R 4
D 3
R 3
L 4
R 2
U 5
D 6
L 5
U 3
L 4
D 2
R 2
D 3
L 6
D 1
U 1
R 4
L 5
D 3
U 4
L 2
U 5
L 2
U 5
D 1
L 4
U 1
R 2
L 3
D 1
U 4
D 2
U 2
L 1
D 4
R 2
L 3
R 2
D 6
R 2
D 3
R 3
D 2
U 6
D 4
R 4
U 4
L 5
R 3
L 4
D 3
L 2
R 2
D 1
L 6
R 7
L 3
U 3
D 6
U 4
R 4
D 7
L 4
U 1
L 5
R 5
L 5
U 7
R 1
D 2
R 6
D 6
U 2
R 1
U 2
L 5
D 7
U 5
L 3
U 4
L 5
U 4
L 2
D 1
U 3
R 4
L 5
R 5
L 4
R 3
L 7
R 4
U 1
L 4
R 4
U 7
D 3
U 4
L 7
D 5
L 1
D 5
L 7
U 5
L 5
D 3
U 5
D 3
U 7
R 5
D 1
U 2
L 1
U 1
D 1
L 1
U 6
D 2
L 7
U 5
R 4
L 3
U 2
L 1
R 1
L 2
D 3
L 3
U 4
D 4
U 7
D 7
U 6
D 7
R 5
U 7
D 4
U 3
R 3
D 3
U 6
L 6
U 1
R 1
U 7
R 2
U 1
D 1
R 1
D 1
L 6
U 6
D 7
R 4
D 1
L 2
U 6
L 2
D 2
R 2
D 3
L 3
R 5
D 1
R 4
L 1
U 1
R 3
L 8
U 3
L 3
R 5
L 8
D 1
R 1
D 8
L 4
D 2
L 4
U 3
D 6
R 2
U 4
D 6
R 8
D 4
L 7
R 6
U 2
R 1
U 3
R 2
L 1
R 8
L 2
D 6
R 6
D 6
U 2
D 6
R 7
D 4
U 6
R 5
D 8
L 3
R 6
D 4
R 4
U 2
D 8
L 8
U 6
D 2
R 3
L 2
D 6
U 8
R 3
D 3
L 5
D 5
L 4
R 8
D 3
U 3
D 2
R 8
U 4
L 8
U 2
R 1
L 4
D 5
L 5
R 5
D 3
L 7
D 4
L 6
D 2
U 3
D 2
L 2
D 6
R 2
U 6
R 7
L 8
U 3
D 8
L 1
U 8
L 8
D 2
R 6
U 4
D 8
U 3
L 7
D 7
L 8
U 8
L 4
U 6
L 1
U 4
D 7
U 3
R 2
D 4
L 5
U 8
R 4
U 1
R 2
U 4
D 7
U 7
R 7
D 7
U 9
L 4
D 8
U 6
L 3
D 1
U 8
L 9
U 8
D 8
U 2
L 1
U 5
L 9
U 4
D 7
L 5
U 4
R 5
D 2
L 3
D 5
R 3
L 2
D 9
U 5
L 6
U 7
D 2
R 2
D 3
R 5
U 7
L 1
U 9
R 3
D 8
R 6
D 5
U 9
R 9
D 3
U 8
D 3
R 6
D 4
U 8
L 4
D 8
R 2
U 2
R 3
D 9
L 9
U 7
R 4
D 4
L 5
U 1
D 3
R 6
L 4
R 1
D 6
R 4
L 4
U 6
D 2
L 2
D 3
R 8
D 3
U 4
L 4
D 2
U 2
L 4
D 9
R 9
D 6
U 7
L 8
U 6
D 5
R 2
D 5
U 9
R 5
D 7
U 2
L 3
R 2
U 5
L 4
D 5
U 2
L 9
U 3
R 2
D 9
R 5
D 9
U 2
R 2
L 6
U 3
L 6
R 8
U 5
D 8
R 9
L 2
R 2
U 1
D 10
R 8
L 6
D 9
R 5
L 10
R 2
L 3
R 9
D 7
U 10
D 9
L 7
U 8
D 7
U 2
D 1
U 5
R 9
U 9
R 6
D 4
L 3
D 6
R 10
D 2
L 4
U 8
R 6
L 3
U 1
D 8
R 6
D 4
L 1
R 3
L 10
U 4
D 5
U 1
R 3
U 5
R 5
D 9
R 1
L 4
U 4
R 4
D 7
L 1
U 6
R 6
U 1
D 4
R 2
L 3
D 6
L 3
R 1
L 10
D 1
L 1
U 6
R 7
L 8
U 2
D 10
R 4
D 9
R 2
U 9
L 2
U 7
D 4
U 4
R 9
U 9
R 6
D 7
L 8
U 1
D 6
L 6
U 8
D 3
R 4
L 4
D 1
L 2
D 3
L 2
U 1
L 5
D 2
U 8
L 1
U 5
L 3
U 3
R 3
U 1
L 7
D 9
R 3
D 10
R 8
L 5
D 3
U 6
R 4
U 4
D 2
L 5
U 11
L 3
U 1
L 6
D 1
L 2
U 8
R 6
L 4
R 2
U 3
L 3
D 4
L 2
R 5
U 4
D 5
U 10
R 1
D 6
L 4
U 9
D 3
U 5
D 9
L 1
U 8
D 11
L 4
D 4
L 5
D 7
L 7
R 10
U 5
R 1
D 10
R 3
D 11
R 7
L 6
U 7
D 4
L 7
U 8
R 6
L 4
U 5
L 7
R 2
D 10
L 8
R 4
U 1
R 5
L 2
R 4
L 6
R 4
U 10
R 6
U 9
R 9
D 6
L 7
D 1
U 1
L 8
U 1
D 8
L 10
R 6
D 4
R 9
D 4
R 3
D 4
U 8
L 3
D 7
L 2
R 10
U 5
R 4
L 3
D 6
U 7
R 8
D 5
R 6
L 7
R 4
U 6
L 8
D 8
R 6
L 6
U 8
R 10
U 4
R 2
D 2
L 11
R 10
L 5
U 10
L 5
D 12
L 7
D 8
L 2
U 1
L 11
R 3
D 8
U 7
L 6
D 9
L 12
U 7
L 11
R 7
U 8
L 4
D 9
U 6
D 9
R 7
L 4
R 12
U 8
D 6
U 8
D 6
U 5
L 2
U 2
L 9
D 10
L 11
U 10
R 4
L 12
U 6
L 11
R 10
U 9
L 2
D 11
L 11
D 10
U 8
R 10
L 7
R 6
L 6
U 5
L 8
D 5
U 7
L 5
D 8
R 5
L 7
D 10
L 10
D 12
L 8
R 11
U 6
L 12
D 9
U 5
R 2
U 6
D 11
U 5
R 1
L 2
R 3
U 8
R 4
L 10
R 6
D 9
L 3
U 11
L 4
U 1
R 5
L 8
D 8
U 2
D 7
U 5
R 10
L 4
D 4
U 2
L 7
D 12
U 12
D 5
U 12
R 3
L 4
U 10
L 1
R 2
L 9
R 9
L 5
U 5
L 10
U 12
L 3
D 5
U 12
L 10
D 11
U 12
L 2
U 11
R 6
D 10
U 6
L 5
D 10
U 10
D 12
U 3
L 10
U 9
R 4
U 4
L 12
R 12
L 4
D 5
L 9
U 5
L 11
R 12
D 8
L 6
U 10
D 2
U 11
D 6
R 10
D 1
L 5
U 1
D 10
U 1
L 13
U 11
L 6
R 1
U 11
D 10
R 11
L 3
U 6
R 13
U 13
L 6
R 4
D 3
L 10
U 2
R 9
D 6
L 12
U 11
D 9
R 2
L 2
D 4
R 13
L 7
R 7
L 9
R 11
U 4
D 2
L 4
R 12
L 3
D 8
U 9
D 3
R 11
U 11
D 13
U 1
D 6
U 2
R 13
L 5
U 6
L 5
U 13
L 12
D 7
L 12
R 4
U 4
D 3
L 5
D 10
U 10
L 2
U 11
L 9
U 11
R 7
D 4
U 9
D 7
R 6
L 2
D 2
L 1
U 10
R 5
U 5
R 1
L 10
D 11
U 5
D 13
R 9
D 6
R 2
D 13
R 12
L 11
D 6
R 9
U 12
R 11
U 6
R 12
D 4
R 11
L 5
D 1
U 5
R 8
D 1
R 12
L 9
U 6
L 9
U 3
L 6
R 3
U 2
R 3
U 14
L 7
R 7
L 12
U 7
R 5
D 7
L 10
U 14
D 8
L 5
U 11
D 4
U 10
R 9
U 4
R 1
L 11
U 13
D 8
R 10
L 3
U 11
L 5
U 2
R 6
U 3
R 10
L 4
R 3
D 8
U 11
L 11
U 6
R 6
U 8
R 4
L 10
R 2
U 4
L 2
D 12
R 12
D 4
U 14
R 6
U 1
R 9
L 5
U 9
D 5
R 13
U 11
L 8
R 11
D 9
L 8
D 8
U 5
R 2
L 3
D 6
U 9
D 1
L 8
U 6
L 6
R 8
D 13
R 12
L 5
U 8
R 11
D 4
U 13
D 10
U 4
D 3
L 14
R 14
D 5
R 2
D 7
L 13
U 1
L 14
U 15
D 11
R 2
U 15
R 11
L 8
U 11
D 4
R 2
U 8
L 8
R 11
U 9
L 11
D 4
L 8
U 9
L 9
D 15
U 1
L 8
U 7
L 12
U 13
R 7
L 6
D 13
U 1
L 9
D 11
R 8
D 3
R 11
L 8
R 9
U 2
L 1
U 5
D 14
R 3
U 12
R 4
U 15
D 1
R 11
L 11
U 11
L 1
D 12
U 7
L 13
R 4
U 5
D 15
L 14
R 10
U 9
R 7
D 13
L 15
R 10
L 2
R 5
D 12
U 13
R 6
L 15
R 14
D 1
U 1
L 2
U 11
R 15
U 8
R 9
U 5
L 9
D 14
R 9
D 9
U 8
R 8
U 3
R 7
D 11
R 4
U 9
R 9
U 11
D 4
U 12
L 9
R 3
D 7
L 6
R 1
D 9
R 1
D 4
R 16
U 12
R 8
D 7
L 7
R 9
U 16
R 15
U 14
L 14
D 3
R 4
U 4
D 1
L 2
D 11
R 2
U 2
D 16
U 4
D 5
R 14
D 3
R 5
D 10
R 2
L 4
U 6
R 9
D 16
R 12
U 2
L 16
R 7
L 8
R 5
L 5
U 14
D 16
U 7
R 2
U 15
L 3
R 7
L 5
U 8
R 11
D 3
L 5
U 7
L 8
R 3
U 3
D 5
U 13
L 8
U 1
D 13
R 11
L 2
U 14
R 15
U 2
D 7
R 15
L 5
D 9
U 8
R 5
L 15
D 3
R 10
D 10
U 7
D 7
U 14
R 2
U 8
R 14
L 11
R 16
U 11
D 16
U 2
L 10
U 5
D 6
L 7
U 8
D 10
R 11
L 7
D 5
U 6
L 5
D 5
R 1
L 7
U 9
L 5
U 3
D 7
R 14
U 9
D 6
L 13
U 8
L 5
D 5
R 3
U 7
D 6
U 8
R 16
U 17
L 5
R 8
L 8
U 8
L 10
D 17
R 5
D 3
L 17
R 17
D 4
R 15
D 6
L 3
R 17
D 9
L 13
U 4
D 2
L 17
R 1
L 14
D 17
L 9
R 12
U 14
D 12
U 3
R 7
D 1
L 13
R 13
U 16
D 13
U 15
R 15
U 4
D 11
L 7
U 1
D 14
U 8
L 11
D 13
L 12
U 17
L 13
U 2
R 10
L 12
R 12
U 1
R 8
U 10
D 15
R 3
U 4
R 12
L 9
D 15
L 13
D 15
L 7
D 8
L 7
U 6
L 17
R 4
D 10
U 11
L 8
R 14
U 9
R 9
L 6
U 1
L 5
D 11
R 10
L 16
U 15
L 12
R 13
D 5
L 4
D 12
L 6
R 3
D 12
L 4
U 2
L 16
D 6
R 15
U 7
D 2
L 14
D 6
R 14
D 7
U 11
R 4
D 3
R 6
L 3
D 11
U 2
R 5
U 3
L 1
U 7
R 13
U 12
D 12
R 13
D 7
L 12
D 10
U 14
R 12
D 4
R 2
U 16
D 9
L 14
U 14
D 8
L 7
R 7
D 18
U 18
R 8
L 15
U 12
D 11
L 16
D 18
R 14
L 5
R 12
D 11
R 13
L 17
D 4
L 10
U 1
L 8
U 10
R 13
D 2
L 14
D 1
R 5
L 4
R 11
L 7
D 2
R 8
L 13
R 5
L 5
R 11
U 15
D 3
U 2
R 6
U 1
R 8
L 5
U 1
R 9
D 13
R 8
U 17
L 16
U 3
L 9
D 9
U 6
L 8
R 13
L 7
R 16
U 6
L 13
U 18
L 17
D 15
R 5
U 6
D 11
U 15
L 4
R 14
U 9
R 11
U 9
L 11
D 7
U 9
L 10
D 4
R 7
U 7
D 8
R 1
D 12
R 16
U 7
R 9
D 10
R 1
L 1
U 16
L 4
U 11
D 1
U 3
R 17
U 17
L 12
R 9
U 14
D 4
L 6
R 4
L 15
U 1
D 2
U 9
R 16
L 1
R 2
L 12
U 12
D 7
R 13
D 6
L 2
D 3
U 8
L 7
U 16
L 15
D 18
L 19
U 7
R 17
D 7
U 3
R 12
D 1
L 17
U 19
D 15
R 3
D 5
U 17
D 16
U 2
R 13
D 7
U 19
L 5
R 3
D 15
L 12
D 19
L 19
U 1
R 10
D 14
U 3
R 1
L 6
U 14
L 11
R 10
L 7
R 14
D 9
U 14
R 5
L 19
U 10
L 12
U 10
L 3
D 13
U 12
R 12
D 6
L 2
U 1
D 16
U 9
L 7
R 14
L 15
U 9
L 19
U 14
D 9
U 15
L 13
U 12
L 1
D 7
U 7
L 7
R 2
U 16
R 6
D 11
U 7
D 11
U 19
R 13
D 7
R 7
L 6
R 14
L 18
D 2
L 5
D 8
L 7
U 17
D 15
U 19",
    ]
    .iter()
    .nth(2)
    .unwrap()
    .split("\n")
    .map(|line| {
        let mut tokens = line.split(" ");
        (
            tokens.next().unwrap(),
            tokens.next().unwrap().parse::<i32>().unwrap(),
        )
    })
    .collect::<Vec<_>>();

    let mut unique_tail_pos = HashSet::new();
    let mut knots = vec![Knot::new(0, 0).clone(); 10];

    for (direction, count) in input {
        for _ in 0..count {
            for i in 0..knots.len() {
                // head moves normally
                if i == 0 {
                    knots[i].move_with_direction(direction);
                    continue;
                }

                // tail moves based on distance vs head,
                // which is the previous element in knots vector
                let dis = knots[i - 1].distance(&(knots[i]));
                match dis {
                    // In part 2, it's no longer guaranteed that the same direction
                    // could be applied when head and tail are alligned.
                    //
                    // From
                    // |   |   | 2 |   |
                    // |   | 1 |   |   |
                    // |   |   | H |   |
                    // |   |   |   |   |
                    //
                    // To
                    // |   |   |   |   |
                    // |   |   | 2 |   |
                    // |   |   | 1 | H |
                    // |   |   |   |   |
                    //
                    // In the example above, as 'H' moved right, '1' needed to move down + right.
                    // This make distance between '1' and '2' to be ( 0,-2).
                    //
                    // So here, 2 needs to move down instead of H's original direction 'right'.
                    (2, 0) => {
                        knots[i].move_with_direction("R");
                    }
                    (-2, 0) => {
                        knots[i].move_with_direction("L");
                    }
                    (0, 2) => {
                        knots[i].move_with_direction("U");
                    }
                    (0, -2) => {
                        knots[i].move_with_direction("D");
                    }

                    // Head could be in a Knight-Chess position away from tail after the move
                    // shown below in coordinates.
                    //
                    // | (-2, 2) | (-1, 2) |         | ( 1, 2) | ( 2, 2) |
                    // | (-2, 1) |    X    |         |    X    | ( 2, 1) |
                    // |         |         |    O    |         |         |
                    // | (-2,-1) |    X    |         |    X    | ( 2,-1) |
                    // | (-2,-2) | (-1,-2) |         | ( 1,-2) | ( 2,-2) |
                    //
                    // in these positions, Tail should move diagonally from `O` to `X`
                    // so that it could catch up with Head. Effectively, this means that for
                    // every two possible Head positions, there is one X position that Tail
                    // should be moving to.
                    //
                    // In part 2, the corner positions ( 2, 2) are also included as multiple
                    // heads could all move diagonally when the relative starting position is
                    // already diagonally.
                    //
                    //       Before                   After
                    // |   |   |   |   |        |   |   |   |   |
                    // |   |   |   |   |        |   |   | h |   |
                    // |   |   | h |   |        |   |   | a |   |
                    // |   |   | a |   |        |   |   | b |   |
                    // |   | b |   |   | =====> |   | t |   |   |
                    // | t |   |   |   |        |   |   |   |   |
                    // |   |   |   |   |        |   |   |   |   |
                    //
                    // In the example above:
                    // - 'h' moves up,                  distance from a becomes ( 0, 2)
                    // - 'a' moves up,                  distance from b becomes ( 1, 2)
                    // - 'b' moves diagonally up right, distance from t becomes ( 2, 2)
                    // - 't' moves diagonally up right
                    //
                    // Here we pair up these Head positions with the diagonal direction that Tail
                    // should move toward.
                    (2, 1) | (1, 2) | (2, 2) => {
                        knots[i].move_with_direction("R");
                        knots[i].move_with_direction("U");
                    }
                    (-2, 1) | (-1, 2) | (-2, 2) => {
                        knots[i].move_with_direction("L");
                        knots[i].move_with_direction("U");
                    }
                    (2, -1) | (1, -2) | (2, -2) => {
                        knots[i].move_with_direction("R");
                        knots[i].move_with_direction("D");
                    }
                    (-2, -1) | (-1, -2) | (-2, -2) => {
                        knots[i].move_with_direction("L");
                        knots[i].move_with_direction("D");
                    }

                    // Do nothing in any other cases
                    _ => (),
                }
            } // moved all knots

            let tail_pos = format!(
                "{} - {}",
                knots[knots.len() - 1].x,
                knots[knots.len() - 1].y
            );
            unique_tail_pos.insert(tail_pos);
        } // finish all moves in 1 line
    } // finish all inputs

    println!("Count: {:?}", unique_tail_pos.len());
}
