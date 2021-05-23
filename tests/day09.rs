const RAW: &str = include_str!("res/09.txt");

#[derive(Debug)]
struct Garbage<'a>(&'a [char]);

impl Garbage<'_> {
    /// assumes that the first character is always '<'
    fn parse(input: &[char]) -> Garbage {
        let mut escaped = false;
        for (index, c) in input.iter().enumerate() {
            if *c == '!' {
                escaped = !escaped;
                continue;
            } else if escaped {
                escaped = false;
                continue;
            } else if *c == '>' {
                println!("garbage hunk: \"{}\"", input[..=index].iter().collect::<String>());
                return Garbage(&input[..=index]);
            }
        }
        panic!("malformed garbage chunk");
    }
    
    fn size_chars(&self) -> usize {
        self.0.len()
    }

    fn valid_chars(&self) -> u32 {
        let mut count = 0;
        let mut escaped = false;
        // let mut escaped_duration = 0;
        for &c in self.0.iter().skip(1).take(self.0.len() - 2) {
            if c == '!' {
                escaped = !escaped;
                continue;
            } else if escaped {
                escaped = false;
                continue;
            }
            count += 1;
        }
        return count;
    }
}

#[derive(Debug)]
enum Element<'a> {
    Group(Group<'a>),
    Garbage(Garbage<'a>)
}

impl Element<'_> {
    fn size_chars(&self) -> usize {
        match self {
            Element::Group(g) => g.size_chars(),
            Element::Garbage(g) => g.size_chars(),
        }
    }

    fn parse(input: &[char], depth: usize) -> Element {
        match input[0] {
            '{' => Element::Group(Group::parse(input, depth + 1)),
            '<' => Element::Garbage(Garbage::parse(input)),
            other => {
                println!("Panicking on char: '{}' for slice \"{}...\"", other, input.iter().take(10).collect::<String>());
                panic!("Bad input for element parsing")
            }
        }
    }

    fn score(&self, depth: u32) -> u32 {
        match self {
            Element::Group(g) => g.score(depth),
            Element::Garbage(g) => 0
        }
    }

    fn count_garbage_chars(&self) -> u32 {
        match self {
            Element::Group(g) => g.count_garbage_chars(),
            Element::Garbage(g) => g.valid_chars(),
        }
    }
}

#[derive(Debug)]
struct Group<'a>(Vec<Element<'a>>);

impl Group<'_> {
    /// assumes the first character is always '{'
    fn parse(input: &[char], depth: usize) -> Group<'_> {
        println!("group input: \"{}...\"", input.iter().take(10).collect::<String>());
        let mut elements: Vec<Element> = Vec::new();
        let mut i = 1;
        while i < input.len() {
            if input[i] == '}' {
                break;
            }
            if input[i] == ',' {
                i += 1;
            }
            let element = Element::parse(&input[i..], depth + 1);
            i += element.size_chars();
            elements.push(element);
        }
        Group(elements)
    }

    fn size_chars(&self) -> usize {
        let num_commas = if self.0.len() == 0 { 0 } else { self.0.len() - 1 };
        self.0.iter().map(Element::size_chars).sum::<usize>() + num_commas + 2
    }

    fn score(&self, depth: u32) -> u32 {
        depth + self.0.iter().map(|e| e.score(depth + 1)).sum::<u32>()
    }

    fn count_garbage_chars(&self) -> u32 {
        self.0.iter().map(Element::count_garbage_chars).sum()
    }
}

#[test]
fn part1() {
    let input: Vec<_> = RAW.chars().collect();
    let group = Group::parse(&input, 1);
    let ans = group.score(1);
    println!("Day 9, part 1: {}", ans);
    assert_eq!(9662, ans);
}

#[test]
fn test_garbage() {
    let input = "{{<>,{<!!!!!>>}},{}}".chars().collect::<Vec<char>>();
    println!("{:?}", Group::parse(&input, 1));
}

#[test]
fn test_score() {
    let input = "{}".chars().collect::<Vec<char>>();
    assert_eq!(1, Group::parse(&input, 1).score(1));
    let input = "{{{}}}".chars().collect::<Vec<char>>();
    assert_eq!(6, Group::parse(&input, 1).score(1));
    let input = "{{},{}}".chars().collect::<Vec<char>>();
    assert_eq!(5, Group::parse(&input, 1).score(1));
    let input = "{{{},{},{{}}}}".chars().collect::<Vec<char>>();
    assert_eq!(16, Group::parse(&input, 1).score(1));
    let input = "{<a>,<a>,<a>,<a>}".chars().collect::<Vec<char>>();
    assert_eq!(1, Group::parse(&input, 1).score(1));
    let input = "{{<ab>},{<ab>},{<ab>},{<ab>}}".chars().collect::<Vec<char>>();
    assert_eq!(9, Group::parse(&input, 1).score(1));
    let input = "{{<!!>},{<!!>},{<!!>},{<!!>}}".chars().collect::<Vec<char>>();
    assert_eq!(9, Group::parse(&input, 1).score(1));
    let input = "{{<a!>},{<a!>},{<a!>},{<ab>}}".chars().collect::<Vec<char>>();
    assert_eq!(3, Group::parse(&input, 1).score(1));
}

#[test]
fn part2() {
    let input: Vec<_> = RAW.chars().collect();
    let group = Group::parse(&input, 1);
    let ans = group.count_garbage_chars();
    println!("Day 9, part 2: {}", ans);
    assert_eq!(4903, ans);
}