use std::collections::HashMap;
use derive_new::new;

#[derive(Debug)]
enum Op {
    Lt,
    Lte,
    Gt,
    Gte,
    Eq,
    Neq
}

impl Op {
    fn eval<'a>(&self, a: i64, b: i64) -> bool {
        match self {
            Op::Lt => a < b,
            Op::Lte => a <= b,
            Op::Gt => a > b,
            Op::Gte => a >= b,
            Op::Eq => a == b,
            Op::Neq => a != b,
        }
    }

    fn parse(op: &str) -> Op {
        match op {
            ">" => Op::Gt,
            ">=" => Op::Gte,
            "<" => Op::Lt,
            "<=" => Op::Lte,
            "==" => Op::Eq,
            "!=" => Op::Neq,
            _ => panic!(),
        }
    }
}

#[derive(Debug, new)]
enum Arg<'a> {
    Num(i64),
    Reg(&'a str)
}

impl Arg<'_> {
    fn resolve(&self, regs: &Regs) -> i64 {
        match self {
            Arg::Num(n) => *n,
            Arg::Reg(reg) => regs.get(reg),
        }
    }

    fn parse(arg: &str) -> Arg {
        match arg.parse::<i64>() {
            Ok(n) => Arg::Num(n),
            Err(_) => Arg::Reg(arg),
        }
    }
}

#[derive(Debug, new)]
struct Cond<'a> {
    op: Op,
    a: Arg<'a>,
    b: Arg<'a>,
}

impl<'a> Cond<'a> {
    fn eval(&self, regs: &Regs) -> bool {
        self.op.eval(self.a.resolve(regs), self.b.resolve(regs))
    }

    fn parse(a: &'a str, op: &'a str, b: &'a str) -> Cond<'a> {
        Cond::new(Op::parse(op), Arg::parse(a), Arg::parse(b))
    }
}

#[derive(Debug)]
enum Cmd {
    Inc,
    Dec,
}

impl Cmd {
    fn parse(input: &str) -> Cmd {
        match input {
            "inc" => Cmd::Inc,
            "dec" => Cmd::Dec,
            _ => panic!()
        }
    }
}

#[derive(Debug, new)]
struct Instr<'a> {
    reg: &'a str,
    cmd: Cmd,
    val: i64,
    cond: Cond<'a>,
}

impl Instr<'_> {
    fn to_add(&self) -> i64 {
        match self.cmd {
            Cmd::Inc => self.val,
            Cmd::Dec => -self.val,
        }
    }

    fn parse(input: &str) -> Instr<'_> {
        let tokens: Vec<_> = input.split_ascii_whitespace().collect();
        let reg = tokens[0];
        let cmd = Cmd::parse(tokens[1]);
        let val = tokens[2].parse().unwrap();
        let cond = Cond::parse(tokens[4], tokens[5], tokens[6]);
        Instr::new(reg, cmd, val, cond)
    }
}

#[derive(Debug)]
struct Regs<'a> {
    regs: HashMap<&'a str, i64>,
}

impl<'a> Regs<'a> {
    fn new() -> Regs<'a> {
        Regs { regs: HashMap::new() }
    }

    fn get(&self, reg: &str) -> i64 {
        self.regs.get(reg).cloned().unwrap_or_default()
    }

    fn exec(&mut self, instr: &Instr<'a>) {
        if instr.cond.eval(self) {
            *self.regs.entry(instr.reg).or_insert(0) += instr.to_add();
        }
    }
}

#[test]
fn part1() {
    let raw = include_str!("res/08.txt").lines();
    let program: Vec<Instr> = raw.map(Instr::parse).collect();
    let mut regs = Regs::new();
    for instr in &program {
        regs.exec(instr);
    }
    let ans = regs.regs.values().max().unwrap();
    println!("Day 8, part 1: {}", ans);
    assert_eq!(4066, *ans);
}

#[test]
fn part2() {
    let raw = include_str!("res/08.txt").lines();
    let program: Vec<Instr> = raw.map(Instr::parse).collect();
    let mut regs = Regs::new();
    let mut abs_max = 0;
    for instr in &program {
        regs.exec(instr);
        let cur_max = *regs.regs.values().max().unwrap_or(&0);
        if cur_max > abs_max {
            abs_max = cur_max;
        }
    }
    println!("Day 8, part 2: {}", abs_max);
    assert_eq!(4829, abs_max);
}