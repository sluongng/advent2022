use serde_json::{json, Value};
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input1.txt").unwrap();

    let result: usize = input
        .trim()
        .split("\n\n")
        .map(|pair| {
            let mut iter = pair
                .split('\n')
                .map(|line| serde_json::from_str(line).expect("line is not json"));

            (iter.next().unwrap(), iter.next().unwrap())
        })
        .inspect(|(l, r)| println!("\nStarting pair: {:?} {:?}", l, r))
        .map(|(l, r)| is_ordered((l, r)))
        .inspect(|b| println!("Is Ordered: {}\n", b))
        .enumerate()
        .filter(|(_, b)| *b)
        .map(|(i, _)| i + 1)
        .inspect(|num| println!("Pair Num: {}", num))
        .sum();

    println!("result: {}", result);
}

fn is_ordered((left, right): (Value, Value)) -> bool {
    println!("Comparing: {:?} vs {:?}", left, right);

    match (left, right) {
        (Value::Number(l), Value::Number(r)) => l.as_i64().unwrap() <= r.as_i64().unwrap(),
        (Value::Number(l), Value::Array(r)) => is_ordered((json!([l]), json!(r))),
        (Value::Array(l), Value::Number(r)) => is_ordered((json!(l), json!([r]))),
        (Value::Array(l), Value::Array(r)) => match (l.len(), r.len()) {
            (0, 0) => true,
            (0, _) => true,
            (_, 0) => false,
            _ => {
                if l[0].is_i64()
                    && r[0].is_i64()
                    && l[0].as_i64().unwrap() == r[0].as_i64().unwrap()
                {
                    is_ordered((json!(l[1..l.len()]), json!(r[1..r.len()])))
                } else {
                    is_ordered((l[0].clone(), r[0].clone()))
                }
            }
        },
        _ => {
            panic!("left and right should be either Number or List")
        }
    }
}
