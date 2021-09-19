use std::collections::VecDeque;

fn lowest_16_match(a: u64, b: u64) -> bool {
    let a = a & u16::MAX as u64;
    let b = b & u16::MAX as u64;
    a == b
}

#[test]
fn part1() {
    let (mut a, mut b) = (873, 583);
    let mut sum = 0;
    for _ in 0..40_000_000 {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;
        if lowest_16_match(a, b) {
            sum += 1;
        }
    }
    println!("Day 15, part 1: {}", sum);
}

#[test]
fn part2() {
    let (mut gen_a, mut gen_b) = (873, 583);
    let (mut vec_a, mut vec_b) = (VecDeque::new(), VecDeque::new());
    let mut evals = 0;
    let mut sum = 0;
    while evals < 5_000_000 {
        gen_a = (gen_a * 16807) % 2147483647;
        gen_b = (gen_b * 48271) % 2147483647;
        if gen_a % 4 == 0 {
            vec_a.push_back(gen_a)
        }
        if gen_b % 8 == 0 {
            vec_b.push_back(gen_b);
        }
        if let (Some(&a), Some(&b)) = (vec_a.front(), vec_b.front()) {
            sum += if lowest_16_match(a, b) { 1 } else { 0 };
            evals += 1;
            if evals % 100_000 == 0 {
                println!(
                    "evals: {}, vec_a: {}, vec_b: {}",
                    evals,
                    vec_a.len(),
                    vec_b.len()
                );
            }
            vec_a.pop_front();
            vec_b.pop_front();
        }
    }
    println!("Day 15, part 2: {}", sum);
}
