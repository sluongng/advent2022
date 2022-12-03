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
        g.iter()
            .map(|line| line.chars().collect::<HashSet<char>>())
            .reduce(|acc, h| acc.intersection(&h).map(|h| *h).collect())
            .unwrap()
            .iter()
            .next()
            .unwrap()
            .clone()
    })
    // calculate value of each badge
    .map(|b| (b, b.to_digit(36).unwrap() - 'a'.to_digit(36).unwrap() + 1))
    .map(|(b, val)| if b.is_uppercase() { val + 26 } else { val })
    .sum();

    println!("{}", sum);
}
