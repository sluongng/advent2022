use std::fs;

struct ClockCircuit {
    cycle: i32,

    // used for addx
    x: i32,
    wait: u32,
    add_val: i32,

    // track important cycles
    tracking_cycle_next: i32,
    tracking_cycle_step: i32,

    current_line: String,
}

impl ClockCircuit {
    fn new() -> ClockCircuit {
        ClockCircuit {
            cycle: 1,
            x: 1,
            wait: 0,
            add_val: 0,
            tracking_cycle_next: 40,
            tracking_cycle_step: 40,
            current_line: "#".into(), // smol hack pls don't @ me
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;

        if self.wait > 0 {
            self.wait -= 1;
        }

        let pos = self.cycle % self.tracking_cycle_step;
        self.current_line += match pos >= self.x && pos <= self.x + 2 {
            true => "#",
            false => ".",
        };

        if self.cycle == self.tracking_cycle_next {
            if !self.current_line.is_empty() {
                println!("{}", self.current_line);
            }

            self.current_line = "".into();
            self.tracking_cycle_next += self.tracking_cycle_step;
        }

        if self.wait == 0 && self.add_val != 0 {
            self.x += self.add_val;
            self.add_val = 0;
        }
    }

    fn noop(&mut self) {
        self.tick();
    }

    fn addx(&mut self, val: i32) {
        self.wait = 1;
        self.add_val = val;

        self.tick();
        self.tick();
    }
}

fn main() {
    let input1 = fs::read_to_string("./src/input1.txt").unwrap();
    let input2 = fs::read_to_string("./src/input2.txt").unwrap();

    let input = [
        "noop
addx 3
addx -5",
        input1.as_str(),
        input2.as_str(),
    ]
    .iter()
    .nth(2)
    .unwrap()
    .split("\n")
    .map(|line| line.split(" ").collect::<Vec<_>>())
    .collect::<Vec<_>>();

    let mut c = ClockCircuit::new();

    for i in 0..input.len() {
        match input[i][0] {
            "addx" => c.addx(input[i][1].parse::<i32>().expect("could not parse int val")),
            "noop" => c.noop(),
            _ => {}
        }
    }
}
