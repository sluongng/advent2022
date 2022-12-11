use std::collections::{HashMap, VecDeque};
use std::fs;

#[allow(dead_code)]
struct Monkey {
    name: String,
    items: VecDeque<HashMap<i32, i32>>,
    op: Box<dyn Fn(i32) -> i32>,
    divisible_test_val: i32,
    test_pass_monkey: usize,
    test_fail_monkey: usize,
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
        .split('\n')
        .map(|line| {
            let mut temp = line.split(':').map(str::trim).map(|s| s.into());

            (temp.next().unwrap(), temp.next())
        })
        .collect::<Vec<_>>()
}

fn to_monkey(lines: Vec<(String, Option<String>)>) -> Monkey {
    // handle first line for Monkey's name
    let mut m = Monkey::new(lines[0].0.to_string());

    lines.iter().skip(1).for_each(|l| match l.0.as_str() {
        "Starting items" => {
            l.1.as_ref()
                .unwrap()
                .split(", ")
                .map(|i| i.parse::<i32>().unwrap())
                .for_each(|i| {
                    let mut hm: HashMap<i32, i32> = HashMap::new();
                    hm.insert(0, i);

                    m.items.push_back(hm);
                })
        }
        "Operation" => {
            let tokens =
                l.1.as_ref()
                    .unwrap()
                    .split(" = ")
                    .nth(1)
                    .unwrap()
                    .split(' ')
                    .collect::<Vec<_>>();

            // Copy these tokens into new string so that we could
            // store them within the 'op' closure later.
            let lhs = tokens[0].to_string();
            let sig = tokens[1].to_string();
            let rhs = tokens[2].to_string();

            let op = move |n: i32| -> i32 {
                let lh = match lhs.as_str() {
                    "old" => n,
                    num => num.parse::<i32>().unwrap(),
                };
                let rh = match rhs.as_str() {
                    "old" => n,
                    num => num.parse::<i32>().unwrap(),
                };

                match sig.as_str() {
                    "+" => lh + rh,
                    "-" => lh - rh,
                    "*" => lh * rh,
                    "/" => lh / rh,
                    // Never happen
                    _ => 0,
                }
            };

            m.op = Box::new(op);
        }
        "Test" => {
            m.divisible_test_val =
                l.1.as_ref()
                    .unwrap()
                    .split(' ')
                    .last()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
        }
        "If true" => {
            m.test_pass_monkey =
                l.1.as_ref()
                    .unwrap()
                    .split(' ')
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
        }
        "If false" => {
            m.test_fail_monkey =
                l.1.as_ref()
                    .unwrap()
                    .split(' ')
                    .last()
                    .unwrap()
                    .parse::<usize>()
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

    let dividers = monkeys
        .iter()
        .map(|m| m.divisible_test_val)
        .collect::<Vec<_>>();
    for m in &mut monkeys {
        for hm in &mut m.items {
            for &d in dividers.iter() {
                let result = hm.get(&0).unwrap() % d;
                hm.insert(d, result);
            }

            hm.remove(&0);
        }
    }

    let mut monkey_business = vec![0; monkeys.len()];
    for round in 1..=10_000 {
        for i in 0..monkeys.len() {
            monkey_business[i] += monkeys[i].items.len();

            while let Some(mut item) = monkeys[i].items.pop_front() {
                for (div, val) in item.clone() {
                    item.insert(div, (monkeys[i].op)(val) % div);
                }

                let mnk_idx = match item.get(&monkeys[i].divisible_test_val).unwrap() {
                    0 => monkeys[i].test_pass_monkey,
                    _ => monkeys[i].test_fail_monkey,
                };

                monkeys[mnk_idx].items.push_back(item);
            }
        }

        if round == 1 || round == 20 || round % 1000 == 0 {
            println!("\nRound: {}", round);
            println!("bzn: {:?}", monkey_business);
        }
    }

    monkey_business.sort();
    println!(
        "Total bzn: {}",
        monkey_business.pop().unwrap() * monkey_business.pop().unwrap()
    );
}
