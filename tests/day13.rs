const INPUT: &str = include_str!("res/13.txt");

#[derive(Debug, Clone)]
struct Scanner {
    range: u32,
}

impl Scanner {
    fn new(range: u32) -> Scanner {
        Scanner { range }
    }

    fn alert(&self, picosecond: u32) -> bool {
        picosecond % ((self.range * 2) - 2) == 0
    }
}

#[derive(Debug)]
struct Firewall {
    path: Vec<Option<Scanner>>,
}

impl Firewall {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Firewall {
        let scanners: Vec<_> = lines
            .map(|line| {
                let (depth, range) = line.split_once(": ").unwrap();
                let depth: usize = depth.parse().unwrap();
                let range = range.parse().unwrap();
                (depth, Scanner::new(range))
            })
            .collect();
        let capacity = scanners.iter().map(|(index, _)| *index).max().unwrap();
        let mut path = vec![None; capacity + 1];
        for (index, scanner) in scanners {
            path[index] = Some(scanner);
        }
        Firewall { path }
    }

    fn score(&self, start: u32) -> u32 {
        let score: u32 = self
            .path
            .iter()
            .enumerate()
            .filter_map(|(i, s)| match s {
                Some(s) => Some((i, s)),
                None => None,
            })
            .map(|(i, s)| match s.alert(start + i as u32) {
                true => i as u32 * s.range,
                false => 0,
            })
            .sum();
        let caught_on_day_1 = self
            .path
            .get(0)
            .unwrap()
            .as_ref()
            .map(|s| match s.alert(start) {
                true => 1,
                false => 0,
            })
            .unwrap_or(0);
        return score + caught_on_day_1;
    }
}

#[test]
fn part1() {
    let firewall = Firewall::parse(INPUT.lines());
    println!("Day 13, part 1: {}", firewall.score(0));
}

#[test]
fn part2() {
    let firewall = Firewall::parse(INPUT.lines());
    let mut picosecond = 10;
    while firewall.score(picosecond) > 0 {
        picosecond += 1;
    }
    println!("Day 13, part 2: {}", picosecond);
}
