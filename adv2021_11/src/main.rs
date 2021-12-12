#![allow(unused_imports)]
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;

fn main() {

    // first test with the example
    let test_input = include_str!("../input/input_test");
    part1(test_input);
    part2(test_input);
    println!("Now for the real deal...");
    // now solve for real
    let input = include_str!("../input/input");
    part1(input);
    part2(input); // for part 2 we need to find the moment where all numbers in the map are at
    //the same time reset to 0, i.e. when we have a syncronized flash.
}


fn part1(input: &str){
// each char is a number for a given position in x and y.
// the numbers go from 0 to 9.
// the numbers are in the same order as the input
// the numbers are not separated by any other character
// new lines indicate a different position in y.
// the input is a square, so the x and y are the same size.

// lets map the input to a 2d HashMap
    let mut input_map: HashMap<(i32, i32), u32> = HashMap::new();
    let mut x_size = 0;
    let mut y_size = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
                let val = c.to_digit(10).unwrap();
                x_size = x as i32+1;
                y_size = y as i32+1;
                input_map.insert((x as i32, y as i32), val);
        }
    }


    let mut flashes = 0;
    for _ in 0..100 {
        // increase all numbers by 1
        for y in 0..y_size {
            for x in 0..x_size {
                let val = input_map[&(x,y)];
                input_map.insert((x,y), val+1);
            }
        }

        input_map = do_flashes(input_map); // returns a reduced map, with the new numbers
        // missing numbers represent the number of flashes and need to be added to the total

        let tmp = count_flashes(input_map, x_size, y_size);
        input_map = tmp.0;
        flashes += tmp.1;
        // print_map(&input_map);

    }
    print_map(&input_map); // for checking the result
    println!("The total number of flashes after 100 iterations were: {}", flashes);
}

fn count_flashes(mut input_map: HashMap<(i32, i32), u32>, x_size: i32, y_size: i32) -> (HashMap<(i32, i32), u32>, u32) {
    let mut flashes = 0;
    for y in 0..y_size {
        for x in 0..x_size {
            if let Some(_val) = input_map.get(&(x,y)) {
            } else{
                input_map.insert((x,y), 0);
                flashes += 1;
            }
        }
    }
    (input_map, flashes)
}

fn do_flashes(mut input_map: HashMap<(i32, i32), u32> ) -> HashMap<(i32, i32), u32> {

        let keys_above_9 = find_keys_larger_than_9(input_map.clone());
        
        for (x, y) in keys_above_9 {
                // increase the surrounding numbers by 1
                for (x_off, y_off) in &[(-1, 0), (1, 0), (0, -1), (0, 1), (-1, -1), (-1, 1), (1, -1), (1, 1)] {
                    let x_off = x + x_off;
                    let y_off = y + y_off;
                    if let Some(val) = input_map.clone().get(&(x_off, y_off)){
                            input_map.insert((x_off, y_off), val+1); // increase number by 1
                    }
                } 

                // now we reset the number to 0
                // we do this by removing it from the map and adding it again with value 0
                // this way we don't have to check if the number was already flashed
                input_map.remove(&(x, y));

        }; // end of for_each key larger than 9
        // print_map(&input_map);
        let keys_above_9 = find_keys_larger_than_9(input_map.clone());
        if keys_above_9.len() > 0 {
             // now we check for follow up flashes, with the slightly smaller map
            input_map = do_flashes(input_map);
        }
        input_map
}

fn print_map(input_map: &HashMap<(i32, i32), u32>) {
    let x_size = 10;
    let y_size = 10;
    for y in 0..y_size {
        for x in 0..x_size {
            if let Some(val) = input_map.get(&(x,y)){
                if val < &10 {
                    print!(" ");
                }
                print!("{} ", val);
            } else {
                print!(" X ");
            }
        }
        println!();
    }
    println!()
}

fn find_keys_larger_than_9(map: HashMap<(i32,i32), u32>) -> Vec<(i32, i32)> {
    map.iter()
        .filter_map(|(key, &val)| if val > 9 { Some(*key) } else { None })
        .collect()
}

fn part2(input: &str){
// each char is a number for a given position in x and y.
// the numbers go from 0 to 9.
// the numbers are in the same order as the input
// the numbers are not separated by any other character
// new lines indicate a different position in y.
// the input is a square, so the x and y are the same size.
// lets map the input to a 2d HashMap
    let mut input_map: HashMap<(i32, i32), u32> = HashMap::new();
    let mut x_size = 0;
    let mut y_size = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
                let val = c.to_digit(10).unwrap();
                x_size = x as i32+1;
                y_size = y as i32+1;
                input_map.insert((x as i32, y as i32), val);
        }
    }

    'step_loop: for step in 0..10000 {
        // increase all numbers by 1
        for y in 0..y_size {
            for x in 0..x_size {
                let val = input_map[&(x,y)];
                input_map.insert((x,y), val+1);
            }
        }

        input_map = do_flashes(input_map); // returns a reduced map, with the new numbers
        // missing numbers represent the number of flashes and need to be added to the total
        let tmp = count_flashes(input_map, x_size, y_size);
        input_map = tmp.0;
        if tmp.1 == 100 {
            dbg!(step+1);
            break 'step_loop;
        }
        // print_map(&input_map);

    }
    
    print_map(&input_map); // for checking the result
}
