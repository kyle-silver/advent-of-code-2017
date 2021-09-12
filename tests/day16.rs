fn chars_to_usize(input: &[char]) -> usize {
    input.iter().collect::<String>().parse().unwrap()
}

#[derive(Debug)]
enum Action {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Action {
    // fn parse(input: &str) -> Action {
    //     let action = input.chars().next().unwrap();
    //     let args: String = input.chars().skip(1).collect();
    //     match action {
    //         's' => Action::Spin(args.parse().unwrap()),
    //         'x' => {
    //             let (first, second) = args.split_once('/')
    //                 .map(|(f, s)| (f.parse().unwrap(), s.parse().unwrap()))
    //                 .unwrap();
    //             Action::Exchange(first, second)
    //         },
    //         'p'  => {
    //             let (first, second) = args.split_once('/').unwrap();
    //             Action::Partner(first.into(), second.into())
    //         }
    //         _ => panic!(),
    //     }
    // }
    fn parse(input: &[char]) -> Action {
        let action = input[0];
        let args = &input[1..];
        match action {
            's' => Action::Spin(chars_to_usize(args)),
            'x' => {
                let (divider_index, _) = args.iter().enumerate().find(|(_, &x)| x == '/').unwrap();
                Action::Exchange(
                    chars_to_usize(&args[..divider_index]),
                    chars_to_usize(&args[divider_index + 1..]),
                )
            }
            'p' => Action::Partner(args[0], args[2]),
            _ => panic!(),
        }
    }
}

#[test]
fn part1() {
    let raw = include_str!("res/16.txt");
    let steps: Vec<Action> = raw
        .split(",")
        .map(|s| s.chars().collect())
        .map(|chars: Vec<char>| Action::parse(&chars))
        .collect();
    let mut state: Vec<char> = "abcdefghijklmnop".chars().collect();
    println!("{}", steps.len());
    // for action in dance {
    //     match action {
    //         Action::Spin(n) => {
    //             let len = state.len();
    //             state = state.into_iter().cycle().skip(len-n).take(len).collect();
    //         },
    //         Action::Exchange(p1, p2) => {
    //             let tmp = state[p1];
    //             state[p1] = state[p2];
    //             state[p2] = tmp;
    //         },
    //         Action::Partner(c1, c2) => {
    //             let (p1, _) = state.iter().enumerate().find(|(_, &c)| c == c1).unwrap();
    //             let (p2, _) = state.iter().enumerate().find(|(_, &c)| c == c2).unwrap();
    //             let tmp = state[p1];
    //             state[p1] = state[p2];
    //             state[p2] = tmp;
    //         },
    //     };
    // }
    state = dance(state, &steps);
    println!("{}", state.iter().collect::<String>());
}

fn dance(mut state: Vec<char>, steps: &[Action]) -> Vec<char> {
    for action in steps {
        match action {
            Action::Spin(n) => {
                let len = state.len();
                state = state.into_iter().cycle().skip(len - n).take(len).collect();
            }
            Action::Exchange(p1, p2) => {
                let tmp = state[*p1];
                state[*p1] = state[*p2];
                state[*p2] = tmp;
            }
            Action::Partner(c1, c2) => {
                let (p1, _) = state.iter().enumerate().find(|(_, &c)| c == *c1).unwrap();
                let (p2, _) = state.iter().enumerate().find(|(_, &c)| c == *c2).unwrap();
                let tmp = state[p1];
                state[p1] = state[p2];
                state[p2] = tmp;
            }
        };
    }
    state
}

#[test]
fn part2() {
    let raw = include_str!("res/16.txt");
    let steps: Vec<Action> = raw
        .split(",")
        .map(|s| s.chars().collect())
        .map(|chars: Vec<char>| Action::parse(&chars))
        .collect();
    let mut state: Vec<char> = "cknmidebghlajpfo".chars().collect();
    /*
    Running this code revealed the dance has a cycle length of 60.
    The closest multiple of 60 to 1,000,000,000 is 999,960 - meaning
    we only need to run the dance 40 times. (actually only 39 because we're
    starting in the state where it was already run once)
    */
    // let initial_state: Vec<char> = "cknmidebghlajpfo".chars().collect();
    // state = dance(state, &steps);
    // let mut count = 1;
    // while initial_state.ne(&state) {
    //     state = dance(state, &steps);
    //     count += 1;
    // }
    // println!("cycle length: {}", count);
    // println!("{}", state.iter().collect::<String>());
    for _ in 0..39 {
        state = dance(state, &steps);
    }
    println!("Day 16, part 2: {}", state.iter().collect::<String>());
}
