use serde_json::Value;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input1.txt").unwrap();

    let pairs: Vec<(Value, Value)> = input
        .trim()
        .split("\n\n")
        .map(|pair| {
            let mut iter = pair
                .split('\n')
                .map(|line| serde_json::from_str(line).expect("line is not json"));

            (iter.next().unwrap(), iter.next().unwrap())
        })
        .collect();

    println!("{:?}", pairs);
}
