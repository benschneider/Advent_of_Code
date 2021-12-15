#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;

type LookupMap = HashMap<String, String>;

fn main() {
    // let input_test = include_str!("../input/input_test");
    // part1(input_test);
    let input = include_str!("../input/input");
    part1(input);
    part2(input); // the soluton from: https://github.com/emilyskidsister/aoc/blob/main/p2021_14/src/lib.rs
}

fn part1(input: &str) {
    let (first_part, second_part) = split_input(input);
    let lookup_map = build_lookup_map(second_part);
    println!("{}", first_part);
    let mut ret_str: String;
    ret_str = process_first_str(first_part, &lookup_map);
    for _i in 1..10 {
        ret_str = process_first_str(&ret_str, &lookup_map);
    }

    // println!("{}", &ret_str);
    // need to count the number of times each letter appears in the string
    let mut count_map: HashMap<char, i64> = count_letters(&ret_str);
    // lets println the count map to see what we have
    // sort the count map by the value
    count_map = sort_map_by_value(&count_map);
    // lets print the count map
    dbg!(&count_map);
    // lets print the smallest and largest count
    let smallest_count = count_map.values().min().unwrap();
    let largest_count = count_map.values().max().unwrap();
    // lets print the largest - smallest count
    println!("largest - smallest count {}", largest_count - smallest_count);
}


fn part2(input: &str) {

    let num = solve(input, 40);
    dbg!(&num);
}


// lets plant a tree :). -> see the count solution.
// we will use a lookup map to map the letters to the next letter combination and counts

// How about something like this:
// I got this code from here: https://github.com/emilyskidsister/aoc/blob/main/p2021_14/src/lib.rs
fn count( p1: char, p2: char, reactions: &HashMap<(char, char), char>, 
          memo: &mut HashMap<(char, char, usize), HashMap<char, usize>>,
    iterations: usize,) -> HashMap<char, usize> {
    // what does this function do?
    // it takes in a pair of characters and a hashmap of reactions and returns a hashmap of the counts of each letter
    // it is a recursive function that will keep calling itself until it reaches the end of the string
    // it updates the memo hashmap to save the results of the recursive calls
    // the memo hashmap is a hashmap of hashmaps of hashmaps ;)
    // the first hashmap is the hashmap of the pair of characters
    // the second hashmap is the hashmap of the pair of characters and the number of iterations
    // the third hashmap is the hashmap of the letter and the count
    if iterations == 0 {
        // if we have reached the end of the string, return the hashmap of the counts
        let mut counts = HashMap::new();
        *counts.entry(p1).or_default() += 1; 
        *counts.entry(p2).or_default() += 1;
        return counts;
    }

    if let Some(result) = memo.get(&(p1, p2, iterations)) {
        // if we have already calculated the counts for this pair of characters and the number of iterations
        return result.clone();
    }

    if let Some(c) = reactions.get(&(p1, p2)) {
        // if we have a reaction for this pair of characters
        let mut counts = HashMap::new();
        for (c, count) in count(p1, *c, reactions, memo, iterations - 1) {
            // for each letter and count in the result of the recursive call
            *counts.entry(c).or_default() += count;
        }
        for (c, count) in count(*c, p2, reactions, memo, iterations - 1) {
            *counts.entry(c).or_default() += count;
        }
        *counts.entry(*c).or_default() -= 1;
        memo.insert((p1, p2, iterations), counts.clone());
        counts
    } else {
        let mut counts = HashMap::new();
        *counts.entry(p1).or_default() += 1;
        *counts.entry(p2).or_default() += 1;
        counts
    }
}

// also this part is from here: https://github.com/emilyskidsister/aoc/blob/main/p2021_14/src/lib.rs
// lets try it :).
fn solve(input: &str, iter: usize) -> usize {
    // what does this function do?
    // it takes in a string and an integer and returns the number of letters in the string after the number of iterations
    // it uses a recursive count function called count

    let mut lines = input.trim().split('\n');
    let template: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut reactions = HashMap::new();

    for line in lines {
        let (pattern, addition) = line.split_once(" -> ").unwrap();
        let mut pattern = pattern.chars();
        let p1 = pattern.next().unwrap();
        let p2 = pattern.next().unwrap();
        let addition = addition.chars().next().unwrap();
        reactions.insert((p1, p2), addition);
    }

    let mut memo: HashMap<(char, char, usize), HashMap<char, usize>> = HashMap::new();
    let mut counts: HashMap<char, usize> = HashMap::new();

    for i in 1..template.len() {
        let p1 = template[i - 1];
        let p2 = template[i];
        for (c, count) in count(p1, p2, &reactions, &mut memo, iter) {
            *counts.entry(c).or_default() += count;
        }

        if i != 1 {
            *counts.entry(p1).or_default() -= 1;
        }
    }

    let most_common = counts.values().max().unwrap();
    let least_common = counts.values().min().unwrap();
    most_common - least_common
}


fn sort_map_by_value(map: &HashMap<char, i64>) -> HashMap<char, i64> {
    let mut sorted_map: HashMap<char, i64> = HashMap::new();
    let mut sorted_vec: Vec<(&char, &i64)> = map.iter().collect();
    sorted_vec.sort_by(|a, b| b.1.cmp(a.1));
    for (key, value) in sorted_vec {
        sorted_map.insert(*key, *value);
    }
    sorted_map
}


fn count_letters(s: &str) -> HashMap<char, i64> {
    // returns a hashmap of the number of times each letter appears in the string
    let mut count_map: HashMap<char, i64> = HashMap::new();
    for c in s.chars() {
        let count = count_map.entry(c).or_insert(0);
        *count += 1;
    }
    count_map
}

fn process_first_str<'a>(first_str: &'a str, lookup_map: &'a LookupMap) -> String {
    let len = first_str.len();
    let mut output_string = String::new();

    for i in 0..len-1 {
        let new_str = &first_str[i..i + 2];
        let mut lookup_str = "";

        if lookup_map.contains_key(new_str) {
            lookup_str = &lookup_map[new_str];
        } 
        // now we put the lookup_str between the two chars
        // if the lookup_str is empty, we just put the two chars
        output_string.push_str(&first_str[i..i + 1]);
        output_string.push_str(lookup_str);
    }
    output_string.push_str(&first_str[len-1..len]);
    output_string
}




fn build_lookup_map(input: &str) -> LookupMap {
    let mut lookup_map = LookupMap::new();
    for line in input.lines() {
        let mut parts = line.split(" -> ");
        let from = parts.next().unwrap();
        let to = parts.next().unwrap();
        lookup_map.insert(from.to_string(), to.to_string());
    }
    lookup_map
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

