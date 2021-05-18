use std::collections::HashSet;

#[test]
fn part1() {
    let ans = include_str!("res/04.txt").lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
        .filter(|tokens| {
            let set: HashSet<_> = tokens.iter().collect();
            return set.len() == tokens.len();
        })
        .count();
    println!("Day 4, part 1: {}", ans);
    assert_eq!(386, ans);
}

#[test]
fn part2() {
    let ans = include_str!("res/04.txt").lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<&str>>())
        .filter(|tokens| {
            let set: HashSet<String> = tokens.iter().map(|token| {
                let mut chars: Vec<char> = token.chars().collect();
                chars.sort_by(|a, b| b.cmp(a));
                chars.into_iter().collect()
            }).collect();
            println!("{:?}", set);
            return set.len() == tokens.len();
        })
        .count();
    println!("Day 4, part 2: {}", ans);
    assert_eq!(208, ans);
}