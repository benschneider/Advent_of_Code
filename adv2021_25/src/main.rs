// use std::collections::{BTreeMap, HashMap};


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Empty,
    None,
}

type Map = Vec<Vec<Direction>>;



fn main() {
    let input = include_str!("input");

    let input_map = parse_input(input);
    println!("Input map:");
    print_map(&input_map);
   
    let mut new_map = input_map.clone();
    for (i, _x) in (0..10008).enumerate(){
        let old_map = new_map.clone();
        new_map = step_map(&old_map);

        if map_is_eq(&new_map, &old_map) {
            println!("Map: {}", i+1);
            print_map(&new_map);
            break;
        }
    }

    // check if new_map is the same as input_map
    //let a = check_eq_map(&input_map, &input_map);
    //println!("{}", a);

}

fn map_is_eq(m1: &Map, m2: &Map) -> bool {
    for (y, row) in m1.iter().enumerate() {
        for (x, dir) in row.iter().enumerate() {
            if dir != &m2[y][x] {
                return false;
            }
        }
    }
    true
}

fn print_map(map: &Map) {
    for (_y, row) in map.iter().enumerate() {
        for (_x, cell) in row.iter().enumerate() {
            print!("{}", cell);
        }
        println!();
    }
    println!();
}

fn step_map(in_map: &Map) -> Map {
    let mut new_map: Map = in_map.clone();
    // move all east moving pieces first
    move_piece_to_empty(&mut new_map, Direction::Right);
    // then move all south moving pieces
    move_piece_to_empty(&mut new_map, Direction::Down);
    new_map
}


fn move_piece_to_empty(map: &mut Map, dir: Direction) {
    let max_x = map[0].len() as usize;
    let max_y = map.len() as usize;
    let (dx, dy) = dir.get_offset();
    let mut next_x: usize;
    let mut next_y: usize;
    let map_copy = map.clone(); // clone map to avoid mutability issues
    // loop over all cells
    for y in 0..max_y {
        for x in 0..max_x {
            let cell = &map_copy[y][x]; // current cell
            if cell == &dir { // cell as selected direction
                next_x = (x as i32 + dx) as usize;
                next_y = (y as i32 + dy) as usize;
                if next_x >= max_x {
                    next_x = 0;  // wrap around x axis
                } 
                if next_y >= max_y {
                    next_y = 0;  // wrap around y axis
                }
                // Check if next cell is empty, before moving
                let next_cell = &map_copy[next_y][next_x];
                if next_cell == &Direction::Empty { 
                    //dbg!(x, y);
                    //dbg!(next_x, next_y);
                    map[next_y][next_x] = dir.clone();  // move to empty cell
                    map[y][x] = Direction::Empty;   // empty current cell
                    // print_map(&map)
                }
            } // end if cell == dir
        } // end for x loop
    } // end for y loop (end of looping over all cells)
}


fn parse_input(input: &str) -> Map {
    let mut map = Vec::new();
    for (_, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (_, c) in line.chars().enumerate() {
            row.push(c.to_direction());
        }
        map.push(row);
    }
    map
}

/// Implementations and helper functions
// implement subtraction for Direction
impl std::ops::Sub for Direction {
    type Output = Direction;

    fn sub(self, other: Direction) -> Direction {
        let (sx, sy) = self.get_offset();
        let (ox, oy) = other.get_offset();
        let nx = sx - ox;
        let ny = sy - oy;
        match (nx, ny) {
            (0, -1) => Direction::Up,
            (0, 1) => Direction::Down,
            (-1, 0) => Direction::Left,
            (1, 0) => Direction::Right,
            (0, 0) => Direction::Empty,
            _ => Direction::None,
        }
    }
}

impl Direction {
    fn from_char(s: &char) -> Direction {
        match s {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            '.' => Direction::Empty,
            _ => Direction::None,
        }
    }

    fn get_offset(&self) -> (i32, i32) {
        // returns offset dx, dy
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            Direction::Empty => (0, 0),
            Direction::None => (0, 0),
        }
    }



}

// display direction
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">"),
            Direction::Empty => write!(f, "."),
            Direction::None => write!(f, ""),
        }
    }
}

trait CharDirection {
    fn to_direction(&self) -> Direction;
}

impl CharDirection for char {
    fn to_direction(&self) -> Direction {
        Direction::from_char(self)
    }
}
    