#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use pathfinding::prelude::{astar, astar_bag, dfs, dijkstra, fringe, idastar};
use std::collections::{BTreeMap, HashMap};

type PuzzelMap = BTreeMap<(i32, i32), i32>;

fn main() {
    // let input = include_str!("input_test");
    let input = include_str!("input");
    let goal = include_str!("goal");
    let map_start = process_input(input);
    let map_end = process_input(goal);

    // An further optimization option would be to replace the hashmaps with 2 u64 values.
    // let corridor: u16 = 0b00_0_0_0_0_0_0_0_00;
    // let hall: u16 = 0b0000_0000_0000_0000;
    // let map = [(corridor, hall); 4];

    // let result = idastar(&map_start, successors, get_map_distance, |p| *p == map_end);
    // let result = fringe(&map_start, successors, get_map_distance, |p| *p == map_end);
    // let result = astar_bag_collect(&map_start, successors, get_map_distance, |p| *p == map_end);
    let result = astar(&map_start, successors, get_map_distance, |p| *p == map_end);
    if let Some((path, cost)) = result {
        for map in path {
            print_map(&map);
        }
        println!("Cost: {}", cost);
    } else {
        println!("No path found");
    }

    //let result2 = dfs(map_start, |&p| successors(&p), |&p| p == map_end);
    //dbg!(result2);

    /*
    let result = dijkstra(&map_start, successors, |p| *p == map_end);
    if let Some((path, cost)) = result {
        for map in path {
            print_map(&map);
        }
        println!("Cost: {}", cost);
    } else {
        println!("No path found");
    }
    */
}

fn update_map(map: &PuzzelMap, step: ((i32, i32), (i32, i32))) -> PuzzelMap {
    let mut new_map = map.clone();
    let pos_a_unit = new_map.entry(step.0).or_insert(0);
    let unit = *pos_a_unit;
    *pos_a_unit = 0_i32;
    let pos_b_unit = new_map.entry(step.1).or_insert(0);
    *pos_b_unit = unit;
    new_map
}

fn get_map_distance(current_map: &PuzzelMap) -> i32 {
    let mut distance = 0_i32;
    for (pos, unit) in current_map.iter() {
        let d = get_unit_distance(current_map, unit, pos);
        distance += d;
    }
    // if distance > 46000 {
    //     dbg!(distance);
    //     print_map(current_map);
    // }
    //print_map(current_map);
    //println!();
    //println!("Distance: {}", distance);
    distance
}

fn get_unit_distance(current_map: &PuzzelMap, unit: &i32, (x0, y0): &(i32, i32)) -> i32 {
    let x0 = *x0;
    let y0 = *y0;
    if unit == &0 {
        return 0;
    }

    let x1: i32 = get_unit_hall(*unit);
    // determine if how far hall is filled
    let mut y1: i32 = 0;
    for y in (2..=5).rev() {
        let val = current_map.get(&(x1, y)).unwrap();
        if val != unit {
            y1 = y;
            break;
        }
    }
    if y1 == 0 {
        return 0; // hall is filled
    }

    // units in target hall
    let mut traffic = 0;
    for y in (2..y1 + 1).into_iter() {
        let val = current_map.get(&(x1, y)).unwrap();
        traffic += (y - 1) * val * 2;
    }

    // units in current hall
    if x0 != x1 && y0 != 1 {
        for y in (2..y0 + 1).into_iter() {
            let val = current_map.get(&(x0, y)).unwrap();
            traffic += (y - 1) * val;
        }
    }

    // units in corridor
    if x0 != x1 {
        let s = x0.min(x1);
        let e = x0.max(x1);
        for x in (s + 1..=e).into_iter() {
            let val = current_map.get(&(x, 1)).unwrap();
            traffic += val;
        }
        // add corridor corner positions:
        // traffic += *current_map.get(&(2, 1)).unwrap()/16;
        // traffic += *current_map.get(&(10, 1)).unwrap()/16;
    }

    let dx = (x1 - x0).abs();
    // let dy = (y1 - y0).abs();

    let dy: i32;
    if dx == 0 || y0 == 1 {
        dy = (y1 - y0).abs();
    } else {
        dy = (y1 - 1).abs() + (y0 - 1).abs();
    }

    // let dx = dx * l10(unit)*2;
    // let dy = dy * l10(unit)*2;
    // dbg!(traffic) + dbg!(dx * dy)*unit/2
    dx * unit + dy * unit + traffic /10
}

fn get_unit_hall(x: i32) -> i32 {
    match x {
        0 => 1,
        1 => 3,
        10 => 5,
        100 => 7,
        1000 => 9,
        _ => panic!("Unknown unit"),
    }
}

fn l10(x: &i32) -> i32 {
    match &x {
        0 => 0,
        1 => 1,
        10 => 2,
        100 => 3,
        1000 => 4,
        _ => panic!("Unknown unit"),
    }
}

fn successors(m1: &PuzzelMap) -> Vec<(PuzzelMap, i32)> {
    let move_options = get_unit_move_options(m1);
    // print_map(m1);
    let mut next_mapstates = Vec::new();

    for (pos_a, pos_b_list) in move_options {
        for (pos_b, cost) in pos_b_list {
            let step = (pos_a, pos_b);
            let new_map = update_map(m1, step);
            next_mapstates.push((new_map, cost));
        }
    }
    next_mapstates
}

