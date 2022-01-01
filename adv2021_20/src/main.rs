use std::collections::HashSet;

fn main() {
    // let input = include_str!("../input/input");
    let input = include_str!("../input/input_test");
    let (enhancement, input_set) = parse_input(input);

    let (mut output_set, mut outside) = get_output_set(&input_set, &enhancement, '.');
    for _i in 1..50 {
        let out = get_output_set(&output_set, &enhancement, outside);
        output_set = out.0;
        outside = out.1;
    }
    // lets print the output set
    plot_output_set(&input_set);
    plot_output_set(&output_set);

    println!("{}", output_set.len());
}

fn plot_output_set(output_set: &HashSet<(i32, i32)>) {
    let (min_x, max_x, min_y, max_y) = get_boundaries(output_set);
    for y in min_y - 2..=max_y + 2 {
        for x in min_x - 2..=max_x + 2 {
            if output_set.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_output_set(
    input_set: &HashSet<(i32, i32)>,
    enhancement: &Vec<char>,
    mut outside: char,
) -> (HashSet<(i32, i32)>, char) {
    let (min_x, max_x, min_y, max_y) = get_boundaries(&input_set);
    let mut output_set: HashSet<(i32, i32)> = HashSet::new();
    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            let mut lookup: usize = 0b0000_0000_0000_0000;
            for (i, (dx, dy)) in [
                (1, 1),
                (0, 1),
                (-1, 1),
                (1, 0),
                (0, 0),
                (-1, 0),
                (1, -1),
                (0, -1),
                (-1, -1),
            ]
            .iter()
            .enumerate()
            {
                let x1 = x + dx;
                let y1 = y + dy;
                if input_set.contains(&(x1, y1)) {
                    lookup |= 1 << i;
                    // println!("{:09b}, i{}", lookup, i);
                } else if x1 < min_x || x1 > max_x || y1 < min_y || y1 > max_y {
                    if outside == '#' {
                        lookup |= 1 << i;
                    }
                }
            }
            if enhancement[lookup] == '#' {
                output_set.insert((x, y));
            }
        }
    }
    if outside == '.' {
        outside = enhancement[0];
    } else {
        outside = enhancement[511];
    }

    (output_set, outside)
}

fn get_boundaries(input_set: &HashSet<(i32, i32)>) -> (i32, i32, i32, i32) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for (x, y) in input_set {
        let x = *x;
        let y = *y;
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }
    (min_x, max_x, min_y, max_y)
}

fn parse_input(input: &str) -> (Vec<char>, HashSet<(i32, i32)>) {
    let split_input: Vec<&str> = input.split("\n\n").collect();
    let image_enhancement_algorithm: Vec<char> = split_input[0].replace("\n", "").chars().collect();
    // lets turn image_input into a hashset we only record the row and column position of
    // characters which are not '.'
    let image_input = split_input[1]; //input.split("\n\n").collect::<Vec<&str>>()[1];
    let mut image_hashset: HashSet<(i32, i32)> = HashSet::new();
    for (row, line) in image_input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c != '.' {
                image_hashset.insert((col as i32, row as i32));
            }
        }
    }
    (image_enhancement_algorithm, image_hashset)
}
