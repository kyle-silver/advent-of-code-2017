#[derive(Debug)]
struct Garbage<'a>(&'a [char]);

impl Garbage<'_> {
    /// assumes that the first character is always '<'
    fn parse(input: &[char]) -> Garbage {
        // println!("parsing garbage: {:?}", &input[..5]);
        let mut escaped = false;
        // let mut escaped_duration = 0;
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
        // println!("element input: {:?}", &input[..5]);
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
}

#[test]
fn part1() {
    let raw = include_str!("res/09.txt");
    let input: Vec<_> = raw.chars().collect();
    let group = Group::parse(&input, 1);
    println!("Day 9, part 1: {}", group.score(1));
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