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
    .map(|mut halves| {
        let mut first = halves
            .next()
            .unwrap()
            .split("-")
            .take(2)
            .map(|position| position.parse::<i32>());
        let mut second = halves
            .next()
            .unwrap()
            .split("-")
            .take(2)
            .map(|position| position.parse::<i32>());

        let first_a = first.next().unwrap().ok().unwrap();
        let first_b = first.next().unwrap().ok().unwrap();
        let second_a = second.next().unwrap().ok().unwrap();
        let second_b = second.next().unwrap().ok().unwrap();

        let first_size = first_b - first_a;
        let second_size = second_b - second_a;

        let mut smaller = (first_a, first_b);
        let mut bigger = (second_a, second_b);

        if first_size > second_size {
            (smaller, bigger) = (bigger, smaller);
        }

        smaller.0 >= bigger.0 && smaller.1 <= bigger.1
    })
    .filter(|x| *x)
    .count();

    println!("{}", output);
}
