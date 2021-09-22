const INPUT: &str = include_str!("res/13.txt");

#[derive(Debug, Clone)]
enum Direction {
    Out,
    Back,
}

#[derive(Debug, Clone)]
struct Scanner {
    range: usize,
    position: usize,
    direction: Direction,
}

impl Scanner {
    fn new(range: usize) -> Scanner {
        Scanner {
            range,
            position: 0,
            direction: Direction::Out,
        }
    }

    fn update(&mut self) {
        if matches!(self.direction, Direction::Out) && self.position == self.range - 1 {
            self.direction = Direction::Back;
        } else if matches!(self.direction, Direction::Back) && self.position == 0 {
            self.direction = Direction::Out;
        }
        match self.direction {
            Direction::Out => self.position += 1,
            Direction::Back => self.position -= 1,
        }
    }

    fn alert(&self) -> bool {
        // self.position == 0 && matches!(self.direction, Direction::Out)
        self.position == 0
    }
}

#[derive(Debug)]
struct Firewall {
    // path: HashMap<usize, Scanner>,
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
        let capacity = scanners.iter().map(|(index, _)| *index).max().unwrap_or(0);
        let mut path = vec![None; capacity + 1];
        for (index, scanner) in scanners {
            path[index] = Some(scanner);
        }
        Firewall { path }
    }

    fn update(&mut self) {
        self.path
            .iter_mut()
            .filter_map(|s| s.as_mut())
            .for_each(|s| s.update())
    }

    fn size(&self) -> usize {
        self.path.len()
    }

    fn score(&self, depth: usize) -> u32 {
        if let Some(Some(scanner)) = self.path.get(depth) {
            if scanner.alert() {
                return (depth * scanner.range) as u32;
            }
        }
        return 0;
    }
}

#[test]
fn part1() {
    let mut firewall = Firewall::parse(INPUT.lines());
    let mut severity = 0;
    for position in 0..=firewall.size() {
        severity += firewall.score(position);
        firewall.update();
    }
    println!("Day 13, part 1: {}", severity);
}

#[derive(Debug)]
struct Attempt {
    current_depth: usize,
    score: u32,
}

impl Attempt {
    fn new() -> Attempt {
        Attempt {
            current_depth: 0,
            score: 0,
        }
    }
}

#[test]
fn part2() {
    let mut firewall = Firewall::parse(INPUT.lines());
    let mut attempts: Vec<Attempt> = vec![Attempt::new()];
    let size = firewall.size();
    (0..10).for_each(|_| firewall.update());
    while !attempts
        .iter()
        .any(|a| a.current_depth > firewall.size() && a.score == 0)
    {
        for attempt in attempts.iter_mut().rev().take(size + 1) {
            attempt.score += firewall.score(attempt.current_depth);
            attempt.current_depth += 1;
        }
        attempts.push(Attempt::new());
        firewall.update();
        println!("{}", attempts.len());
    }
    for (index, attempt) in attempts.iter().enumerate() {
        println!("{}: {:?}", index + 10, attempt);
    }
}
