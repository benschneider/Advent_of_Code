#![allow(unused_imports)]
#![allow(dead_code)]
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::str::FromStr;

type InputMap<'a> = HashMap<&'a str, Vec<&'a str>>;
type PathMap<'a> = Vec<Vec<&'a str>>;
type Path<'a> = Vec<&'a str>;

fn main() {
    // first test with the example
    let _small_test = include_str!("../input/small_test");
    // let input_map = process_input(small_test);
    // part1(input_map);
    // part2(small_test);

    // let input_test = include_str!("../input/input_test");
    // let input_map = process_input(input_test);
    // part1(input_map);
    // part2(input_test);

    // println!("Now for the real deal...");
    let input = include_str!("../input/input");
    // let input_map = process_input(input);
    // part1(input_map);
    // let input_map = process_input(input);
    part2(input);
    // part2(input); // for part 2 we need to find the moment where all numbers in the map are at
    //the same time reset to 0, i.e. when we have a syncronized flash.
}

fn process_input(input: &str) -> HashMap<&str, Vec<&str>> {
    // Like to have a hashmap, which for any given key returns a vector of values/keys
    // Meaning the vector will be expanded as we go along
    let mut input_set: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let mut parts = line.split('-').collect_vec();
        let a = parts.swap_remove(0);
        let b = parts.swap_remove(0);
        // or_insert will create a new vector if it doesn't exist
        input_set.entry(a).or_insert_with(Vec::new).push(b);
        input_set.entry(b).or_insert_with(Vec::new).push(a);
        // how to remove duplicates?
        input_set.entry(a).or_insert_with(Vec::new).dedup();
        input_set.entry(b).or_insert_with(Vec::new).dedup();
    }
    input_set // all connections for each key
}

fn part1(input_map: HashMap<&str, Vec<&str>>) {
    // we need to find all possible path/combinations that connect start to end
    // we can do this by using a depth first search
    // lower case keys/values can only be used once
    // capital letter keys/values can be used multiple times
    // we can use a hashset to keep track of visited nodes
    /* Example input map
     input_map = {
        "start": [
            "A",
            "b",
        ],
        "b": [
            "start",
            "A",
            "d",
            "end",
        ],
        "A": [
            "start",
            "c",
            "b",
            "end",
        ],
        "c": [
            "A",
        ],
        "d": [
            "b",
        ],
        "end": [
            "A",
            "b",
        ],
    }
    */
    // make things easier before we start by making a filtered hashmap, which only contains keys
    // that can be used.
    let mut filtered_map = filter_invalid_keys(input_map);

    dbg!(&filtered_map);

    let pathmap: Vec<Vec<&str>> = vec![vec![]]; // start with an empty pathmap, which contains all possible paths
    let path: Vec<&str> = vec!["start"]; // start is always the first element in the path
    remove_value_from_map(&mut filtered_map, "start");
    let ret = dfs(filtered_map, pathmap, path);
    let path_options = ret.1;

    // now we need to remove all the paths that don't connect to the end
    let filtered_path_options = path_options
        .into_iter()
        .filter(|path| {
            if let Some(end) = path.last() {
                end == &"end"
            } else {
                false
            }
        })
        .collect_vec();

    dbg!(&filtered_path_options);
    println!("possible path: {}", &filtered_path_options.len());
}

fn part2(input: &str) {
    // now keys with lowecase letters can be used 2 times.
    let input_map = process_input(input);
    let mut filtered_map = input_map.clone();
    let pathmap: Vec<Vec<&str>> = vec![vec![]]; // start with an empty pathmap, which contains all possible paths
    let path: Vec<&str> = vec!["start"]; // start is always the first element in the path
    remove_value_from_map(&mut filtered_map, "start");
    let ret = dfs2(filtered_map, pathmap, path);
    let path_options = ret.1;

    /* now we need to remove all the paths that don't connect to the end
    let filtered_path_options = path_options
        .clone()
        .into_iter()
        .filter(|path| {
            if let Some(end) = path.last() {
                end == &"end"
            } else {
                false
            }
        })
        .collect_vec();
    */
    let mut filtered_path_options = path_options.clone();
    clean_pathmap(&mut filtered_path_options);
    // dbg!(&filtered_path_options);
    // print content of filtered_path_options in a nice way

    /*for path in &filtered_path_options {
        println!("{}", path.join(","));
    }*/

    println!("possible path: {}", &filtered_path_options.len());
}

