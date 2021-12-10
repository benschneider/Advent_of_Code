/* --- Day 9: Smoke Basin ---

These caves seem to be lava tubes. Parts are even still volcanically active; small hydrothermal vents release smoke into the caves that slowly settles like rain.

If you can model how the smoke flows through the caves, you might be able to avoid it and be that much safer. The submarine generates a heightmap of the floor of the nearby caves for you (your puzzle input).

Smoke flows to the lowest point of the area it's in. For example, consider the following heightmap:

2199943210
3987894921
9856789892
8767896789
9899965678
Each number corresponds to the height of a particular location, where 9 is the highest and 0 is the lowest a location can be.

Your first goal is to find the low points - the locations that are lower than any of its adjacent locations. Most locations have four adjacent locations (up, down, left, and right); locations on the edge or corner of the map have three or two adjacent locations, respectively. (Diagonal locations do not count as adjacent.)

In the above example, there are four low points, all highlighted: two are in the first row (a 1 and a 0), one is in the third row (a 5), and one is in the bottom row (also a 5). All other locations on the heightmap have some lower adjacent location, and so are not low points.

The risk level of a low point is 1 plus its height. In the above example, the risk levels of the low points are 2, 1, 6, and 6. The sum of the risk levels of all low points in the heightmap is therefore 15.

Find all of the low points on your heightmap. What is the sum of the risk levels of all low points on your heightmap?

Your puzzle answer was 548.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---

Next, you need to find the largest basins so you know what areas are most important to avoid.

A basin is all locations that eventually flow downward to a single low point. Therefore, every low point has a basin, although some basins are very small. Locations of height 9 do not count as being in any basin, and all other locations will always be part of exactly one basin.

The size of a basin is the number of locations within the basin, including the low point. The example above has four basins.

The top-left basin, size 3:

2199943210
3987894921
9856789892
8767896789
9899965678
The top-right basin, size 9:

2199943210
3987894921
9856789892
8767896789
9899965678
The middle basin, size 14:

2199943210
3987894921
9856789892
8767896789
9899965678
The bottom-right basin, size 9:

2199943210
3987894921
9856789892
8767896789
9899965678
Find the three largest basins and multiply their sizes together. In the above example, this is 9 * 14 * 9 = 1134.

What do you get if you multiply together the sizes of the three largest basins?
*/

#[allow(unused_imports)]
use itertools::Itertools;
use std::collections::HashMap;

// learning & inspiration source: https://github.com/emilyskidsister/aoc/blob/main/p2021_09/src/lib.rs
// splitup in parts is handy
// Note: it seems Rust's HashMaps are unavoidably usable ...
//
//
// Thanks Copilot for the help with the solution! :D

fn main() {
    let input_test = include_str!("input_test");
    let (map_t, maxt_p) = string_to_map(input_test);
    assert_eq!(15, part1(&map_t, &maxt_p));

    let input = include_str!("input");
    let (map, max_p) = string_to_map(input);
    assert_eq!(548, part1(&map, &max_p));
    
    assert_eq!(1134, part2(&map_t, &maxt_p));
    let score_part2 = part2(&map, &max_p);
    dbg!(score_part2);

}

fn part1(map: &HashMap<(i64, i64), i64>, max_p: &(i64, i64)) -> i64 {
    let vally_vec = get_valleys(map, max_p);
    let score: i64 = vally_vec.into_iter().map(|(x, y)| map[&(x, y)] + 1).sum();
    dbg!(score)
}

fn part2(map: &HashMap<(i64, i64), i64>, max_p: &(i64, i64)) -> i64 {
    let vally_vec = get_valleys(map, max_p);
    let mut basin_vec: Vec<i64> = vec![];

    for (x, y) in vally_vec.into_iter() {
        let mut basin_map: HashMap<(i64, i64), i64> = HashMap::new();
        basin_map = check_immediate_neighbors(map, (x,y), basin_map);
        let basin_size: i64 = basin_map.len() as i64;
        basin_vec.push(basin_size);
    }
    basin_vec.sort_unstable();
    let len = basin_vec.len() - 1;
    basin_vec[len] * basin_vec[len - 1] * basin_vec[len - 2]
}

