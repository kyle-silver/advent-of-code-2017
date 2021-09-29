use std::convert::TryInto;

fn twist<const N: usize>(pos: usize, length: usize, state: &mut [u8; N]) {
    let to_swap: Vec<u8> = state
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

fn knot_hash_round<const N: usize>(
    pos: &mut usize,
    skip: usize,
    state: &mut [u8; N],
    input: &[usize],
) {
    for (base_skip, &length) in input.iter().enumerate() {
        twist(*pos, length, state);
        *pos = (*pos + length + base_skip + skip) % N;
    }
}

fn dense_hash(state: &[u8]) -> u8 {
    state.into_iter().fold(0, |a, b| a ^ *b)
}

fn knot_hash(input: &[usize]) -> u128 {
    let (mut pos, mut skip) = (0, 0);
    let mut state = [0; 256];
    for i in 0..256 {
        state[i] = i as u8;
    }
    for _ in 0..64 {
        knot_hash_round(&mut pos, skip, &mut state, input);
        skip += input.len();
    }
    let bytes: Vec<_> = state.chunks(16).map(dense_hash).rev().collect();
    let bytes: [u8; 16] = bytes.try_into().unwrap();
    u128::from_le_bytes(bytes)
}

#[test]
fn test() {
    let input = "225,171,131,2,35,5,0,13,1,246,54,97,255,98,254,110".as_bytes();
    let mut extra = vec![17, 31, 73, 47, 23];
    let mut input: Vec<_> = input.iter().map(|byte| *byte as usize).collect();
    input.append(&mut extra);
    println!("{:x}", knot_hash(&input))
}

#[test]
fn part1() {
    // spell-checker: disable
    let seed = "stpzcrnm";
    let extra = vec![17, 31, 73, 47, 23];
    let hashes: u32 = (0..128)
        .map(|i| format!("{}-{}", seed, i))
        .map(|s| s.as_str().bytes().map(|b| b as usize).collect())
        .map(|mut input: Vec<_>| {
            input.append(&mut extra.clone());
            input
        })
        .map(|input| knot_hash(&input))
        .map(|h| h.count_ones())
        .sum();
    println!("Day 14, part 1: {}", hashes);
    assert_eq!(8250, hashes);
}

#[derive(Debug, Clone, Copy)]
enum Point {
    Empty,
    Unset,
    Region(u32),
}

fn flood_fill<const N: usize>(grid: &mut [[Point; N]], region: u32, i: usize, j: usize) {
    if let Point::Unset = grid[i][j] {
        grid[i][j] = Point::Region(region);
    }
    for (di, dj) in [(-1, 0), (0, -1), (1, 0), (0, 1)] {
        let r = (i as isize + di) as usize;
        let c = (j as isize + dj) as usize;
        if let Some(Point::Unset) = grid.get(r).and_then(|g| g.get(c)) {
            flood_fill(grid, region, r, c)
        }
    }
}

#[test]
fn part2() {
    // spell-checker: disable
    let seed = "stpzcrnm";
    let extra = vec![17, 31, 73, 47, 23];
    // create data
    let data: Vec<u128> = (0..128)
        .map(|i| format!("{}-{}", seed, i))
        .map(|s| s.as_str().bytes().map(|b| b as usize).collect())
        .map(|mut input: Vec<_>| {
            input.append(&mut extra.clone());
            input
        })
        .map(|input| knot_hash(&input))
        .collect();
    // populate grid
    let mut grid = [[Point::Empty; 128]; 128];
    for (i, d) in data.iter().enumerate() {
        for j in 0..128 {
            if d & (1 << j) != 0 {
                grid[i][j] = Point::Unset;
            }
        }
    }
    let mut region = 0;
    // define regions
    for i in 0..128 {
        for j in 0..128 {
            if let Point::Unset = grid[i][j] {
                flood_fill(&mut grid, region, i, j);
                region += 1;
            }
        }
    }
    println!("Day 14, part 2: {}", region);
    assert_eq!(1113, region)
}
