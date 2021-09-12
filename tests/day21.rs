use std::collections::HashMap;

const INPUT: &str = include_str!("res/21.txt");

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Cell {
    On,
    Off,
}

impl Cell {
    fn parse(c: char) -> Cell {
        match c {
            '.' => Cell::Off,
            _ => Cell::On,
        }
    }

    fn on(&self) -> bool {
        match self {
            Cell::On => true,
            Cell::Off => false,
        }
    }
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Pattern<const N: usize> {
    grid: [[Cell; N]; N],
}

impl<const N: usize> Pattern<N> {
    fn parse(input: &str) -> Pattern<N> {
        let mut grid = [[Cell::Off; N]; N];
        for (i, c) in input.chars().filter(|c| *c != '/').enumerate() {
            grid[i % N][i / N] = Cell::parse(c);
        }
        Pattern { grid }
    }

    fn rotated(&self) -> Pattern<N> {
        let mut grid = [[Cell::Off; N]; N];
        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                grid[N - j - 1][i] = *cell;
            }
        }
        Pattern { grid }
    }

    fn mirrored(&self) -> Pattern<N> {
        let mut grid = [[Cell::Off; N]; N];
        for (i, row) in self.grid.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                grid[i][N - j - 1] = *cell;
            }
        }
        Pattern { grid }
    }
}

#[derive(Debug)]
struct Replication<const N: usize, const M: usize> {
    rules: HashMap<Pattern<N>, Pattern<M>>,
}

impl<const N: usize, const M: usize> Replication<N, M> {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Replication<N, M> {
        let mut rules = HashMap::new();
        for line in lines {
            let (seed, transform) = line.split_once(" => ").unwrap();
            let mut seed = Pattern::<N>::parse(seed);
            let transform = Pattern::<M>::parse(transform);
            for _ in 0..4 {
                rules.insert(seed.clone(), transform.clone());
                rules.insert(seed.mirrored(), transform.clone());
                seed = seed.rotated();
            }
        }
        Replication { rules }
    }

    fn rule(&self, input: &[&[Cell]]) -> &Pattern<M> {
        let mut grid = [[Cell::Off; N]; N];
        for (i, row) in input.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                grid[i][j] = *cell;
            }
        }
        self.rules.get(&Pattern { grid }).unwrap()
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
        let mut grid = vec![vec![Cell::Off; segments * M]; segments * M];
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
                        grid[r + (i * M)][c + (j * M)] = pattern.grid[r][c];
                    }
                }
            }
        }
        Grid { grid }
    }

    fn on(&self) -> u32 {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|c| c.on()).count() as u32)
            .sum()
    }
}

#[test]
fn part1() {
    let input = vec![
        vec![Cell::Off, Cell::On, Cell::Off],
        vec![Cell::Off, Cell::Off, Cell::On],
        vec![Cell::On, Cell::On, Cell::On],
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
        vec![Cell::Off, Cell::On, Cell::Off],
        vec![Cell::Off, Cell::Off, Cell::On],
        vec![Cell::On, Cell::On, Cell::On],
    ];
    let patterns = Patterns::parse(INPUT);
    let mut grid = Grid { grid: input };
    for i in 0..18 {
        grid = grid.grow(&patterns);
        println!("iteration {}: {}", i, grid.on());
    }
    println!("Day 21, part 2: {}", grid.on());
}
