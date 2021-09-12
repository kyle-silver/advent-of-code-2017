use std::cmp::Ordering;

const INPUT: &str = include_str!("res/20.txt");

#[derive(Debug, PartialEq, Eq)]
struct Particle {
    pos: [i64; 3],
    vel: [i64; 3],
    acc: [i64; 3],
}

impl Particle {
    fn parse(line: &str) -> Particle {
        let mut triples = line.split(", ");
        let pos = triples.next().unwrap();
        let vel = triples.next().unwrap();
        let acc = triples.next().unwrap();
        Particle {
            pos: Particle::_parse_triple(pos),
            vel: Particle::_parse_triple(vel),
            acc: Particle::_parse_triple(acc),
        }
    }

    fn _parse_triple(triple: &str) -> [i64; 3] {
        let triple: Vec<_> = triple.chars().collect();
        let triple = &triple[3..triple.len() - 1];
        let mut res = [0, 0, 0];
        let mut start = 0;
        let mut index = 0;
        for (i, &c) in triple.iter().enumerate() {
            if c == ',' {
                res[index] = triple[start..i].iter().collect::<String>().parse().unwrap();
                start = i + 1;
                index += 1;
            }
        }
        res[index] = triple[start..].iter().collect::<String>().parse().unwrap();
        return res;
    }

    fn update(&mut self) {
        for i in 0..2 {
            self.vel[i] += self.acc[i];
            self.pos[i] += self.vel[i];
        }
    }

    fn distance_from_origin(&self) -> i64 {
        self.pos.iter().map(|x| (*x).abs()).sum()
    }
}

impl PartialOrd for &Particle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for &Particle {
    fn cmp(&self, other: &Self) -> Ordering {
        match abs_sum(&self.acc).cmp(&abs_sum(&other.acc)) {
            Ordering::Equal => match abs_sum(&self.vel).cmp(&abs_sum(&other.vel)) {
                Ordering::Equal => abs_sum(&self.pos).cmp(&abs_sum(&other.pos)),
                ordering => ordering,
            },
            ordering => ordering,
        }
    }
}

fn abs_sum(p: &[i64]) -> i64 {
    p.iter().map(|x| x.abs()).sum()
}

#[test]
fn part1() {
    let mut particles: Vec<_> = INPUT.lines().map(Particle::parse).enumerate().collect();
    particles.sort_unstable_by(|(_, p1), (_, p2)| p1.cmp(&p2));
    println!("Day 20, part 1: {:?}", particles.first().unwrap().0);
}

#[test]
fn part2() {
    let mut particles: Vec<_> = INPUT.lines().map(Particle::parse).enumerate().collect();
    let time_since_last_collision = 0;
}
