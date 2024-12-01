fn main() {
    let input = include_str!("../inputs/day1.txt");

    let part_one_res = part_one(input);
    println!("Part 1: {part_one_res}");

    let part_two_res = part_two(input);
    println!("Part 2: {part_two_res}");
}

fn part_one(input: &str) -> usize {
    let mut a: Vec<i32> = vec![];
    let mut b: Vec<i32> = vec![];

    for line in input.lines() {
        let mut nums = line.split_whitespace();
        a.push(nums.next().unwrap().parse().unwrap());
        b.push(nums.next().unwrap().parse().unwrap());
    }

    a.sort();
    b.sort();

    a.iter()
        .zip(b.iter())
        .map(|(a, b)| (a - b).abs() as usize)
        .sum()
}

fn part_two(input: &str) -> usize {
    let mut a: Vec<usize> = vec![];
    let mut b: Vec<usize> = vec![];

    for line in input.split("\n") {
        if line == "" {
            break;
        }
        let mut nums = line.split_whitespace();
        a.push(nums.next().unwrap().parse().unwrap());
        b.push(nums.next().unwrap().parse().unwrap());
    }

    a.iter()
        .map(|i| b.iter().filter(|&n| n == i).count() * i)
        .sum()
}
