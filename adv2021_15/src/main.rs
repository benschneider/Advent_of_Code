// Make live easier to read ;)
#![allow(dead_code)]
#![allow(unused_variables)]   
#![allow(unused_imports)]

use pathfinding::prelude::{dijkstra, astar};
use std::collections::HashMap;

type PuzzelMap = HashMap<(i32, i32), u32>;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn neighbours(&self, map: &PuzzelMap) -> Vec<(Pos, i32)> {
        let directions = vec![Pos(0, -1), Pos(0, 1), Pos(-1, 0), Pos(1, 0)];
        let mut neighbours: Vec<(Pos, i32)> = Vec::new();
        for direction in directions {
            let neighbour = Pos(self.0 + direction.0, self.1 + direction.1);
            if let Some(value) = map.get(&(neighbour.0, neighbour.1)) {
                neighbours.push((neighbour, *value as i32));
            }
        }
        neighbours
    }

    fn distance(&self, other: &Pos) -> i32 {
        let square_dist = (self.0 - other.0).abs().pow(2) + (self.1 - other.1).abs().pow(2);
        (square_dist as f64).sqrt() as i32
    }
}

fn main() {
    let input = include_str!("../input/test_input");
    let input = include_str!("../input/input");
    // let input = include_str!("../input/test2_input");
    let mut map: PuzzelMap = HashMap::new();

    // input are just numbers separated by newlines
    let mut xmax = 0;
    let mut ymax = 0;
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            xmax = x as i32 + 1;
            ymax = y as i32 + 1;
            let key = (x as i32, y as i32);
            let value = c.to_digit(10).unwrap();
            map.insert(key, value);
        }
    }

    let xmul = 50;
    let ymul = 50;
    let new_map = extend_map(map, xmax, ymax, xmul, ymul);

    // lets try the Dijkstra algorithm
    let start = Pos(0, 0);
    let end = Pos(xmax*xmul - 1, ymax*ymul - 1);
    // print_map(&new_map, xmax*5, ymax*5);

    let distances = dijkstra(&start, |p| p.neighbours(&new_map), |p| p == &end);
    let distances = astar(&start, |p| p.neighbours(&new_map), |p| p.distance(&end), |p| p == &end);

    //println!("{:?}", distances);
    println!("{:?}", distances);

}

fn extend_map(map: PuzzelMap, xmax: i32, ymax: i32, xmul: i32, ymul: i32) -> PuzzelMap {
    // this function copies the map multiple times into a new map.
    // it is used to extend the map in all directions.
    let mut new_map: HashMap<(i32, i32), u32> = HashMap::new();
    for i in 0..xmul {
        for j in 0..ymul {
            for x in 0..xmax {
                for y in 0..ymax {
                    let val = map.get(&(x, y)).unwrap();
                    // dbg!(val);
                    let new_val = (*val as i32 - 1 + i + j) % 9 + 1;
                    let xpos = x + i*xmax;
                    // dbg!(xpos, x, i);
                    let ypos = y + j*ymax;
                    new_map.insert((xpos, ypos), new_val as u32);
                }
            }

        }
    }
    new_map
}

fn print_map(map: &PuzzelMap, xmax: i32, ymax: i32) {
    for x in 0..xmax {
        for y in 0..ymax {
            let value = map.get(&(x, y)).unwrap();
            if *value < 10 {
                print!(" ");
            }
            print!(" {}", value);
        }
        println!("");
    }
}
