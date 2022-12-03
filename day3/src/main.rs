use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let sum: u32 = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"
    .trim()
    .split("\n")
    // Fold the lines into a big group,
    // where each group is a vector of 3 lines.
    .fold(vec![], |mut groups, line| {
        match groups.last_mut() {
            None => groups.push(vec![line]),
            Some(g) => {
                if g.len() >= 3 {
                    groups.push(vec![line]);
                } else {
                    g.push(line);
                }
            }
        }

        groups
    })
    .iter()
    // Find a common badge between 3 lines inside a group
    .map(|g| {
        let mut item_count = HashMap::new();
        let mut res = 'a';

        // TODO: rewrite in functional style?
        'group_loop: for line in g {
            for c in line.chars().collect::<HashSet<char>>() {
                item_count
                    .entry(c)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);

                match item_count.get(&c) {
                    Some(counter) => {
                        if *counter >= 3 {
                            res = c;
                            break 'group_loop;
                        }
                    }
                    None => {}
                }
            }
        }

        res
    })
    // calculate value of each badge
    .map(|b| (b, b.to_digit(36).unwrap() - 'a'.to_digit(36).unwrap() + 1))
    .map(|(b, val)| if b.is_uppercase() { val + 26 } else { val })
    .sum();

    println!("{}", sum);
}
