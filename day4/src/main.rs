fn main() {
    let output = "
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"
    .trim()
    .split("\n")
    .map(|line| line.split(",").take(2))
    .map(|halves| halves.map(half_to_pos).collect::<Vec<Vec<i32>>>())
    .map(is_overlap)
    .filter(|x| *x)
    .count();

    println!("{}", output);
}

// half_to_pos take a string of half the line and return the parsed coordinates
//
// Example:
//   "2-4" -> !vec[2, 4]
fn half_to_pos(half: &str) -> Vec<i32> {
    half.split("-")
        .take(2)
        .map(|position| position.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
}

fn is_overlap(positions: Vec<Vec<i32>>) -> bool {
    positions[0][0] <= positions[1][1] && positions[0][1] >= positions[1][0]
}
