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
    let input = [
        "noop
addx 3
addx -5",
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
        "addx 1
noop
addx 29
addx -24
addx 4
addx 3
addx -2
addx 3
addx 1
addx 5
addx 3
addx -2
addx 2
noop
noop
addx 7
noop
noop
noop
addx 5
addx 1
noop
addx -38
addx 21
addx 8
noop
addx -19
addx -2
addx 2
addx 5
addx 2
addx -12
addx 13
addx 2
addx 5
addx 2
addx -18
addx 23
noop
addx -15
addx 16
addx 7
noop
noop
addx -38
noop
noop
noop
noop
noop
noop
addx 8
addx 2
addx 3
addx -2
addx 4
noop
noop
addx 5
addx 3
noop
addx 2
addx 5
noop
noop
addx -2
noop
addx 3
addx 6
noop
addx -38
addx -1
addx 35
addx -6
addx -19
addx -2
addx 2
addx 5
addx 2
addx 3
noop
addx 2
addx 3
addx -2
addx 2
noop
addx -9
addx 16
noop
addx 9
addx -3
addx -36
addx -2
addx 11
addx 22
addx -28
noop
addx 3
addx 2
addx 5
addx 2
addx 3
addx -2
addx 2
noop
addx 3
addx 2
noop
addx -11
addx 16
addx 2
addx 5
addx -31
noop
addx -6
noop
noop
noop
noop
noop
addx 7
addx 30
addx -24
addx -1
addx 5
noop
noop
noop
noop
noop
addx 5
noop
addx 5
noop
addx 1
noop
addx 2
addx 5
addx 2
addx 1
noop
noop
noop
noop",
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
