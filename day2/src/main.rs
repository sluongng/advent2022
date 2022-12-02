fn main() {
    let score: i32 = "
A Y
B X
C Z
"
    .trim()
    .split("\n")
    .map(|line| line.split(" "))
    .map(|mut tokens| {
        let first_move = tokens.next().unwrap();
        let second_move = tokens.next().unwrap();

        // expecting exact 2 tokens
        (first_move, second_move)
    })
    .map(|(first_move, second_move)| match first_move {
        "A" => match second_move {
            "X" => 3 + 0,
            "Y" => 1 + 3,
            "Z" => 2 + 6,
            // unexpected
            _ => 0,
        },
        "B" => match second_move {
            "X" => 1 + 0,
            "Y" => 2 + 3,
            "Z" => 3 + 6,
            // unexpected
            _ => 0,
        },
        "C" => match second_move {
            "X" => 2 + 0,
            "Y" => 3 + 3,
            "Z" => 1 + 6,
            // unexpected
            _ => 0,
        },
        // unexpected
        _ => 0,
    })
    .sum();

    println!("{}", score);
}
