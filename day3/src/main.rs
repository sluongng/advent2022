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
    .fold(vec![], groups_of_three)
    .iter()
    .map(group_to_badge)
    .map(badge_to_priority)
    .sum();

    println!("{}", sum);
}

// Fold the lines into a big group,
// where each group is a vector of 3 lines.
fn groups_of_three<'a>(mut groups: Vec<Vec<&'a str>>, line: &'a str) -> Vec<Vec<&'a str>> {
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
}

// Find a common badge between 3 lines inside a group
fn group_to_badge(group: &Vec<&str>) -> char {
    group
        .iter()
        .map(|line| line.chars().collect::<HashSet<char>>())
        .reduce(set_of_commons)
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .clone()
}

fn set_of_commons(set1: HashSet<char>, set2: HashSet<char>) -> HashSet<char> {
    set1.intersection(&set2).map(|h| *h).collect()
}

fn badge_to_priority(b: char) -> u32 {
    let p = b.to_digit(36).unwrap() - 'a'.to_digit(36).unwrap() + 1;
    if b.is_uppercase() {
        p + 26
    } else {
        p
    }
}
