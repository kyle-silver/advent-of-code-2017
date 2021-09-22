use std::collections::HashMap;

const INPUT: &str = include_str!("res/21.txt");

type Cell = bool;

/// Just an absolutely freaky implementation for the fun of it.
/// Why not use bit masks and pre-sorted arrays for hyper efficiency?
/// This implementation hashes the pattern object so we don't need to
/// do binary search at runtime. (although testing has revealed that this
/// implementation is ever so slightly slower)
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pattern<const N: usize>(u32);

impl<const N: usize> Pattern<N> {
    fn parse(token: &str) -> Pattern<N> {
        let mut pattern = 0;
        for (i, c) in token.chars().filter(|c| *c != '/').enumerate() {
            if c == '#' {
                pattern += 1 << i;
            }
        }
        Pattern(pattern)
    }

    fn from_segment(segment: &[&[Cell]]) -> Pattern<N> {
        let mut pattern = Pattern(0);
        for (i, &row) in segment.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if cell {
                    pattern.set(i, j);
                }
            }
        }
        pattern
    }

    fn get(&self, i: usize, j: usize) -> bool {
        let bit_position = (i * N) + j;
        self.0 & (1 << bit_position) != 0
    }

    fn set(&mut self, i: usize, j: usize) {
        let bit_position = (i * N) + j;
        self.0 |= 1 << bit_position
    }

    fn rotated(&self) -> Pattern<N> {
        let mut pattern = Pattern(0);
        for i in 0..N {
            for j in 0..N {
                if self.get(i, j) {
                    pattern.set(N - j - 1, i);
                }
            }
        }
        pattern
    }

    fn mirrored(&self) -> Pattern<N> {
        let mut pattern = Pattern(0);
        for i in 0..N {
            for j in 0..N {
                if self.get(i, j) {
                    pattern.set(i, N - j - 1);
                }
            }
        }
        pattern
    }
}

#[derive(Debug)]
struct Replication<const N: usize, const M: usize>(HashMap<Pattern<N>, Pattern<M>>);

impl<const N: usize, const M: usize> Replication<N, M> {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Replication<N, M> {
        let mut rules = HashMap::new();
        for line in lines {
            let (seed, transform) = line.split_once(" => ").unwrap();
            let mut seed = Pattern::<N>::parse(seed);
            let transform = Pattern::<M>::parse(transform);
            for _ in 0..4 {
                rules.insert(seed, transform);
                rules.insert(seed.mirrored(), transform);
                seed = seed.rotated();
            }
        }
        Replication(rules)
    }

    fn rule(&self, segment: &[&[Cell]]) -> &Pattern<M> {
        self.0.get(&Pattern::from_segment(segment)).unwrap()
    }
}

#[derive(Debug)]
struct Patterns {
    small: Replication<2, 3>,
    big: Replication<3, 4>,
}

impl Patterns {
    fn parse(input: &str) -> Patterns {
        let small_rules = input.lines().filter(|line| line.len() == 20);
        let small = Replication::parse(small_rules);
        let big_rules = input.lines().filter(|line| line.len() == 34);
        let big = Replication::parse(big_rules);
        Patterns { small, big }
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<Cell>>,
}

impl Grid {
    fn grow(&self, patterns: &Patterns) -> Grid {
        if self.grid.len() % 2 == 0 {
            self.replicate(&patterns.small)
        } else {
            self.replicate(&patterns.big)
        }
    }

    fn replicate<const N: usize, const M: usize>(&self, replication: &Replication<N, M>) -> Grid {
        let size = self.grid.len();
        let segments = size / N;
        let mut grid = vec![vec![false; segments * M]; segments * M];
        for i in 0..segments {
            for j in 0..segments {
                let (ri, ci) = (i * N, j * N); // row index, col index
                let segment: Vec<&[Cell]> = self.grid[ri..ri + N]
                    .iter()
                    .map(|s| &s[ci..ci + N])
                    .collect();
                let pattern = replication.rule(&segment);
                for r in 0..M {
                    for c in 0..M {
                        // grid[r + (i * M)][c + (j * M)] = pattern.grid[r][c];
                        grid[r + (i * M)][c + (j * M)] = pattern.get(r, c);
                    }
                }
            }
        }
        Grid { grid }
    }

    fn on(&self) -> u32 {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|c| **c).count() as u32)
            .sum()
    }
}

#[test]
fn bit_shift() {
    let p = Pattern::<3>::parse(".../###/..#");
    println!("{:#b}", p.0);
    println!("{}", p.get(0, 2));
    println!("{}", p.get(1, 2));
    println!("{}", p.get(2, 2));
    let mut p = Pattern::<3>::parse(".../.../...");
    p.set(1, 1);
    p.set(2, 2);
    println!("{:#b}", p.0);
    let p1 = p.rotated();
    println!("{:#b}", p1.0);
}

#[test]
fn part1() {
    let input = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];
    let patterns = Patterns::parse(INPUT);
    let mut grid = Grid { grid: input };
    for _ in 0..5 {
        grid = grid.grow(&patterns);
    }
    println!("Day 21, part 1: {}", grid.on());
}

#[test]
fn part2() {
    let input = vec![
        vec![false, true, false],
        vec![false, false, true],
        vec![true, true, true],
    ];
    let patterns = Patterns::parse(INPUT);
    let mut grid = Grid { grid: input };
    for i in 0..18 {
        grid = grid.grow(&patterns);
        println!("iteration {}: {}", i, grid.on());
    }
    println!("Day 21, part 2: {}", grid.on());
}
