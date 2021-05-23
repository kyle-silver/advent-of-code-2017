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
}

#[test]
fn string_slices() {
    let raw = include_str!("res/09.txt");
    let input: Vec<_> = raw.chars().collect();
    let group = Group::parse(&input, 1);
    println!("{:#?}", group);
}

#[test]
fn test_garbage() {
    let input = "{{<>,{<!!!!!>>}},{}}".chars().collect::<Vec<char>>();
    println!("{:?}", Group::parse(&input, 1));
}