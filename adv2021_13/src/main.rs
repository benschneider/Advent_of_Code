#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;

type Instructions = Vec<(char, i64)>;
type InputSet = Vec<(i64, i64)>;
type FoldPositions = HashMap<(i64, i64), i64>;

fn main() {
    let input_test = include_str!("../input/input_test");
    part1_and_2(input_test);
    
    let input = include_str!("../input/input");
    part1_and_2(input);
}


fn part1_and_2(input_test: &str) {
    let (first_part, second_part) = split_input(input_test);
    let mut input_set: InputSet = process_upper_part(first_part);
    let instruction_vec = process_lower_part(second_part);
    // let's print input_set in a nice way
    input_set.sort_by(|a, b| a.0.cmp(&b.0));
    //nice_print_i64_vec(&input_set);
    // nice_print_char_vec(&instruction_vec);

    // lets follow the instructions
    // first instruction says: fold along y=7
    // this means numbers larger than 7 are moved towards 0 (mirrored at y=7)
    // numbers smaller than 7 stay in place


    // lets just "fold along y=7"
    

    let mut x_new: i64;
    let mut y_new: i64;
    let mut fold_positions: FoldPositions = HashMap::new();

    for (x, y) in input_set.iter() {
            fold_positions.insert((*x, *y), 1);
    }

    for (fold_ax, fold_val) in instruction_vec.iter() {
    
    let mut new_fold_positions : FoldPositions = HashMap::new();
    for (x,y) in fold_positions.keys() {
        x_new = *x;
        y_new = *y;
            let fold_val = *fold_val;

            if *fold_ax == 'x' {
                let pos = fold_along_y(y_new, x_new, fold_val);
                x_new = pos.1;
                y_new = pos.0;
            } else {
                let pos = fold_along_y(x_new, y_new, fold_val);
                x_new = pos.0;
                y_new = pos.1;
            }
        let val = new_fold_positions.entry((x_new, y_new)).or_insert(0);
        *val += 1;
        // break; // we only need to fold once for part 1
        }
    fold_positions = new_fold_positions;
    }

    // let's print fold_positions in a nice way
    // let mut tt: Vec<(i64, i64)> = fold_positions.keys().map(|(x,y)| (*x, *y)).collect();
    // tt.sort();
    // nice_print_i64_vec(&tt);
    println!("len {}", fold_positions.len());
    println!("{}", fold_positions.values().sum::<i64>());

    // need to plot the x, y positions 
    // we can mark the positions with a '#' and and all other positions with a '.'
    // we can then use the plot_points function to plot the points
    
    plot_points(&fold_positions); // part 2
}

fn plot_points(fold_positions: &FoldPositions) {
    let x_min: i64 = fold_positions.keys().map(|(x,_)| *x).min().unwrap();
    let x_max: i64 = fold_positions.keys().map(|(x,_)| *x).max().unwrap();
    let y_min: i64 = fold_positions.keys().map(|(_,y)| *y).min().unwrap();
    let y_max: i64 = fold_positions.keys().map(|(_,y)| *y).max().unwrap();

    let mut plot: Vec<Vec<char>> = vec![vec!['.'; (x_max - x_min + 1) as usize]; (y_max - y_min + 1) as usize];
    for (x,y) in fold_positions.keys() {
        plot[(y - y_min) as usize][(x - x_min) as usize] = '#';
    }
    for row in plot.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        println!("");
    }
}


fn fold_along_y(x: i64, y: i64, fold_val: i64) -> (i64, i64) {
    if y > fold_val {
        (x, 2*fold_val - y)
    } else {
        (x, y)
    }
}

fn nice_print_char_vec(vec: &Instructions) {
    for x in vec {
            println!("{} {}", x.0, x.1);
        }
}

fn nice_print_i64_vec(vec: &InputSet) {
    for x in vec {
            println!("{} {}", x.0, x.1);
        }
}


fn part2(_input_test: &str) {
}

fn split_input(input: &str) -> (&str, &str) {
    // split string input into two parts
    // First part are couple of numbers until an empty line is found
    // After that, the second part is the instructions
    // The instructions of a string and a number separated by an equal sign
    let mut split_input = input.split("\n\n");
    let first_part = split_input.next().unwrap();
    let second_part = split_input.next().unwrap();
    (first_part, second_part)
}

fn process_upper_part(input: &str) -> InputSet {
    // First part is a list of couples of numbers,
    // The numbers are separated by a comma
    // Each couple is separated by a new line
    let mut input_set: InputSet = vec![];
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let mut split_line = line.split(",");
        let first_number = split_line.next().unwrap().parse::<i64>().unwrap();
        let second_number = split_line.next().unwrap().parse::<i64>().unwrap();
        input_set.push((first_number, second_number));
    }
    input_set
}

fn process_lower_part(input: &str) -> Instructions {
    // Second part is a list of instructions
    // Each instruction is separated by a new line
    // Each instruction is a string and a number separated by an equal sign
    // an example instruction looks like this:
    // "fold along x=5" here the last part: "x=5" is the instruction we want to use.
    // The instruction is in the first part of the string, the number is the second part
    let mut instructions: Instructions = vec![];
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        let mut split_line = line.split("=");
        let instruction_str = split_line.next().unwrap();
        // now we only want to keep the last part of the string (the char x or y).
        let instruction = instruction_str.chars().last().unwrap();
        assert!(instruction == 'x' || instruction == 'y');
        let number = split_line.next().unwrap().parse::<i64>().unwrap();
        instructions.push((instruction, number));
    }
    instructions
}
