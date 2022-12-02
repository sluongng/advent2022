use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let input = "
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"
    .trim()
    .split("\n");

    let mut queue = BinaryHeap::new();

    let mut current_elf = 0;
    for i in input {
        if i.is_empty() {
            queue.push(Reverse(current_elf.clone()));
            if queue.len() > 3 {
                queue.pop();
            }

            current_elf = 0;
            continue;
        }

        let calories = i.parse::<i32>().unwrap();
        current_elf += calories;
    }

    let mut sum = 0;
    for i in queue.iter() {
        sum += i.0;
    }

    println!("{}", sum);
}
