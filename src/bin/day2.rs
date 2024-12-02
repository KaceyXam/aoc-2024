fn main() {
    let input = include_str!("../inputs/day2.txt");

    let res_one = part_one(&input);
    println!("Part 1: {res_one}");

    let res_two = part_two(&input);
    println!("Part 2: {res_two}");
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|l| is_valid(l.to_vec()))
        .count()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|c| c.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .filter(|l| {
            let len = l.len();
            if !is_valid(l.to_vec()) {
                for i in 0..len {
                    let mut list = l.clone();
                    list.remove(i);
                    if is_valid(list) {
                        return true;
                    }
                }
                return false;
            }
            true
        })
        .count()
}

fn is_valid(line: Vec<i32>) -> bool {
    (line.windows(2).all(|l| l[0] < l[1]) || line.windows(2).all(|l| l[0] > l[1]))
        && line.windows(2).all(|l| (l[0] - l[1]).abs() <= 3)
}