fn clean_pathmap(pathmap: &mut Vec<Vec<&str>>) {
    // remove all paths that don't connect to the end
    let mut filtered_pathmap = pathmap.clone();
    filtered_pathmap.retain(|path| {
        if let Some(end) = path.last() {
            end == &"end"
        } else {
            false
        }
    });
    *pathmap = filtered_pathmap; // seems good to me
}


fn dfs2<'a>(
    input_map: InputMap<'a>,
    pathmap: PathMap<'a>,
    path: Path<'a>,
) -> (InputMap<'a>, PathMap<'a>, Path<'a>) {
    let input_map = input_map.clone();
    let mut pathmap = pathmap.clone();
    let path = path.clone();

    let current_map = input_map.clone();
    let current_key = path.last().unwrap();
    if current_key == &"end" {
        // clean up the pathmap
        clean_pathmap(&mut pathmap);
        return (current_map, pathmap, path);
    } // we found the end

    let mut path_options = pathmap.clone();

    if let Some(next_keys) = current_map.get(current_key) {
        match next_keys.len() {
            0 => {
                // bad path, return empty path and pathmap
                let empty_path: Vec<&str> = vec![];
                let empty_pathmap: Vec<Vec<&str>> = vec![vec![]];
                return (current_map, empty_pathmap, empty_path); // no more keys to use
            }

            _ => {
                // we have path to follow
                // if the current path is invalid we need to discard it

                // we need to check if the current path is valid
                // an invalid path contains lowercase keys that are used more than once
                // after one has already been used twice.
                // let mut invalid_path = false;

                for &next_key in next_keys {
                    // here we prepare the next path
                    let mut new_path = path.clone();
                    let mut new_map = input_map.clone();
                    new_path.push(next_key);
                    reduce_map_under_conditions(&mut new_map, next_key, &new_path);
                    // new_path and new_map contain the infortmation we need to continue the search

                    // recursively call dfs with the new map and path
                    if validate_current_path(&new_path) {
                        let (_new_map, mut new_pathmap, new_path) =
                            dfs2(new_map, pathmap.clone(), new_path);
                        // if we have a valid path we can add it to the path options
                        if !new_path.is_empty() {
                            path_options.append(&mut new_pathmap);
                            path_options.push(new_path);
                        }
                    }
                }

                // remove empty_path and double path_options
                path_options.dedup();
                path_options.retain(|path| !path.is_empty());
            }
        }
        return (current_map, path_options, path);
    }
    panic!("should not happen");
}

fn reduce_map_under_conditions<'a>(
    new_map: &mut HashMap<&'a str, Vec<&'a str>>,
    next_key: &str,
    path: &[&str],
) {
    // dbg!(lowercase_keys_can_be_used_once(path));
    if lowercase_keys_can_be_used_once(path) {
        // if the next key is lowercase, we can only use it once
        // so we need to remove it from the map
        if next_key.to_lowercase() == next_key && next_key != "end" {
            for values in new_map.values_mut() {
                remove_value_from_vec(values, next_key);
            }
        }
    }
}

fn lowercase_keys_can_be_used_once(path: &[&str]) -> bool {
    // check if the path already contains two lowercase keys
    // if it does, we can use lowercase keys only once.
    // if it doesn't, we can use lowercase keys multiple times.
    // true if we still can use lowercase keys twice
    let mut count = 0;
    let mut lowercase_keys: HashMap<&str, i32> = HashMap::new();
    for &key in path {
        if key.to_lowercase() == key {
            let val = lowercase_keys.entry(key).or_insert(0);
            *val += 1;
            if *val >= 2 {
                count += 1;
            }
        }
    }
    count >= 1
}

fn validate_current_path(path: &[&str]) -> bool {
    // check if the path is valid
    // a path is invalid if it contains lowercase keys that more than once after one has already been used twice.
    let mut count = 0;
    let mut lowercase_keys: HashMap<&str, i32> = HashMap::new();
    // let mut path = path.clone();
    for &key in path {
        if key.to_lowercase() == key {
            let val = lowercase_keys.entry(key).or_insert(0);
            *val += 1;
            if *val >= 2 {
                count += 1;
            }
        }
    }
    count < 2
}

