#[derive(Debug)]
enum Arg {
    Int(i64),
    Reg(u32),
}

impl Arg {
    fn parse(token: &str) -> Arg {
        let first = token.chars().next().unwrap();
        if first.is_alphabetic() {
            return Arg::Reg(first as u32 - 'a' as u32);
        }
        return Arg::Int(token.parse().unwrap());
    }
}

#[derive(Debug)]
enum Op {
    Snd(usize),
    Set(usize, Arg),
    Add(usize, Arg),
    Mul(usize, Arg),
    Mod(usize, Arg),
    Rcv(usize),
    Jgz(Arg, Arg),
}

impl Op {
    fn parse(token: &str) -> Op {
        use Op::*;
        let tokens: Vec<_> = token.split(' ').collect();
        println!("{:?}", tokens);
        match tokens[0] {
            "snd" => Snd(Op::_reg(tokens[1])),
            "set" => Set(Op::_reg(tokens[1]), Arg::parse(tokens[2])),
            "add" => Add(Op::_reg(tokens[1]), Arg::parse(tokens[2])),
            "mul" => Mul(Op::_reg(tokens[1]), Arg::parse(tokens[2])),
            "mod" => Mod(Op::_reg(tokens[1]), Arg::parse(tokens[2])),
            "rcv" => Rcv(Op::_reg(tokens[1])),
            "jgz" => Jgz(Arg::parse(tokens[1]), Arg::parse(tokens[2])),
            _ => panic!("unrecognized op"),
        }
    }

    fn _reg(token: &str) -> usize {
        token.chars().next().map(|r| r as u32 - 'a' as u32).unwrap() as usize
    }
}

#[derive(Debug, Default)]
struct State {
    mem: [i64; 26],
    freq: i64,
    recovered: i64,
}

impl State {
    fn execute(&mut self, ops: &[Op]) {
        let mut pos = 0;
        while 0 <= pos && pos < ops.len() as i64 {
            println!("{}: {:?}", pos, ops[pos as usize]);
            match &ops[pos as usize] {
                Op::Snd(r) => {
                    self.freq = self.mem[*r];
                    pos += 1;
                }
                Op::Set(r, a) => {
                    self.mem[*r] = self.val(a);
                    pos += 1;
                }
                Op::Add(r, a) => {
                    self.mem[*r] += self.val(a);
                    pos += 1;
                }
                Op::Mul(r, a) => {
                    self.mem[*r] *= self.val(a);
                    pos += 1;
                }
                Op::Mod(r, a) => {
                    self.mem[*r] %= self.val(a);
                    pos += 1;
                    println!("modulo result: {}", self.mem[*r])
                }
                Op::Rcv(r) => {
                    if self.mem[*r] != 0 {
                        self.recovered = self.freq;
                        // break;
                    }
                    pos += 1;
                }
                Op::Jgz(r, a) => {
                    pos += if self.val(r) > 0 { self.val(a) } else { 1 };
                }
            };
            println!("{:?}", self);
        }
    }

    fn val(&self, arg: &Arg) -> i64 {
        match arg {
            Arg::Int(i) => *i,
            Arg::Reg(r) => self.mem[*r as usize],
        }
    }
}

const INPUT: &str = include_str!("res/18.txt");

// #[test]
// fn part1() {
//     let ops: Vec<_> = INPUT.lines().map(Op::parse).collect();
//     let mut state = State::default();
//     state.execute(&ops);
//     println!("{}", state.recovered)
// }
