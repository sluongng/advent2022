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
        // There is not a good way to tell Rust that we gona have
        // a guarentee matched in 3 loops.
        // So init result value with a dummy character to satisfy Rust.
        let mut res = 'a';
        let mut item_count = HashMap::new();

        // Run through each line and add all chars into a Set to ensure uniqueness
        // Then add items from the Set into a Map with counter value
        // When we reach last line(3rd line), break on the first char with 3 counts
        //
        // TODO: rewrite in functional style?
        'group_loop: for line in g {
            for c in line.chars().collect::<HashSet<char>>() {
                // add char to counter map
                item_count
                    .entry(c)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);

                // check counter
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
