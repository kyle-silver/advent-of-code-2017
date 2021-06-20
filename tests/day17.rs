use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Node {
    val: u32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new_unlinked_rcrefcell(val: u32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node { val, next: None }))
    }

    fn update_next(&mut self, node: Rc<RefCell<Node>>) {
        if let Some(n) = &self.next {
            node.borrow_mut().next = Some(n.clone());
        }
        self.next = Some(node.clone());
    }

    fn get_next(&self) -> Option<Rc<RefCell<Node>>> {
        match &self.next {
            Some(n) => Some((*n).clone()),
            None => None,
        }
    }

    fn insert_after(&mut self, val: u32) {
        self.update_next(Node::new_unlinked_rcrefcell(val));
    }
}

#[test]
fn methods_test() {
    let r1 = Node::new_unlinked_rcrefcell(1);
    r1.borrow_mut().update_next(r1.clone());
    r1.borrow_mut().insert_after(2);
    r1.borrow_mut().insert_after(3);
    let two_after = r1.as_ref().borrow().get_next().unwrap().as_ref().borrow().get_next().unwrap().as_ref().borrow().val;
    println!("Two from head: {}", two_after);
}

#[test]
fn day17_part1_cll() {
    let r1 = Node::new_unlinked_rcrefcell(0);
    r1.borrow_mut().update_next(r1.clone());
    let mut ptr = r1.clone();
    for i in 1..=2017 {
        // step forward
        for _ in 0..337 {
            let next = ptr.as_ref().borrow().get_next().unwrap();
            ptr = next;
        }
        // insert the next node after the current pointer
        ptr.borrow_mut().insert_after(i);
        // step forward one more time
        let next = ptr.as_ref().borrow().get_next().unwrap();
        ptr = next;
    }
    let next = ptr.as_ref().borrow().get_next().unwrap();
    ptr = next;
    let ans = ptr.as_ref().borrow().val;
    println!("Day 17, part 1: {}", ans);
    assert_eq!(600, ans);
}

#[test]
fn day17_part1_alt() {
    let mut state = vec![0];
    let mut pos = 0;
    for i in 1..=2017 {
        pos = (pos + 337) % state.len();
        if pos >= state.len() {
            state.push(1);
        } else {
            state.insert(pos, i);
        }
        pos = (pos + 1) % state.len();
    }
    let ans = state[pos];
    println!("Day 17, part 1: {} (alt. impl.)", ans);
}

#[test]
fn part2() {
    let mut len = 1;
    let mut pos = 0;
    let mut next_to_zero = 0;
    for i in 1..=50_000_000 {
        pos = (pos + 337) % len;
        len += 1;
        pos = (pos + 1) % len;
        if pos == 1 {
            next_to_zero = i;
            println!("{} -> next to zero: {} ", i, next_to_zero);
        }
    } 
    println!("Day 17, part 2: {}", next_to_zero);
    assert_eq!(31220910, next_to_zero);
}