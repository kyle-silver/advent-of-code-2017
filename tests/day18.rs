//! --- Day 18: Duet ---
//!
//! You discover a tablet containing some strange assembly code labeled simply
//! "Duet". Rather than bother the sound card with it, you decide to run the
//! code yourself. Unfortunately, you don't see any documentation, so you're
//! left to figure out what the instructions mean on your own. It seems like the
//! assembly is meant to operate on a set of registers that are each named with
//! a single letter and that can each hold a single integer. You suppose each
//! register should start with a value of 0.
//!
//! There aren't that many instructions, so it shouldn't be hard to figure out
//! what they do. Here's what you determine:
//!
//! * snd X plays a sound with a frequency equal to the value of X.
//! * set X Y sets register X to the value of Y.
//! * add X Y increases register X by the value of Y.
//! * mul X Y sets register X to the result of multiplying the value contained
//!   in register X by the value of Y.
//! * mod X Y sets register X to the remainder of dividing the valŒue contained
//!   in register X by the value of Y (that is, it sets X to the result of X
//!   modulo Y).
//! * rcv X recovers the frequency of the last sound played, but only when the
//!   value of X is not zero. (If it is zero, the command does nothing.)
//! * jgz X Y jumps with an offset of the value of Y, but only if the value of X
//!   is greater than zero. (An offset of 2 skips the next instruction, an
//!   offset of -1 jumps to the previous instruction, and so on.)
//!
//! Many of the instructions can take either a register (a single letter) or a
//! number. The value of a register is the integer it contains; the value of a
//! number is that number.
//!
//! After each jump instruction, the program continues with the instruction to
//! which the jump jumped. After any other instruction, the program continues
//! with the next instruction. Continuing (or jumping) off either end of the
//! program terminates it.
//!
//! For example:
//!
//! ```txt
//! set a 1
//! add a 2
//! mul a a
//! mod a 5
//! snd a
//! set a 0
//! rcv a
//! jgz a -1
//! set a 1
//! jgz a -2
//! ```
//!
//! The first four instructions set a to 1, add 2 to it, square it, and then set
//! it to itself modulo 5, resulting in a value of 4. Then, a sound with
//! frequency 4 (the value of a) is played. After that, a is set to 0, causing
//! the subsequent rcv and jgz instructions to both be skipped (rcv because a is
//! 0, and jgz because a is not greater than 0). Finally, a is set to 1, causing
//! the next jgz instruction to activate, jumping back two instructions to
//! another jump, which jumps again to the rcv, which ultimately triggers the
//! recover operation. At the time the recover operation is executed, the
//! frequency of the last sound played is 4.
//!
//! What is the value of the recovered frequency (the value of the most recently
//! played sound) the first time a rcv instruction is executed with a non-zero
//! value?

use std::{cell::RefCell, collections::VecDeque, rc::Rc};

const INPUT: &str = include_str!("res/18.txt");

#[derive(Debug)]
enum Arg {
    Val(i64),
    Reg(usize),
}

impl Arg {
    fn parse(token: &&str) -> Arg {
        match token.parse() {
            Ok(val) => Arg::Val(val),
            Err(_) => Arg::Reg(
                token
                    .chars()
                    .next()
                    .map(|c| c as usize - 'a' as usize)
                    .unwrap(),
            ),
        }
    }
}

#[derive(Debug)]
enum Op {
    Snd(usize),
    Rcv(usize),
    Set(usize, Arg),
    Add(usize, Arg),
    Mul(usize, Arg),
    Mod(usize, Arg),
    Jgz(usize, Arg),
}

impl Op {
    fn parse(input: &str) -> Op {
        let tokens: Vec<_> = input.split(' ').collect();
        let reg = match Arg::parse(&tokens[1]) {
            Arg::Val(v) => v as usize,
            Arg::Reg(r) => r,
        };
        let val: Option<Arg> = tokens.get(2).map(Arg::parse);
        match tokens[0] {
            "snd" => Op::Snd(reg),
            "rcv" => Op::Rcv(reg),
            "set" => Op::Set(reg, val.unwrap()),
            "add" => Op::Add(reg, val.unwrap()),
            "mul" => Op::Mul(reg, val.unwrap()),
            "mod" => Op::Mod(reg, val.unwrap()),
            "jgz" => Op::Jgz(reg, val.unwrap()),
            _ => panic!("Unidentified token"),
        }
    }
}

