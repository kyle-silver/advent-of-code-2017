use std::collections::HashMap;

#[test]
fn part1() {
    // you could also look for the only name in the file
    // that only appears once...
    let raw = include_str!("res/07.txt");
    let entries: HashMap<_, _> = raw.lines().map(parse_line).collect();
    let mut reverse_index: HashMap<&str, &str> = HashMap::new();
    for (parent, (_, children)) in entries {
        for child in children {
            reverse_index.insert(child, parent);
        }
    }
    let (_, mut parent) = reverse_index.iter().next().unwrap();
    while let Some(grandparent) = reverse_index.get(parent) {
        parent = grandparent;
    }
    println!("Day 7, part 1: {}", parent)
}

fn parse_line(line: &str) -> (&str, (u32, Vec<&str>)) {
    let halves: Vec<&str> = line.split(" -> ").collect();
    let parent_tokens: Vec<&str> = halves[0].split_ascii_whitespace().collect();
    let name = parent_tokens[0];
    let weight: u32 = parent_tokens[1]
        .chars()
        .skip(1)
        .take(parent_tokens[1].len() - 2)
        .collect::<String>()
        .parse()
        .unwrap();
    let children = if halves.len() == 1 {
        vec![]
    } else {
        halves[1].split(", ").collect()
    };
    return (name, (weight, children));
}

#[test]
fn part2() {
    let raw = include_str!("res/07.txt");
    let entries: HashMap<_, _> = raw.lines().map(parse_line).collect();
    let mut reverse_index: HashMap<&str, &str> = HashMap::new();
    for (parent, (_, children)) in &entries {
        for child in children {
            reverse_index.insert(child, parent);
        }
    }
    let (_, mut parent) = reverse_index.iter().next().unwrap();
    while let Some(grandparent) = reverse_index.get(parent) {
        parent = grandparent;
    }
    let ans = is_balanced(parent, &entries).unwrap_err();
    println!("Day 7, part 2: {}", ans);
    assert_eq!(299, ans);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct BalancedNode {
    weight: u32,
    children: u32,
}

impl BalancedNode {
    fn new(weight: u32, children: u32) -> BalancedNode {
        BalancedNode { weight, children }
    }

    fn total_weight(&self) -> u32 {
        self.weight + self.children
    }
}

fn is_balanced(key: &str, tree: &HashMap<&str, (u32, Vec<&str>)>) -> Result<BalancedNode, u32> {
    let (weight, children) = tree.get(key).unwrap();
    // base case: a leaf node is always balanced
    if children.is_empty() {
        return Ok(BalancedNode::new(*weight, 0));
    }
    // separate out children which are already unbalanced
    let (children, unbalanced): (Vec<_>, Vec<_>) = children
        .iter()
        .map(|&child| is_balanced(child, tree))
        .partition(Result::is_ok);
    // if a child node is unbalanced, then the parent is also unbalanced
    if let Some(unbalanced_node_weight) = unbalanced.into_iter().next() {
        return unbalanced_node_weight;
    }
    // check if the children are balanced (all have the same weight)
    let children: Vec<_> = children.into_iter().map(Result::unwrap).collect();
    let mut weights: HashMap<u32, u32> = HashMap::new();
    for child in &children {
        *weights.entry(child.total_weight()).or_default() += 1;
    }
    // if all children weigh the same, the parent is balanced
    if weights.keys().len() <= 1 {
        let children_weight: u32 = weights
            .iter()
            .map(|(weight, occurrences)| weight * occurrences)
            .sum();
        return Ok(BalancedNode::new(*weight, children_weight));
    }
    // find the "heaviest" and "lightest" to calculate the re-balance
    let heaviest = weights.keys().max().unwrap();
    let heaviest = children
        .iter()
        .filter(|c| c.total_weight() == *heaviest)
        .next()
        .unwrap();
    let lightest = weights.keys().min().unwrap();
    let lightest = children
        .iter()
        .filter(|c| c.total_weight() == *lightest)
        .next()
        .unwrap();
    let weight_delta = heaviest.total_weight() - lightest.total_weight();
    let expected_weight = heaviest.weight - weight_delta;
    Err(expected_weight)
}
