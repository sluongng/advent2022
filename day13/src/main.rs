use serde_json::{json, Value};
use std::cmp::Ordering;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input2.txt").unwrap();

    let p1_result: usize = input
        .trim()
        .split("\n\n")
        .map(|pair| {
            let mut iter = pair
                .split('\n')
                .map(|line| serde_json::from_str(line).expect("line is not json"));

            (iter.next().unwrap(), iter.next().unwrap())
        })
        .map(is_ordered)
        .enumerate()
        .filter(|(_, b)| *b == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum();

    println!("p1 result: {}", p1_result);

    let mut all_lines: Vec<Value> = input
        .trim()
        .split("\n\n")
        .flat_map(|pair| {
            pair.split('\n')
                .map(|line| serde_json::from_str(line).expect("line is not json"))
        })
        .collect();

    all_lines.extend(vec![2, 6].iter().map(|i| json!(i)));
    all_lines.sort_by(|a, b| is_ordered((a.clone(), b.clone())));

    let p2_result: usize = all_lines
        .iter()
        .enumerate()
        .filter(|(_, v)| match v {
            Value::Number(n) => n.as_i64().unwrap() == 2 || n.as_i64().unwrap() == 6,
            _ => false,
        })
        .map(|(i, _)| i + 1)
        .product();

    println!("p2_result: {:?}", p2_result)
}

fn is_ordered((left, right): (Value, Value)) -> Ordering {
    match (left, right) {
        (Value::Number(l), Value::Number(r)) => {
            let (l64, r64) = (l.as_i64().unwrap(), r.as_i64().unwrap());

            l64.cmp(&r64)
        }
        (Value::Number(l), Value::Array(r)) => is_ordered((json!([l]), json!(r))),
        (Value::Array(l), Value::Number(r)) => is_ordered((json!(l), json!([r]))),
        (Value::Array(l), Value::Array(r)) => match (l.len(), r.len()) {
            (0, 0) => Ordering::Equal,
            (0, _) => Ordering::Less,
            (_, 0) => Ordering::Greater,
            (l_len, r_len) => {
                let res = is_ordered((l[0].clone(), r[0].clone()));

                if res.is_ne() {
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
