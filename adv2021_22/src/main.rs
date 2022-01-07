use std::collections::HashMap;

type Min = i64; // to be a bit more explicit
type Max = i64;
type Cube = Vec<(Min, Max)>;

fn main() {
    // let input: &str = include_str!("../input/input_test_minimal");
    // let input: &str = include_str!("../input/input_test");
    let input: &str = include_str!("../input/input");
    // let input: &str = include_str!("../input/test_part2");
    let input_map: Vec<(bool, Vec<(Min, Max)>)> = parse_input(input);
    let mut on_map: HashMap<Vec<(Min, Max)>, i64> = HashMap::new();

    for (new_on, new_cube) in input_map {
        for (old_cube, old_value) in on_map.clone().iter() {
            let (overlap, overlap_cube) = get_overlap(&new_cube, old_cube);
            if overlap {
                let ovl = on_map.entry(overlap_cube.clone()).or_insert(0);
                *ovl += old_value * -1;
                if *ovl == 0 { on_map.remove(&overlap_cube); }
            }
        }
        if new_on {
            let entry = on_map.entry(new_cube).or_insert(0);
            *entry += 1;
        }
    }

    let mut total_volume = 0;
    for (cube, val) in on_map.clone() {
        let vol = cube
            .iter()
            .fold(1, |acc, (min, max)| acc * ((max - min).abs() + 1));
        total_volume += vol * val;
    }
    println!("{}", total_volume);
}

fn get_overlap(cube_new: &[(Min, Max)], cube_old: &[(Min, Max)]) -> (bool, Cube) {
    let mut overlap = true;
    let mut overlap_vec: Vec<(Min, Max)> = vec![];
    'overlap_check: for (i, (min1, max1)) in cube_old.iter().enumerate() {
        let min2 = cube_new[i].0;
        let max2 = cube_new[i].1;
        let (ax_overlap, (a, b)) = get_overlap_1d(min1, max1, &min2, &max2);
        if !ax_overlap {
            overlap = false;
            break 'overlap_check;
        }
        overlap_vec.push((a, b));
    }
    (overlap, overlap_vec)
}

fn get_overlap_1d(min1: &Min, max1: &Max, min2: &Min, max2: &Max) -> (bool, (Min, Max)) {
    if min2 <= max1 && max2 >= min1 {
        (true, (*min1.max(min2), *max1.min(max2)))
    } else {
        (false, (0, 0))
    }
}

fn parse_input(input: &str) -> Vec<(bool, Vec<(Min, Max)>)> {
    let mut toggle_map: Vec<(bool, Vec<(Min, Max)>)> = vec![];

    for line in input.lines() {
        // example line: "on x=-27..23,y=-28..26,z=21..29"
        if line.is_empty() {
            continue; // skip empty lines
        }
        let mut parts = line.split_ascii_whitespace();
        let toggle = parts.next().unwrap();
        let ranges = parts.next().unwrap(); //
        let ran_parts = ranges.split(',');

        let mut ax_order = vec!['z', 'y', 'x'];
        let area_box: Vec<(i64, i64)> = ran_parts
            .map(|part| {
                let ax: char = part[..1].chars().next().unwrap();
                let (min, max) = part[2..].split_once("..").unwrap();
                let val1: i64 = min.parse().unwrap();
                let val2: i64 = max.parse().unwrap();
                assert!(ax == ax_order.pop().unwrap()); // confirm axis order x, y, z
                (val1.min(val2), val1.max(val2))
            })
            .collect();

        toggle_map.push((toggle == "on", area_box));
    }
    toggle_map
}