fn check_immediate_neighbors(map: &HashMap<(i64, i64), i64>, pos: (i64, i64), basin_map: HashMap<(i64, i64), i64>) -> HashMap<(i64, i64), i64> {
    // check immediate neighbors,
    // if they are smaller than 9,
    // check them next if they have not been checked
    // after check add point to the basin map
    // if point is not in basin map, add it to the basin map
    // until there are no more smaller points
    // then return the basin size
    // note: this is a recursive function
    // in simple code:
    // point(x,y) -> add to basin map -> check neighbor points -> if smaller than 9 && if point is not in basin map -> add to basin map
    // -> redo check neighbor points -> if smaller than 9 && if point is not in basin map -> add to
    // basin map -> until there are no more smaller points -> return basin size
    let mut basin_map = basin_map;
    let peak: i64 = 9;
    for (dx, dy) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)].into_iter() {
        let (x, y) = (pos.0 + dx, pos.1 + dy);
        if let Some(height) = map.get(&(x, y)) {
            if *height < peak && !basin_map.contains_key(&(x, y)) {
                basin_map.insert((x, y), *height);
                basin_map = check_immediate_neighbors(map, (x, y), basin_map);
            }
        }
    }
    basin_map
}


// Dear Copilot, let's become good friends! :)

fn get_valleys(map: &HashMap<(i64, i64), i64>, max_p: &(i64, i64)) -> Vec<(i64, i64)> {
    // returns a vec with all vally coordinates
    let x_max = max_p.0;
    let y_max = max_p.1;
    let mut val_vec: Vec<(i64, i64)> = vec![];
    for y in 0..=y_max {
        for x in 0..=x_max {
            if is_valley(map, (x, y)) {
                val_vec.push((x, y));
            }
        }
    }
    val_vec
}

fn string_to_map(input: &str) -> (HashMap<(i64, i64), i64>, (i64, i64)) {
    let mut map = HashMap::new();
    let mut x_max = 0;
    let mut y_max = 0;
    for (y, f) in input.lines().enumerate() {
        for (x, c) in f.chars().enumerate() {
            map.insert((x as i64, y as i64), c.to_digit(10).unwrap() as i64);
            x_max = x as i64;
            y_max = y as i64;
        }
    }
    (map, (x_max, y_max))
}

fn is_valley(map: &HashMap<(i64, i64), i64>, pos: (i64, i64)) -> bool {
    let (x, y) = pos;
    let p = map[&(x, y)];
    let mut is_valley = true;
    for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if let Some(other) = map.get(&(x + dx, y + dy)) {
            if other <= &p {
                is_valley = false;
                break;
            }
        }
    }
    is_valley
}

/*
let vecmap: Vec<Vec<i64>> = input
    .lines()
    .map(|line| {
        line.chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect::<Vec<i64>>()
    })
    .collect();
plot_map(&vecmap);
let vecmap_dx: Vec<Vec<i64>> = vecmap.iter().map(|line| line.fwddiff()).collect();
plot_map(&vecmap_dx); // problem is > these are peaks + dips
*/

pub trait VecDiffs {
    fn fwd_diff(&self) -> Self;
    fn diff(&self) -> Self;
}

impl VecDiffs for Vec<i64> {
    // fn fwd_diff(input_numbers: Vec<i64>) -> Vec<i64> {
    fn fwd_diff(self: &Vec<i64>) -> Vec<i64> {
        let input_numbers = self;
        let mut s: i64 = input_numbers[0]; // start with first entry
        let mut ret_vec: Vec<i64> = Vec::new();
        for &e in input_numbers {
            ret_vec.push(e - s);
            s = e;
        }
        ret_vec
    }

    fn diff(self: &Vec<i64>) -> Vec<i64> {
        // returns the derivative for each point exactly
        let input_numbers = self;
        let mut ret_vec: Vec<i64> = vec![];

        let mut s1: i64 = input_numbers[0]; // start with first entry
        let mut s2: i64 = input_numbers[1]; // start with first entry
        let idx_max = input_numbers.len() - 1;
        let s2_last = *input_numbers.last().unwrap();
        for (idx, &e) in input_numbers.iter().enumerate() {
            let le_slope = s1 - e;
            let re_slope = s2 - e;
            let avg_slope = (le_slope + re_slope) / 2;
            ret_vec.push(avg_slope);
            s1 = e;
            s2 = if idx < idx_max - 1 {
                input_numbers[idx + 2]
            } else {
                s2_last
            };
        }
        ret_vec
    }
}

#[allow(dead_code)]
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[allow(dead_code)]
fn plot_map(vec_map: &[Vec<i64>]) {
    let m = vec_map.len();
    let n = vec_map[0].len();
    for y in 0..m {
        for item in vec_map.iter().take(n) {
            let num = item[y];
            if num < 0 {
                print! {"{}", item[y]};
            } else {
                print! {" {}", item[y]};
            }
        }
        println! {};
    }
    println! {};
}