#[derive(Debug)]
struct Comp<'a> {
    program: &'a [Op],
    regs: [i64; 16],
    pos: usize,
    waiting: bool,
    snd: Rc<RefCell<VecDeque<i64>>>,
    rcv: Rc<RefCell<VecDeque<i64>>>,
    snd_count: u64,
    id: i64,
}

impl Comp<'_> {
    fn new(
        program: &[Op],
        id: i64,
        snd: Rc<RefCell<VecDeque<i64>>>,
        rcv: Rc<RefCell<VecDeque<i64>>>,
    ) -> Comp {
        let mut comp = Comp {
            program,
            regs: [0; 16],
            pos: 0,
            waiting: false,
            snd,
            rcv,
            snd_count: 0,
            id,
        };
        comp.regs['p' as usize - 'a' as usize] = id;
        comp
    }

    fn get(&self, arg: &Arg) -> i64 {
        match arg {
            Arg::Val(v) => *v,
            Arg::Reg(r) => self.regs[*r],
        }
    }

    fn step(&mut self) {
        if self.waiting && self.rcv.borrow().is_empty() {
            println!("{}: Waiting (snd count {})", self.id, self.snd_count);
            return;
        }
        match self.program[self.pos] {
            Op::Snd(r) => {
                self.snd.borrow_mut().push_back(self.regs[r]);
                self.snd_count += 1;
            }
            Op::Rcv(r) => match self.rcv.borrow_mut().pop_front() {
                Some(val) => {
                    self.regs[r] = val;
                    if self.waiting {
                        println!("{}: Data received! No longer waiting", self.id);
                    }
                    self.waiting = false;
                }
                None => {
                    println!("{}: No data! Waiting", self.id);
                    self.waiting = true;
                    return;
                }
            },
            Op::Set(r, ref a) => {
                self.regs[r] = self.get(a);
            }
            Op::Add(r, ref a) => {
                self.regs[r] += self.get(a);
            }
            Op::Mul(r, ref a) => {
                self.regs[r] *= self.get(a);
            }
            Op::Mod(r, ref a) => {
                self.regs[r] %= self.get(a);
            }
            Op::Jgz(r, ref a) => {
                if self.regs[r] > 0 {
                    self.pos = (self.pos as i64 + self.get(a)) as usize;
                    return;
                }
            }
        }
        self.pos += 1;
    }
}

#[test]
fn part1() {
    let lines = INPUT.lines();
    let program: Vec<_> = lines.map(Op::parse).collect();
    let snd = Rc::new(RefCell::new(VecDeque::new()));
    let rcv = Rc::new(RefCell::new(VecDeque::new()));
    let mut comp = Comp::new(&program, 0, snd, rcv);

    while comp.waiting == false {
        comp.step();
    }

    let i = comp.snd.borrow();
    let ans = i.iter().next().unwrap();

    println!("Day 18, part 1: {}", ans);
    // assert_eq!(8600, *ans);
}

// #[test]
// fn part2() {
//     let lines = INPUT.lines();
//     let program: Vec<_> = lines.map(Op::parse).collect();
//     let q1 = Rc::new(RefCell::new(VecDeque::new()));
//     let q2 = Rc::new(RefCell::new(VecDeque::new()));
//     let mut comp1 = Comp::new(&program, 0, q1.clone(), q2.clone());
//     let mut comp2 = Comp::new(&program, 1, q2.clone(), q1.clone());
//     let mut i: u64 = 0;
//     while !(comp1.waiting && comp2.waiting) {
//         if i % 10_000_000 == 0 {
//             println!("i: {}", i);
//             println!("{:?}, {:?}", q1, q2);
//         }
//         comp1.step();
//         comp2.step();
//         i += 1;
//     }
//     println!("Day 18, part 2: {}", comp1.snd_count);
// }
