#[test]
fn part1() {
    let input: Vec<_> = include_str!("res/01.txt")
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let partial_sum: u32 = input
        .windows(2)
        .filter_map(|pair| {
            if pair[0] == pair[1] {
                Some(pair[0])
            } else {
                None
            }
        })
        .sum();
    let ans = if input.first().unwrap() == input.last().unwrap() {
        partial_sum + input[0]
    } else {
        partial_sum
    };
    println!("Day 1, part 1: {}", ans);
    assert_eq!(1029, ans);
}

#[test]
fn part2() {
    let input: Vec<u32> = include_str!("res/01.txt")
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let length = input.len();
    let step = length / 2;
    let ans: u32 = input
        .iter()
        .enumerate()
        .filter_map(|(index, digit)| {
            let sibling = (index + step) % length;
            if *digit == input[sibling] {
                Some(digit)
            } else {
                None
            }
        })
        .sum();
    println!("Day 1, part 2: {}", ans);
    assert_eq!(1220, ans);
}
