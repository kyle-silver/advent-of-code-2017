use std::{iter::FromIterator, usize};

const INPUT: &str = include_str!("res/19.txt");

#[derive(Debug, Clone)]
enum Direction {
    Up, Down, Left, Right, Halted,
}

#[derive(Debug)]
struct State {
    world: Vec<Vec<char>>,
    x: usize,
    y: usize,
    dir: Direction,
}

impl State {
    fn new(world: Vec<Vec<char>>) -> State {
        let x = world[0].iter().enumerate()
            .filter(|(_, c)| **c == '|')
            .next()
            .map(|(i, _)| i)
            .unwrap();
        State { world, x, y: 0, dir: Direction::Down }
    }

    fn advance(&mut self, dir: Direction) {
        self.dir = dir;
        match self.dir {
            Direction::Up => {self.y -= 1;},
            Direction::Down => {self.y += 1;},
            Direction::Left => {self.x -= 1;},
            Direction::Right => {self.x += 1;},
            Direction::Halted => {},
        }
    }

    fn next_dir(&self) -> Option<Direction> {
        match self.current() {
            '+' => {
                let candidates: [(Option<&char>, Direction); 2] = match self.dir {
                    Direction::Up | Direction::Down => {
                        [(self.left(), Direction::Left), (self.right(), Direction::Right)]
                    },
                    Direction::Left | Direction::Right => {
                        [(self.up(), Direction::Up), (self.down(), Direction::Down)]
                    },
                    Direction::Halted => return None,
                };
                println!("Candidates: {:?}", candidates);
                let next = candidates.iter()
                    .filter(|(c, _)| c.map(|c| *c != ' ').unwrap_or(false))
                    .next()
                    .map(|(_, dir)| dir)
                    .unwrap_or(&Direction::Halted)
                    .clone();
                Some(next)
            },
            ' ' => None,
            _ => Some(self.dir.clone())
        }
    }

    fn current(&self) -> char {
        self.world[self.y][self.x]
    }

    fn up(&self) -> Option<&char> {
        if self.y == 0 {
            return None;
        }
        self.world.get(self.y - 1)?.get(self.x)
    }

    fn down(&self) -> Option<&char> {
        self.world.get(self.y + 1)?.get(self.x)
    }

    fn left(&self) -> Option<&char> {
        if self.x == 0 {
            return None;
        }
        self.world.get(self.y)?.get(self.x - 1)
    }

    fn right(&self) -> Option<&char> {
        self.world.get(self.y)?.get(self.x + 1)
    }
}


#[test]
fn part1() {
    let world: Vec<Vec<char>> = INPUT.lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut stack: Vec<char> = Vec::new();
    let mut state = State::new(world);
    let mut steps = 0;
    while let Some(next_dir) = state.next_dir() {
        println!("cur: {:?}, next: {:?}", state.current(), next_dir);
        let current = state.current();
        if current.is_alphabetic() {
            stack.push(current);
        }
        state.advance(next_dir);
        steps += 1;
    }
    let ans = String::from_iter(stack.iter());
    println!("Day 19, part 1: {}", ans);
    println!("Day 19, part 2: {}", steps);
    // let mut pos = (start, 0);
    // let mut dir = Direction::Down;
    
}