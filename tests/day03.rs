use std::collections::HashMap;

/*
17  16  15  14  13
18   5   4   3  12
19   6   1   2  11
20   7   8   9  10
21  22  23---> ...

the bottom right corner is always equal to (2n+1)^2, where n is the
number of "rings" out it is.

(3 <= sqrt(12) <= 5) => 12 is in ring ceil(5/2) = 2 ("1" is the only member of ring 0)
(5 <= sqrt(26) <= 7) => 26 is in ring ceil(7/2) = 4

ring n (where n > 1) has side lengths 2n+1

for a given k:
    ring number
        R := ceil(floor(sqrt(k-1))/2)
    ring start
        r := (2(R-1)+1)^2
    ring length
        l := (2R+1)^2 - (2(R-1)+1)^2
    ring side lengths
        s := l / 4
    corners
        c1..c4 := r + (n*s)
    (x, y) coordinates of corners
        c1 := (R,R)
        c2 := (-R,R)
        c3 := (-R,-R)
        c4 := (R,-R)
    side of x
        side(k) := floor((k - r)/4) if k != r
    (x, y) coordinates of k
        if k <= c1
            (x,y) := c1 - (0, c1-k)
        if c1 < k <= c2
            (x,y) := c2 + (c2-k,0)
        if c2 < k <= c3
            (x,y) := c3 + (0, c3-k)
        if c3 < k <= c4
            (x,y) := c4 - (c4-k, 0)
*/

fn ring_number(k: u32) -> u32 {
    if k == 0 {
        return 0;
    }
    let k = (k - 1) as f64;
    let lower_bound = k.sqrt().floor();
    return (lower_bound / 2f64).ceil() as u32;
}

fn ring_start(ring_number: u32) -> u32 {
    ((2 * (ring_number - 1)) + 1).pow(2)
}

fn spiral_coords(k: u32) -> (i32, i32) {
    if k <= 1 {
        return (0, 0);
    }
    let ring_number = ring_number(k);
    let start = ring_start(ring_number);
    let end = ring_start(ring_number + 1);
    let side_length = (end - start) / 4;
    // corners
    let c1 = (start + side_length) as i32;
    let c2 = (start + (2 * side_length)) as i32;
    let c3 = (start + (3 * side_length)) as i32;
    let c4 = end as i32;
    // cases
    let k = k as i32;
    let ring_number = ring_number as i32;
    if k <= c1 {
        (ring_number, ring_number - c1 + k)
    } else if k <= c2 {
        (-ring_number + c2 - k, ring_number)
    } else if k <= c3 {
        (-ring_number, -ring_number + c3 - k)
    } else {
        (ring_number - c4 + k, -ring_number)
    }
}

#[test]
fn part1() {
    let input = 265149;
    let (x, y) = spiral_coords(input);
    let ans = x.abs() + y.abs();
    println!("Day 3, part 1: {}", ans);
    assert_eq!(438, ans);
}

const ADJACENCY_VECTORS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[test]
fn part2() {
    let input = 265149;
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    map.insert((0, 0), 1);
    let mut ans = 0;
    for i in 2.. {
        let next = spiral_coords(i) as (i32, i32);
        let sum: i32 = ADJACENCY_VECTORS
            .iter()
            .map(|(x, y)| (x + next.0, y + next.1))
            .filter_map(|pos| map.get(&pos))
            .sum();
        map.insert(next, sum);
        if sum > input {
            ans = sum;
            break;
        }
    }
    println!("Day 3, part 2: {}", ans);
    assert_eq!(266330, ans)
}
