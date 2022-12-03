fn main() {
    let output: u32 = "
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"
    .trim()
    .split("\n")
    .map(|line| line.split_at(line.len() / 2))
    .map(|(first, second)| {
        let mut item = 'a';

        for f in first.chars() {
            for s in second.chars() {
                if f == s {
                    item = f;
                }
            }
        }

        item
    })
    .map(|c| {
        let mut d = c.to_digit(36).unwrap() - 'a'.to_digit(36).unwrap() + 1;
        if c.is_uppercase() {
            d += 26;
        }

        d
    })
    .sum();

    println!("{}", output);
}
