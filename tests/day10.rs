fn twist<const N: usize>(pos: usize, length: usize, state: &mut [u32; N]) {
    let to_swap: Vec<u32> = state
        .iter()
        .cycle()
        .skip(pos)
        .take(length)
        .cloned()
        .collect();
    for (i, val) in to_swap.into_iter().rev().enumerate() {
        state[(pos + i) % N] = val
    }
}

#[test]
fn part1() {
    let input: Vec<usize> = vec![
        225, 171, 131, 2, 35, 5, 0, 13, 1, 246, 54, 97, 255, 98, 254, 110,
    ];
    let mut state = [0u32; 256];
    for i in 0..256 {
        state[i] = i as u32;
    }
    let mut pos = 0;
    for (skip_size, &length) in input.iter().enumerate() {
        twist(pos, length, &mut state);
        pos = (pos + length + skip_size) % 256;
    }
    let ans = state[0] * state[1];
    println!("Day 10, part 1: {}", ans);
    assert_eq!(23874, ans);
}

fn knot_hash_round<const N: usize>(
    pos: &mut usize,
    skip: usize,
    state: &mut [u32; N],
    input: &[usize],
) {
    for (base_skip, &length) in input.iter().enumerate() {
        twist(*pos, length, state);
        *pos = (*pos + length + base_skip + skip) % N;
    }
}

fn dense_hash(state: &[u32]) -> u32 {
    state.into_iter().fold(0, |a, b| a ^ *b)
}

#[test]
fn test_dense_hash() {
    let input = [65, 27, 9, 1, 4, 3, 40, 50, 91, 7, 6, 0, 2, 5, 68, 22];
    let ans = dense_hash(&input);
    println!("{}", ans);
    assert_eq!(64, ans);
}

#[test]
fn part2() {
    let input = "225,171,131,2,35,5,0,13,1,246,54,97,255,98,254,110".as_bytes();
    let mut extra = vec![17, 31, 73, 47, 23];
    let mut input: Vec<_> = input.iter().map(|byte| *byte as usize).collect();
    input.append(&mut extra);
    let (mut pos, mut skip) = (0, 0);
    let mut state = [0u32; 256];
    for i in 0..256 {
        state[i] = i as u32;
    }
    for _ in 0..64 {
        knot_hash_round(&mut pos, skip, &mut state, &input);
        skip += input.len();
    }
    let ans: String = state
        .chunks(16)
        .map(dense_hash)
        .map(|n| format!("{:02x}", n))
        .collect();
    println!("Day 10, part 2: {}", ans);
    assert_eq!(String::from("e1a65bfb5a5ce396025fab5528c25a87"), ans);
}
