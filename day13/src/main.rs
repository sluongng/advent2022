use serde_json::{json, Value};
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input2.txt").unwrap();

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
        .filter(|(_, b)| *b == 1)
        .map(|(i, _)| i + 1)
        .inspect(|num| println!("Pair Num: {}", num))
        .sum();

    println!("result: {}", result);
}

fn is_ordered((left, right): (Value, Value)) -> i32 {
    println!("Comparing: {:?} vs {:?}", left, right);

    match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            let (l64, r64) = (l.as_i64().unwrap(), r.as_i64().unwrap());

            if l64 == r64 {
                0
            } else if l64 < r64 {
                1
            } else {
                -1
            }
        }
        (Value::Number(l), Value::Array(r)) => is_ordered((json!([l]), json!(r))),
        (Value::Array(l), Value::Number(r)) => is_ordered((json!(l), json!([r]))),
        (Value::Array(l), Value::Array(r)) => match (l.len(), r.len()) {
            (0, 0) => 0,
            (0, _) => 1,
            (_, 0) => -1,
            (l_len, r_len) => {
                let res = is_ordered((l[0].clone(), r[0].clone()));

                if res != 0 {
                    res
                } else {
                    is_ordered((json!(l[1..l_len]), json!(r[1..r_len])))
                }
            }
        },
        _ => {
            panic!("left and right should be either Number or List")
        }
    }
}
