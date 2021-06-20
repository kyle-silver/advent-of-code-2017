use std::collections::{HashMap, HashSet};

const RAW: &str = include_str!("res/13.txt");

#[derive(Debug)]
enum ScanDirection {
    Out,
    Back,
}


#[derive(Debug)]
struct Scanner {
    range: u32,
    current_position: u32,
    scan_direction: ScanDirection,
}

impl Scanner {
    fn new(range: u32) -> Scanner {
        Scanner { range, current_position: 0, scan_direction: ScanDirection::Out }
    }

    fn step(&mut self) {
        if self.current_position == 0 {
            self.current_position += 1;
            self.scan_direction = ScanDirection::Out;
        } else if self.current_position == self.range - 1 {
            self.current_position -= 1;
            self.scan_direction = ScanDirection::Back;
        } else {
            match self.scan_direction {
                ScanDirection::Out => self.current_position += 1,
                ScanDirection::Back => self.current_position -= 1,
            };
        }
    }

    fn collision(&self) -> bool {
        self.current_position == 0
    }
}

#[derive(Debug)]
struct Firewall(HashMap<u32, Scanner>);

impl Firewall {
    fn parse(input: &str) -> Firewall {
        let state = input.lines().map(|line| {
            let (pos, depth) = line.split_once(": ").unwrap();
            (pos.parse().unwrap(), Scanner::new(depth.parse().unwrap()))
        }).collect();
        Firewall(state)
    }

    fn step(&mut self) {
        self.0.iter_mut().for_each(|(_, v)| v.step());
    }

    fn collision(&self, pos: u32) -> bool {
        self.0.get(&pos).map(|s| s.collision()).unwrap_or(false)
    }

    fn passed(&self, state: &HashSet<u32>) -> bool {
        state.iter().max().unwrap_or(&0) > self.0.keys().max().unwrap()
    }
}

#[test]
fn part1() {
}