fn dfs<'a>(
    input_map: InputMap<'a>,
    pathmap: PathMap<'a>,
    path: Path<'a>,
) -> (InputMap<'a>, PathMap<'a>, Path<'a>) {
    dbg!(&path);
    // we need to find all possible path/combinations that connect start to end
    // we can do this by using a depth first search
    // lower case keys/values can only be used once
    // capital letter keys/values can be used multiple times

    let current_map = input_map.clone(); // figure out at which key we are at
    let current_key = path.last().unwrap();

    // if we are at the end, we are done
    if current_key == &"end" {
        return (current_map, pathmap, path);
    }

    let mut path_options = pathmap.clone();
    // get next possible keys
    if let Some(next_keys) = current_map.get(current_key) {
        match next_keys.len() {
            0 => {
                // we are at a dead end, so we need to backtrack
                return (current_map, pathmap, path);
            }
            _ => {
                // we are at a valid key, so we can continue
                //
                for &next_key in next_keys {
                    // we need to make a copy of the path, so we can add the next key to it
                    let mut new_path = path.clone();
                    new_path.push(next_key);
                    // we need to make a copy of the input map, so we can remove the next key the values
                    let mut new_map = input_map.clone();

                    // remove the next key from all values in the map if the key is a lowercase letter
                    if next_key.to_lowercase() == next_key {
                        remove_value_from_map(&mut new_map, next_key);
                    }

                    // recursively call dfs with the new map and path
                    let (_new_map, mut pathmap, new_path) = dfs(new_map, pathmap.clone(), new_path);
                    // if we have a valid path we can add it to the path options
                    if !new_path.is_empty() {
                        path_options.append(&mut pathmap);
                        path_options.push(new_path);
                    }
                }
                return (current_map, path_options, path);
            }
        }
    }
    // get all the possible next keys
    (current_map, pathmap, path)
}

fn remove_value_from_map(map: &mut HashMap<&str, Vec<&str>>, value: &str) {
    for values in map.values_mut() {
        remove_value_from_vec(values, value);
    }
}

fn remove_value_from_vec(xs: &mut Vec<&str>, value: &str) {
    // A helper function to remove a value from a vector
    if let Some(index) = xs.iter().position(|x| *x == value) {
        xs.remove(index);
    }
}

fn filter_invalid_keys<'a>(
    input_map: HashMap<&'a str, Vec<&'a str>>,
) -> HashMap<&'a str, Vec<&'a str>> {
    // Find invalid keys

    let mut invalid_keys = vec![];
    for (key, value) in input_map.clone() {
        if value[0].to_lowercase() == value[0] && key.to_lowercase() == *key && value.len() == 1 {
            invalid_keys.push(key);
        }
    }

    // dbg!(&invalid_keys);
    // now remove all invalid keys from keys and values in the map
    let mut filtered_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for (key, value) in input_map {
        if !invalid_keys.contains(&key) {
            // also remove all values that are not valid
            let mut filtered_value: Vec<&str> = Vec::new();
            for v in value {
                if !invalid_keys.contains(&v) {
                    filtered_value.push(v);
                }
            }
            filtered_map.insert(key, filtered_value);
        }
    }

    filtered_map
}

