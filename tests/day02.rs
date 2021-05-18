#[test]
fn part1() {
    let raw = include_str!("res/02.txt");
    let input: Vec<Vec<u32>> = raw.lines()
        .map(|line| line.split_ascii_whitespace().map(|t| t.parse().unwrap()).collect())
        .collect();
    let ans: u32 = input.iter()
        .map(|v| minmax(v))
        .map(|(min, max)| max - min)
        .sum();
    println!("Day 2, part 1: {}", ans);
    assert_eq!(47623, ans)
}

fn minmax(line: &[u32]) -> (u32, u32) {
    let (mut min, mut max) = (line[0], line[0]);
    for i in line {
        if *i < min {
            min = *i;
        }
        if *i > max {
            max = *i;
        }
    }
    return (min, max)
}

#[test]
fn part2() {
    let raw = include_str!("res/02.txt");
    let input: Vec<Vec<u32>> = raw.lines()
        .map(|line| line.split_ascii_whitespace().map(|t| t.parse().unwrap()).collect())
        .collect();
    let ans: u32 = input.iter()
        .map(|line| find_dividend_and_divisor(line))
        .map(|(dividend, divisor)| dividend / divisor)
        .sum();
    println!("Day 2, part 2: {}", ans);
    assert_eq!(312, ans)
}

fn find_dividend_and_divisor(line: &[u32]) -> (u32, u32) {
    for dividend in line {
        for divisor in line {
            if dividend == divisor {
                continue;
            }
            if dividend % divisor == 0 {
                return (*dividend, *divisor);
            }
        }
    }
    panic!("No evenly divisible pair found");
}