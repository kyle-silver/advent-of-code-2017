// infinite thanks to https://www.redblobgames.com/grids/hexagons/
#[derive(Debug)]
enum HexDirection {
    North,
    Northeast,
    Southeast,
    South,
    Southwest,
    Northwest,
}

impl HexDirection {
    fn parse(token: &str) -> HexDirection {
        use HexDirection::*;
        match token {
            "n" => North,
            "ne" => Northeast,
            "nw" => Northwest,
            "s" => South,
            "se" => Southeast,
            "sw" => Southwest,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Tile(i32, i32, i32);

impl Tile {
    fn step(&mut self, dir: HexDirection) {
        match dir {
            HexDirection::North => {
                self.1 += 1;
                self.2 -= 1;
            }
            HexDirection::Northeast => {
                self.0 += 1;
                self.2 -= 1;
            }
            HexDirection::Southeast => {
                self.0 += 1;
                self.1 -= 1;
            }
            HexDirection::South => {
                self.1 -= 1;
                self.2 += 1;
            }
            HexDirection::Southwest => {
                self.0 -= 1;
                self.2 += 1;
            }
            HexDirection::Northwest => {
                self.0 -= 1;
                self.1 += 1;
            }
        }
    }

    fn distance_from_origin(&self) -> i32 {
        (self.0.abs() + self.1.abs() + self.2.abs()) / 2
    }
}

#[test]
fn part1() {
    let raw = include_str!("res/11.txt");
    let input: Vec<_> = raw.split(',').map(HexDirection::parse).collect();
    let mut pos = Tile(0, 0, 0);
    for dir in input {
        pos.step(dir);
    }
    let ans = pos.distance_from_origin();
    println!("Day 11, part 1: {}", ans);
    assert_eq!(764, ans);
}

#[test]
fn part2() {
    let raw = include_str!("res/11.txt");
    let input: Vec<_> = raw.split(',').map(HexDirection::parse).collect();
    let mut pos = Tile(0, 0, 0);
    let mut max_dist = 0;
    for dir in input {
        pos.step(dir);
        let dist = pos.distance_from_origin();
        if dist > max_dist {
            max_dist = dist;
        }
    }
    println!("Day 11, part 2: {}", max_dist);
    assert_eq!(1532, max_dist);
}
