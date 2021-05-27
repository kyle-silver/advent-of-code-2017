use std::collections::{HashMap, HashSet};

const RAW: &str = include_str!("res/12.txt");

#[test]
fn part1() {
    let map: HashMap<_,_> = RAW.lines().map(parse_line).collect();
    let set = find_all(0, &map);
    println!("{}", set.len());
    assert_eq!(115, set.len());
}

fn parse_line(line: &str) -> (u32, Vec<u32>) {
    let (left, right) = line.split_once(" <-> ").unwrap();
    let left = left.parse::<u32>().unwrap();
    let right: Vec<u32> = right.split(',').map(|t| t.trim().parse().unwrap()).collect();
    (left, right)
}

fn find_all(seed: u32, candidates: &HashMap<u32, Vec<u32>>) -> HashSet<u32> {
    let mut set = HashSet::new();
    set.insert(seed);
    let mut prev_len = 0;
    while prev_len < set.len() {
        prev_len = set.len();
        for (k, v) in candidates {
            if set.contains(k) {
                for &e in v {
                    set.insert(e);
                }
            }
        }
    }
    set
}

#[test]
fn part2() {
    let mut map: HashMap<_,_> = RAW.lines().map(parse_line).collect();
    let mut count = 0;
    while !map.is_empty() {
        let &seed = map.keys().next().unwrap();
        let network = find_all(seed, &map);
        for item in network {
            map.remove(&item);
        }
        count += 1;
    }
    println!("Day 12, part 2: {}", count);
    assert_eq!(221, count);
}