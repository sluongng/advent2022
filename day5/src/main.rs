fn main() {
    let temp_input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    let real_input = "        [J]         [B]     [T]    
        [M] [L]     [Q] [L] [R]    
        [G] [Q]     [W] [S] [B] [L]
[D]     [D] [T]     [M] [G] [V] [P]
[T]     [N] [N] [N] [D] [J] [G] [N]
[W] [H] [H] [S] [C] [N] [R] [W] [D]
[N] [P] [P] [W] [H] [H] [B] [N] [G]
[L] [C] [W] [C] [P] [T] [M] [Z] [W]
 1   2   3   4   5   6   7   8   9 

move 6 from 6 to 5
move 2 from 5 to 9
move 8 from 9 to 1
move 3 from 5 to 4
move 9 from 1 to 8
move 2 from 1 to 5
move 1 from 1 to 8
move 14 from 8 to 2
move 1 from 1 to 2
move 2 from 6 to 8
move 2 from 5 to 7
move 6 from 8 to 6
move 4 from 4 to 2
move 2 from 4 to 9
move 5 from 7 to 4
move 2 from 7 to 5
move 6 from 2 to 4
move 2 from 4 to 7
move 4 from 5 to 8
move 1 from 5 to 2
move 3 from 3 to 5
move 3 from 8 to 3
move 4 from 3 to 7
move 2 from 9 to 8
move 1 from 3 to 7
move 1 from 6 to 8
move 5 from 7 to 1
move 3 from 7 to 2
move 1 from 6 to 3
move 2 from 5 to 9
move 5 from 4 to 2
move 3 from 5 to 9
move 5 from 9 to 6
move 2 from 1 to 3
move 4 from 4 to 1
move 2 from 8 to 1
move 18 from 2 to 5
move 3 from 4 to 1
move 1 from 1 to 2
move 1 from 6 to 8
move 1 from 7 to 1
move 10 from 1 to 5
move 1 from 1 to 5
move 3 from 8 to 1
move 2 from 1 to 5
move 3 from 6 to 5
move 8 from 2 to 9
move 2 from 9 to 7
move 3 from 3 to 8
move 1 from 4 to 8
move 3 from 5 to 3
move 15 from 5 to 8
move 4 from 6 to 1
move 2 from 7 to 4
move 9 from 5 to 7
move 1 from 6 to 8
move 5 from 3 to 5
move 5 from 7 to 5
move 3 from 1 to 5
move 2 from 4 to 8
move 3 from 1 to 6
move 20 from 5 to 4
move 1 from 7 to 6
move 21 from 8 to 2
move 1 from 3 to 7
move 2 from 4 to 2
move 1 from 7 to 1
move 18 from 2 to 8
move 3 from 9 to 2
move 1 from 6 to 4
move 1 from 1 to 9
move 8 from 8 to 6
move 4 from 8 to 2
move 1 from 2 to 6
move 7 from 8 to 5
move 2 from 5 to 3
move 1 from 9 to 5
move 5 from 2 to 4
move 1 from 3 to 7
move 2 from 5 to 7
move 4 from 4 to 9
move 2 from 5 to 9
move 6 from 2 to 8
move 3 from 7 to 3
move 2 from 5 to 4
move 4 from 8 to 2
move 2 from 7 to 4
move 7 from 6 to 4
move 1 from 8 to 4
move 3 from 6 to 7
move 2 from 7 to 2
move 7 from 9 to 7
move 1 from 9 to 2
move 3 from 3 to 6
move 3 from 7 to 4
move 2 from 7 to 9
move 6 from 4 to 1
move 3 from 7 to 9
move 1 from 8 to 5
move 1 from 3 to 6
move 3 from 9 to 4
move 2 from 6 to 4
move 3 from 9 to 1
move 4 from 2 to 8
move 1 from 8 to 5
move 9 from 1 to 2
move 1 from 6 to 5
move 1 from 7 to 2
move 1 from 8 to 1
move 2 from 8 to 9
move 1 from 9 to 8
move 1 from 5 to 7
move 1 from 7 to 6
move 1 from 9 to 8
move 1 from 6 to 3
move 26 from 4 to 3
move 1 from 5 to 8
move 3 from 6 to 3
move 7 from 4 to 3
move 1 from 1 to 3
move 1 from 4 to 8
move 13 from 3 to 1
move 1 from 3 to 4
move 12 from 2 to 5
move 20 from 3 to 2
move 1 from 4 to 1
move 4 from 5 to 7
move 1 from 7 to 8
move 9 from 5 to 2
move 5 from 1 to 5
move 21 from 2 to 8
move 5 from 8 to 4
move 4 from 5 to 2
move 6 from 1 to 7
move 1 from 5 to 4
move 4 from 3 to 1
move 6 from 1 to 3
move 1 from 1 to 9
move 6 from 8 to 7
move 4 from 8 to 2
move 4 from 2 to 7
move 5 from 4 to 1
move 8 from 8 to 4
move 1 from 9 to 6
move 18 from 7 to 6
move 15 from 6 to 5
move 2 from 6 to 8
move 2 from 6 to 3
move 8 from 3 to 7
move 15 from 5 to 7
move 3 from 4 to 9
move 12 from 2 to 3
move 3 from 9 to 4
move 6 from 7 to 9
move 9 from 4 to 5
move 10 from 3 to 5
move 9 from 5 to 2
move 14 from 7 to 8
move 14 from 8 to 5
move 4 from 2 to 4
move 1 from 4 to 6
move 2 from 8 to 4
move 3 from 8 to 9
move 18 from 5 to 1
move 1 from 5 to 9
move 1 from 7 to 4
move 5 from 5 to 9
move 3 from 2 to 4
move 13 from 9 to 2
move 13 from 2 to 6
move 1 from 7 to 3
move 3 from 3 to 1
move 9 from 6 to 5
move 1 from 7 to 8
move 20 from 1 to 8
move 2 from 2 to 8
move 5 from 6 to 9
move 15 from 8 to 7
move 3 from 5 to 3
move 5 from 1 to 3
move 2 from 3 to 4
move 3 from 9 to 5
move 4 from 5 to 2
move 4 from 5 to 7
move 3 from 4 to 9
move 10 from 7 to 8
move 2 from 9 to 4
move 1 from 5 to 6
move 8 from 7 to 9
move 1 from 6 to 7
move 6 from 3 to 4
move 12 from 9 to 8
move 1 from 1 to 5
move 2 from 7 to 8
move 1 from 7 to 5
move 1 from 9 to 5
move 2 from 2 to 9
move 11 from 8 to 1
move 7 from 1 to 5
move 3 from 1 to 6
move 5 from 8 to 9
move 8 from 4 to 3
move 4 from 4 to 6
move 5 from 9 to 3
move 4 from 4 to 5
move 2 from 6 to 7
move 1 from 9 to 5
move 2 from 7 to 4
move 12 from 5 to 2
move 8 from 8 to 9
move 8 from 8 to 6
move 9 from 6 to 2
move 4 from 9 to 2
move 1 from 5 to 1
move 5 from 2 to 1
move 2 from 5 to 4
move 5 from 2 to 5
move 5 from 5 to 6
move 3 from 4 to 7
move 11 from 2 to 7
move 2 from 2 to 1
move 4 from 3 to 7
move 2 from 2 to 4
move 6 from 1 to 4
move 1 from 2 to 8
move 2 from 9 to 5
move 4 from 4 to 3
move 5 from 4 to 1
move 2 from 2 to 1
move 1 from 8 to 5
move 14 from 7 to 6
move 3 from 9 to 2
move 15 from 6 to 8
move 4 from 1 to 3
move 2 from 2 to 3
move 1 from 1 to 7
move 2 from 3 to 5
move 4 from 5 to 4
move 1 from 3 to 5
move 5 from 1 to 6
move 12 from 6 to 7
move 7 from 8 to 4
move 12 from 7 to 9
move 4 from 7 to 9
move 1 from 2 to 8
move 12 from 9 to 4
move 23 from 4 to 3
move 1 from 6 to 5
move 3 from 9 to 3
move 1 from 7 to 9
move 1 from 9 to 1
move 1 from 9 to 7
move 42 from 3 to 1
move 3 from 5 to 4
move 5 from 1 to 3
move 3 from 4 to 7
move 1 from 1 to 9
move 4 from 3 to 8
move 1 from 3 to 7
move 1 from 9 to 1
move 2 from 7 to 8
move 8 from 1 to 6
move 2 from 7 to 5
move 9 from 1 to 2
move 5 from 2 to 3
move 3 from 2 to 4
move 20 from 1 to 2
move 1 from 1 to 5
move 1 from 6 to 7
move 3 from 4 to 7
move 2 from 3 to 6
move 3 from 6 to 1
move 1 from 6 to 4
move 2 from 1 to 6
move 3 from 5 to 9
move 1 from 4 to 3
move 2 from 7 to 4
move 6 from 8 to 4
move 1 from 1 to 9
move 1 from 2 to 9
move 2 from 8 to 7
move 3 from 6 to 2
move 5 from 7 to 5
move 4 from 2 to 5
move 4 from 4 to 6
move 3 from 9 to 6
move 4 from 3 to 1
move 1 from 9 to 2
move 7 from 8 to 9
move 4 from 2 to 4
move 2 from 1 to 7
move 3 from 4 to 5
move 4 from 2 to 4
move 1 from 7 to 4
move 4 from 2 to 9
move 7 from 4 to 3
move 1 from 7 to 3
move 6 from 2 to 3
move 2 from 1 to 5
move 10 from 3 to 6
move 2 from 6 to 1
move 2 from 2 to 7
move 2 from 3 to 1
move 1 from 7 to 8
move 11 from 5 to 3
move 2 from 3 to 1
move 4 from 6 to 1
move 1 from 4 to 6
move 8 from 3 to 4
move 2 from 5 to 6
move 3 from 3 to 5
move 1 from 8 to 4
move 1 from 4 to 9
move 2 from 6 to 1
move 1 from 5 to 1
move 9 from 4 to 3
move 5 from 6 to 9
move 5 from 6 to 7
move 13 from 9 to 3
move 5 from 1 to 8
move 4 from 8 to 4
move 10 from 3 to 2
move 3 from 6 to 1
move 2 from 7 to 9
move 1 from 8 to 3
move 1 from 7 to 3
move 1 from 9 to 5
move 1 from 6 to 3
move 7 from 2 to 4
move 3 from 5 to 2
move 8 from 3 to 5
move 7 from 4 to 3
move 5 from 9 to 7
move 1 from 7 to 1
move 9 from 1 to 8
move 9 from 5 to 8
move 2 from 7 to 8
move 3 from 8 to 1
move 10 from 3 to 6
move 1 from 1 to 6
move 5 from 1 to 7
move 3 from 2 to 8
move 7 from 8 to 6
move 7 from 8 to 6
move 1 from 3 to 5
move 5 from 7 to 9
move 4 from 8 to 4
move 3 from 2 to 8
move 1 from 7 to 8
move 3 from 3 to 9
move 3 from 7 to 4
move 1 from 7 to 2
move 9 from 9 to 1
move 5 from 1 to 9
move 4 from 8 to 6
move 1 from 2 to 7
move 1 from 5 to 3
move 1 from 3 to 7
move 1 from 1 to 9
move 1 from 1 to 7
move 5 from 9 to 6
move 2 from 7 to 6
move 10 from 4 to 8
move 1 from 4 to 2
move 1 from 4 to 1
move 1 from 9 to 2
move 3 from 1 to 2
move 1 from 7 to 3
move 1 from 2 to 1
move 16 from 6 to 3
move 9 from 6 to 1
move 6 from 6 to 1
move 5 from 6 to 1
move 3 from 8 to 1
move 11 from 3 to 4
move 1 from 6 to 2
move 3 from 8 to 2
move 4 from 1 to 6
move 5 from 3 to 2
move 1 from 2 to 5
move 1 from 8 to 5
move 5 from 8 to 3
move 4 from 6 to 9
move 2 from 9 to 6
move 3 from 3 to 9
move 1 from 5 to 7
move 5 from 1 to 6
move 3 from 6 to 4
move 2 from 2 to 9
move 8 from 4 to 2
move 9 from 1 to 7
move 3 from 3 to 5
move 3 from 5 to 7
move 12 from 7 to 1
move 5 from 4 to 6
move 1 from 4 to 5
move 7 from 1 to 8
move 5 from 9 to 3
move 1 from 7 to 4
move 10 from 1 to 8
move 1 from 4 to 8
move 4 from 6 to 8
move 1 from 6 to 9
move 2 from 5 to 1
move 4 from 3 to 4
move 1 from 1 to 8
move 4 from 4 to 7
move 2 from 1 to 8
move 4 from 6 to 1
move 3 from 9 to 5
move 1 from 6 to 5
move 1 from 3 to 7
move 24 from 8 to 6
move 3 from 6 to 5
move 4 from 6 to 7
move 1 from 1 to 7
move 7 from 7 to 6
move 7 from 5 to 3
move 13 from 6 to 8
move 3 from 1 to 2
move 7 from 6 to 3
move 12 from 2 to 4
move 4 from 6 to 9
move 6 from 3 to 1
move 1 from 2 to 4
move 2 from 8 to 7
move 2 from 2 to 9
move 6 from 3 to 4
move 12 from 8 to 2
move 18 from 2 to 5
move 10 from 4 to 3
move 4 from 7 to 3
move 5 from 4 to 7
move 3 from 5 to 2
move 4 from 7 to 9
move 1 from 5 to 4
move 3 from 2 to 1
move 4 from 3 to 6
move 7 from 5 to 6
move 2 from 5 to 7
move 5 from 1 to 7
move 9 from 7 to 6
move 8 from 9 to 8
move 1 from 1 to 3
move 1 from 3 to 1
move 10 from 3 to 9
move 8 from 8 to 4
move 1 from 3 to 8
move 1 from 1 to 3
move 6 from 9 to 1
move 5 from 5 to 3
move 5 from 3 to 6
move 1 from 8 to 9
move 19 from 6 to 2
move 13 from 4 to 1
move 4 from 1 to 5
move 6 from 2 to 1
move 2 from 9 to 4
move 1 from 3 to 1
move 9 from 2 to 3
move 4 from 5 to 1
move 5 from 9 to 6
move 4 from 3 to 4
move 3 from 2 to 7
move 2 from 4 to 8
move 6 from 1 to 9
move 1 from 8 to 6
move 4 from 1 to 5
move 3 from 4 to 5
move 1 from 7 to 2
move 11 from 1 to 6
move 1 from 2 to 7
move 5 from 3 to 7
move 1 from 3 to 4
move 1 from 4 to 8
move 3 from 5 to 6
move 8 from 1 to 7
move 1 from 8 to 9
move 1 from 6 to 9
move 1 from 8 to 5
move 11 from 6 to 5
move 12 from 5 to 2
move 1 from 5 to 2
move 8 from 7 to 3
move 1 from 5 to 6
move 2 from 5 to 6
move 3 from 7 to 1
move 6 from 2 to 6
move 1 from 3 to 1
move 1 from 4 to 1
move 4 from 6 to 2
move 5 from 1 to 5
move 10 from 2 to 3
move 2 from 9 to 4
move 4 from 5 to 8
move 2 from 2 to 7
move 12 from 6 to 7
move 1 from 8 to 2
move 10 from 3 to 4
move 2 from 3 to 5
move 1 from 3 to 1";

    let blocks = real_input
        .split("\n\n")
        .take(2)
        .map(|block| block.split("\n").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut stacks: Vec<Vec<&str>> = vec![];

    for (i, line) in blocks[0].iter().rev().enumerate() {
        if i == 0 {
            line.trim().split("   ").for_each(|_| stacks.push(vec![]));
            continue;
        }

        for stack_no in 0..stacks.len() {
            let position = stack_no * 4 + 1;

            if position >= line.len() {
                break;
            }

            let c = &line[position..(position + 1)];

            if c != " " {
                stacks[stack_no].push(c);
            }
        }
    }

    for line in blocks[1].iter() {
        let tokens = line.split(" ").collect::<Vec<_>>();

        let count = tokens[1].parse::<usize>().unwrap();
        let from_stack = tokens[3].parse::<usize>().unwrap() - 1;
        let to_stack = tokens[5].parse::<usize>().unwrap() - 1;

        for _ in 0..count {
            let val = stacks[from_stack].pop().unwrap();
            stacks[to_stack].push(val);
        }
    }

    for s in stacks {
        print!("{}", s.last().unwrap())
    }
    println!("");
}