fn get_unit_move_options(m1_map: &PuzzelMap) -> HashMap<(i32, i32), HashMap<(i32, i32), i32>> {
    // need to find which units can move to where and how much it would cost
    // input is the map with unit positions and types

    let mut unit_moves: HashMap<(i32, i32), HashMap<(i32, i32), i32>> = HashMap::new();
    for ((x, y), unit) in m1_map.iter() {
        if *unit > 0 {
            // we have a unit here at x, y and value v
            let mut moves = HashMap::new();
            check_available(m1_map, *x, *y, &mut moves, 1);
            if !moves.is_empty() {
                // filter forbiden locations
                for (nx, ny) in [(3, 1), (5, 1), (7, 1), (9, 1)].iter() {
                    moves.remove(&(*nx, *ny));
                }

                // if unit is in a room, remove same room moves
                /*
                if *y >= 1 {
                    moves.remove(&(*x, 2));
                    moves.remove(&(*x, 3));
                    moves.remove(&(*x, 4));
                    moves.remove(&(*x, 5));
                }
                */

                let room_pos = get_room_pos_target(m1_map, unit);

                // remove all other positions except for units target room_pos
                for x in [3, 5, 7, 9].iter() {
                    for y in [2, 3, 4, 5].iter() {
                        if *x != room_pos.0 || *y != room_pos.1 {
                            moves.remove(&(*x, *y));
                        }
                    }
                }

                if *y ==1 {
                    // remove all inter-corridor moves
                    for x in (1..=11).into_iter() {
                        moves.remove(&(x, 1));
                    }
                }
            

                moves.iter_mut().for_each(|(_, val)| *val *= *unit);
                unit_moves.insert((*x, *y), moves);
            }
        }
        // lets filter to move only between room and corridor
    }
    unit_moves
}

fn get_room_pos_target(m1_map: &PuzzelMap, unit: &i32) -> (i32, i32) {
    // returns (1, 1) if no room is found
    // only return if the room is pure
    let x = get_unit_hall(*unit);
    for y in (2..=5).rev() {
        let unit_at_position = m1_map.get(&(x, y)).unwrap();
        if unit_at_position != unit && unit_at_position != &0 {
            // not pure
            break;
        }
        if unit_at_position == &0 {
            // found empty space here
            return (x, y);
        }
    }
    (1, 1)
}

fn check_available(
    m1: &PuzzelMap,
    x: i32,
    y: i32,
    available: &mut HashMap<(i32, i32), i32>,
    step: i32,
) {
    // collects all available moves for a unit
    for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let (nx, ny) = (x + dx, y + dy);
        if !available.contains_key(&(nx, ny)) {
            if let Some(v) = m1.get_key_value(&(nx, ny)) {
                if v.1 == &0 {
                    let val = available.entry((nx, ny)).or_insert(step);
                    check_available(m1, nx, ny, available, step + 1);
                }
            }
        }
    }
}

fn process_input(input: &str) -> PuzzelMap {
    println!("{}", input);
    // let mut map: BTreeMap<(i32, i32), i32> = BTreeMap::new();
    let mut map: PuzzelMap = PuzzelMap::new();
    let mut x = 0;
    for (yu, line) in input.lines().enumerate() {
        let y = yu as i32;
        // for line in input.lines() {
        for c in line.chars() {
            if c == '.' {
                map.insert((x, y), 0);
            } else if c == 'A' {
                map.insert((x, y), 1);
            } else if c == 'B' {
                map.insert((x, y), 10);
            } else if c == 'C' {
                map.insert((x, y), 100);
            } else if c == 'D' {
                map.insert((x, y), 1000);
            }
            x += 1;
        }
        x = 0;
    }
    map
}

fn print_map(m: &PuzzelMap) {
    // print the map with all the units on it
    // print!("{}[2J", 27 as char);  // clear screen
    // print!("{esc}[2J{esc}[2;1H", esc = 27 as char); // position cursor
    // println!();
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for (k, _) in m.iter() {
        if k.0 < min_x {
            min_x = k.0;
        }
        if k.0 > max_x {
            max_x = k.0;
        }
        if k.1 < min_y {
            min_y = k.1;
        }
        if k.1 > max_y {
            max_y = k.1;
        }
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x + 1 {
            let mut found = false;
            for (k, v) in m.iter() {
                if k.0 == x && k.1 == y {
                    if v == &0 {
                        if [3, 5, 7, 9].contains(&x) {
                            print!(" ");
                        } else {
                            print!("_");
                        }
                    } else if v == &1 {
                        print!("a");
                    } else if v == &10 {
                        print!("b");
                    } else if v == &100 {
                        print!("c");
                    } else if v == &1000 {
                        print!("d");
                    } else {
                        print!("#");
                    }
                    found = true;
                }
            }
            if !found {
                print!(" ");
            }
        }
        println!();
    }
}

// 11 + 16 = 27
// 27 * 3 = 81
/* Example input:
#############
#...........#
###D#A#C#D###
  #C#A#B#B#
  #########
add between starting rows:
#D#C#B#A#
#D#B#A#C#
such that it becomes:
#############
#...........#
###D#A#C#D###
  #D#C#B#A#
  #D#B#A#C#
  #C#A#B#B#
  #########
*/
/* Goal:
#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########
*/
