use std::collections::VecDeque;
use std::fs;

struct Monkey {
    name: String,
    items: VecDeque<i32>,
    op: Box<dyn Fn(i32) -> i32>,
    divisible_test_val: i32,
    test_pass_monkey: i32,
    test_fail_monkey: i32,
}

impl Monkey {
    fn new(name: String) -> Self {
        Monkey {
            name,
            items: VecDeque::new(),
            op: Box::new(|n| n),
            divisible_test_val: 0,
            test_pass_monkey: 0,
            test_fail_monkey: 0,
        }
    }
}

fn block_to_pairs(block: &str) -> Vec<(String, Option<String>)> {
    block
        .split("\n")
        .map(|line| {
            let mut temp = line.split(":").map(str::trim).map(|s| s.into());

            (temp.next().unwrap(), temp.next())
        })
        .collect::<Vec<_>>()
}

fn to_monkey(lines: Vec<(String, Option<String>)>) -> Monkey {
    // handle first line for Monkey's name
    let mut m = Monkey::new(lines[0].0.clone());

    lines.iter().skip(1).for_each(|l| match l.0.as_str() {
        "Starting items" => {
            l.1.as_ref()
                .unwrap()
                .split(", ")
                .map(|i| i.parse::<i32>().unwrap())
                .for_each(|i| m.items.push_back(i))
        }
        "Operation" => {
            let tokens =
                l.1.as_ref()
                    .unwrap()
                    .split(" = ")
                    .nth(1)
                    .unwrap()
                    .split(" ")
                    .collect::<Vec<_>>();

            let lhs = tokens[0].to_owned();
            let rhs = tokens[2].to_owned();
            let sign = tokens[1].to_owned();

            let op = move |n: i32| -> i32 {
                let lh = match lhs.as_str() {
                    "old" => n,
                    num => num.parse::<i32>().unwrap(),
                };
                let rh = match rhs.as_str() {
                    "old" => n,
                    num => num.parse::<i32>().unwrap(),
                };

                match sign.as_str() {
                    "+" => lh + rh,
                    "-" => lh - rh,
                    "*" => lh * rh,
                    "/" => lh / rh,
                    _ => 0,
                }
            };

            m.op = Box::new(op);
        }
        "Test" => {
            m.divisible_test_val =
                l.1.as_ref()
                    .unwrap()
                    .split(" ")
                    .last()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
        }
        "If true" => {
            m.test_pass_monkey =
                l.1.as_ref()
                    .unwrap()
                    .split(" ")
                    .last()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
        }
        "If false" => {
            m.test_fail_monkey =
                l.1.as_ref()
                    .unwrap()
                    .split(" ")
                    .last()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
        }
        _ => (),
    });

    m
}

fn main() {
    let input = fs::read_to_string("./src/input2.txt").unwrap();
    let mut monkeys = input
        .as_str()
        .split("\n\n")
        .map(block_to_pairs)
        .map(to_monkey)
        .collect::<Vec<_>>();

    let mut monkey_business = vec![0; monkeys.len()];
    for iter in 0..20 {
        println!("\nIter: {}", iter);

        for i in 0..monkeys.len() {
            monkey_business[i] += monkeys[i].items.len();

            while let Some(item) = monkeys[i].items.pop_front() {
                let level = (monkeys[i].op)(item) / 3;

                let mnk_idx = match level % monkeys[i].divisible_test_val {
                    0 => monkeys[i].test_pass_monkey as usize,
                    _ => monkeys[i].test_fail_monkey as usize,
                };
                monkeys[mnk_idx].items.push_back(level);
            }
        }

        for m in monkeys.iter() {
            println!("{}: {:?}", m.name, m.items);
        }
        println!("bzn: {:?}", monkey_business);
    }

    monkey_business.sort();

    println!(
        "Total bzn: {}",
        monkey_business.pop().unwrap() * monkey_business.pop().unwrap()
    );
}
