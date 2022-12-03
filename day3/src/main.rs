use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let lines = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"
    .trim()
    .split("\n");

    let mut badges = vec![];
    let mut group = vec![];
    let mut pos = 0;
    for line in lines {
        group.insert(group.len(), line);

        if pos % 3 == 2 {
            let mut item_count = HashMap::new();

            'group_loop: for line in group {
                for c in line.chars().collect::<HashSet<char>>() {
                    item_count
                        .entry(c)
                        .and_modify(|counter| *counter += 1)
                        .or_insert(1);

                    match item_count.get(&c) {
                        Some(counter) => {
                            if *counter >= 3 {
                                badges.insert(badges.len(), c);
                                break 'group_loop;
                            }
                        }
                        None => {}
                    }
                }
            }

            group = vec![];
        }

        pos += 1;
    }

    let mut sum = 0;
    for b in badges {
        sum += b.to_digit(36).unwrap() - 'a'.to_digit(36).unwrap() + 1;
        if b.is_uppercase() {
            sum += 26;
        }
    }

    println!("{}", sum);
}
