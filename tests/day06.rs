use std::collections::HashSet;

#[test]
fn part1() {
    let mut set: HashSet<[u32; 16]> = HashSet::new();
    let mut state = [4, 1, 15, 12, 0, 9, 9, 5, 5, 8, 7, 3, 14, 5, 12, 3];
    let mut cycles = 0;

    while let None = set.get(&state) {
        set.insert(state.clone());
        redistribute(&mut state);
        cycles += 1;
    }
    println!("Day 6, part 1: {}", cycles);
    assert_eq!(6681, cycles);
}

fn redistribute<const T: usize>(state: &mut [u32; T]) {
    let mut max = state[0];
    let mut maxindex = 0;
    for (i, k) in state.iter().enumerate() {
        if *k > max {
            max = *k;
            maxindex = i;
        }
    }
    state[maxindex] = 0;
    maxindex += 1;
    for _ in (0..max).rev() {
        state[maxindex % T] += 1;
        maxindex += 1;
    }
}

#[test]
fn part2() {
    let mut set: HashSet<[u32; 16]> = HashSet::new();
    let mut state = [4, 1, 15, 12, 0, 9, 9, 5, 5, 8, 7, 3, 14, 5, 12, 3];
    while let None = set.get(&state) {
        set.insert(state.clone());
        redistribute(&mut state);
    }
    let mut set: HashSet<[u32; 16]> = HashSet::new();
    let mut cycles = 0;
    while let None = set.get(&state) {
        set.insert(state.clone());
        redistribute(&mut state);
        cycles += 1;
    }
    println!("Day 6, part 2: {}", cycles);
    assert_eq!(2392, cycles);
}