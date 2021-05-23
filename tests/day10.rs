fn twist<const N: usize>(pos: usize, length: usize, input: &mut [u32; N]) {
    let mut to_swap: Vec<u32> = input.iter().cycle().skip(pos).take(length).cloned().collect();
    to_swap.reverse();
    for (i, val) in to_swap.iter().enumerate() {
        input[(pos + i) % N] = *val;
    }
}

#[test]
fn part1() {
    let input: Vec<usize> = vec![225,171,131,2,35,5,0,13,1,246,54,97,255,98,254,110];
    let mut state = [0u32; 256];
    for i in 0..255 {
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