//

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowercase_keys_can_be_used_once() {
        assert_eq!(
            lowercase_keys_can_be_used_once(&vec!["a", "b", "c", "a"]),
            true
        );
        assert_eq!(
            lowercase_keys_can_be_used_once(&vec!["a", "b", "c", "b"]),
            true
        );
        assert_eq!(
            lowercase_keys_can_be_used_once(&vec!["a", "c", "c", "d"]),
            true
        );
        assert_eq!(
            lowercase_keys_can_be_used_once(&vec!["a", "b", "c", "d"]),
            false
        );
        assert_eq!(
            lowercase_keys_can_be_used_once(&vec!["a", "b", "c", "e"]),
            false
        );
        assert_eq!(
            lowercase_keys_can_be_used_once(&vec!["ab", "ba", "c", "f"]),
            false
        );
    }

    #[test]
    fn test_remove_value_from_map() {
        let mut map = HashMap::new();
        map.insert("a", vec!["b", "c"]);
        map.insert("b", vec!["c", "d"]);
        map.insert("c", vec!["d", "e"]);
        map.insert("d", vec!["a", "f"]);
        map.insert("e", vec!["f", "g"]);
        map.insert("f", vec!["g", "h"]);
        map.insert("g", vec!["h", "i"]);
        map.insert("h", vec!["i", "j"]);
        map.insert("i", vec!["j", "k"]);
        map.insert("j", vec!["k", "l"]);
        map.insert("k", vec!["l", "m"]);
        map.insert("l", vec!["m", "n"]);
        map.insert("m", vec!["n", "o"]);
        map.insert("n", vec!["o", "p"]);
        map.insert("o", vec!["p", "q"]);
        map.insert("p", vec!["q", "r"]);
        map.insert("q", vec!["r", "s"]);
        map.insert("r", vec!["s", "t"]);
        map.insert("s", vec!["t", "u"]);
        map.insert("t", vec!["u", "v"]);
        map.insert("u", vec!["v", "w"]);
        map.insert("v", vec!["w", "x"]);
        map.insert("w", vec!["x", "y"]);
        map.insert("x", vec!["y", "z"]);
        map.insert("y", vec!["z", "a"]);
        map.insert("z", vec!["a", "b"]);

        let mut map_copy = map.clone();
        remove_value_from_map(&mut map_copy, "a");
        assert_eq!(map_copy["a"], vec!["b", "c"]);
        assert_eq!(map_copy["b"], vec!["c", "d"]);
        assert_eq!(map_copy["d"], vec!["f"]);
        remove_value_from_map(&mut map_copy, "f");
        let empty_vec: Vec<&str> = vec![];
        assert_eq!(map_copy["f"], vec!["g", "h"]);
        assert_eq!(map_copy["d"], empty_vec);
        assert_eq!(map_copy["e"], vec!["g"]);
    }

    #[test]
    fn test_reduce_map_under_condition() {
        let mut map = HashMap::new();
        map.insert("a", vec!["b", "c"]);
        map.insert("b", vec!["c", "d"]);
        map.insert("c", vec!["d", "e"]);
        map.insert("d", vec!["a", "e"]);

        let mut map_copy = map.clone();
        // let path = vec!["a", "b", "c"];
        let path_a = vec!["a", "b", "c", "a"];
        // let path_b = vec!["a", "b", "c", "b"];
        // let path_c = vec!["a", "b", "c", "c"];
        let path_d = vec!["a", "b", "c", "d"];

        reduce_map_under_conditions(&mut map_copy, "a", &path_a);
        assert_eq!(map_copy["a"], vec!["b", "c"]);
        assert_eq!(map_copy["d"], vec!["e"]);

        let mut map_copy2 = map.clone();
        reduce_map_under_conditions(&mut map_copy2, "d", &path_d);
        assert_eq!(map_copy2["a"], vec!["b", "c"]);
        assert_eq!(map_copy2["b"], vec!["c", "d"]);
        assert_eq!(map_copy2["d"], vec!["a", "e"]);

        let mut map_copy3 = map.clone();
        let path = vec!["a", "b", "c", "a"];
        reduce_map_under_conditions(&mut map_copy3, "a", &path);
        assert_eq!(map_copy3["a"], vec!["b", "c"]);
        assert_eq!(map_copy3["b"], vec!["c", "d"]);
        assert_eq!(map_copy3["c"], vec!["d", "e"]);
        assert_eq!(map_copy3["d"], vec!["e"]);

        let mut map_copy4 = map.clone();
        let path = vec!["a", "b", "c", "a"];
        reduce_map_under_conditions(&mut map_copy4, "d", &path);
        assert_eq!(map_copy4["a"], vec!["b", "c"]);
        assert_eq!(map_copy4["b"], vec!["c"]);
        assert_eq!(map_copy4["c"], vec!["e"]); // this needs to pass
        assert_eq!(map_copy4["d"], vec!["a", "e"]);
    }

    #[test]
    fn test_validate_paths() {
        let _test_path: Vec<&str> = vec!["start", "A", "b", "A", "c", "A", "c", "A", "end"];
        assert_eq!(validate_current_path(&_test_path), true);
        let _test_path: Vec<&str> = vec!["start", "A", "b", "A", "c", "A", "c", "A", "a", "b"];
        assert_eq!(validate_current_path(&_test_path), false);
    }
}
