#[test]
fn part1() {
    let mut input: Vec<i32> = include_str!("res/05.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut cur: isize = 0;
    let mut count = 0;
    while let Some(jump) = if cur >= 0 {
        input.get_mut(cur as usize)
    } else {
        None
    } {
        cur += *jump as isize;
        *jump += 1;
        count += 1;
    }
    println!("Day 5, part 1: {}", count);
    assert_eq!(318883, count);
}

#[test]
fn part2() {
    let mut input: Vec<i32> = include_str!("res/05.txt")
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();
    let mut cur: isize = 0;
    let mut count = 0;
    while let Some(jump) = if cur >= 0 {
        input.get_mut(cur as usize)
    } else {
        None
    } {
        cur += *jump as isize;
        if *jump >= 3 {
            *jump -= 1;
        } else {
            *jump += 1;
        }
        count += 1;
    }
    println!("Day 5, part 2: {}", count);
    assert_eq!(23948711, count);
}
