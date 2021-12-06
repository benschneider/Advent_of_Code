// extern crate nalgebra as na;
// #[allow(unused_imports)]
// use na::{dmatrix, Const, DMatrix, Dynamic, Matrix, SMatrix, VecStorage};
// use std::collections::{HashMap, HashSet}; // would be a lot faster!

const M: usize = 1000;
const N: usize = 1000;

// All of the following types crash for large matrices!!!
// type Matrix9x9 = OMatrix<i64, M, N>;
// type Matrix9x9 = SMatrix<u32, M, N>;
// type Matrix9x9 = Matrix<u32, Dynamic, Dynamic, VecStorage<u32, Dynamic, Dynamic>>; // seriously wrong

fn main() {
    // let mut ground_map = vecvec_to_m9x9(m0);
    // let mut ground_map = DMatrix::<u32>::zeros(M, N);
    let m0 = vec![vec![0u32; N]; M]; // "cargo run --release" -> F%#K RUST is fast! :D
    let mut ground_map = m0;

    // let input_string = get_test_input();
    let input_string = include_str!("input");

    let pipe_map: Vec<_> = input_string
        .lines()
        .map(|f| {
            let xy: Vec<Vec<u32>> = f
                .split("->")
                .map(|xy_str| get_xy_from_str(xy_str))
                .collect();
            xy
        })
        .collect();

    for pipe in pipe_map {
        // horizontal pipe or vertical pipe
        // put on map
        let pipemap = put_on_vec_map(pipe);
        // let pipe_map = vecvec_to_m9x9(pipemap);
        ground_map = add_vec_to_vec(ground_map, pipemap);
        // add_vec_to_dm9x9(pipemap, &mut ground_map); // nalgebra is not usefull..
        //let pipe_map = vecvec_to_dm9x9(pipemap);
        //ground_map += pipe_map;
    }
    plot_map(&ground_map);

    // count all values >= 2
    let mut count: i64 = 0;
    for y in 0..M {
        for item in ground_map.iter().take(M) {
            if item[y] >= 2 {
                count += 1;
            }
        }
        /*
        for x in 0..M {
            let val = ground_map[x][y];
            if val >= 2 {
                count += 1;
            }
        }
        */
    }
    dbg!(count);
}

/*
fn plot_mat(mat: &Matrix9x9) {
    for y in 0..M {
        for x in 0..N {
            let val = mat[(y, x)];
            print! {" {}", val};
        }
        println! {};
    }
    println! {};
}
*/

fn plot_map(pipemap: &[Vec<u32>]) {
    for y in 0..M {
        for item in pipemap.iter().take(N) {
            print! {"{}", item[y]};
        }
        println! {};
    }
    println! {};
}

fn put_on_vec_map(pipe: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut pipe_map = vec![vec![0u32; N]; M];

    let (x1, y1) = (pipe[0][0], pipe[0][1]);
    let (x2, y2) = (pipe[1][0], pipe[1][1]);

    let mut xy = vec![(x1, y1), (x2, y2)];
    xy.sort_by(|a, b| a.0.cmp(&b.0));
    let (x1, y1) = xy[0];
    let (x2, y2) = xy[1];
    let dx = x2 as f64 - x1 as f64;
    let dy = y2 as f64 - y1 as f64;
    let m = dy / dx;

    if x1 == x2 {
        // vertical pipes
        let mut y12 = [y1, y2];
        y12.sort_by(|a, b| a.cmp(b));

        for y in y12[0]..=y12[1] {
            let x = x1;
            // dbg!(x, y);
            pipe_map[x as usize][y as usize] += 1;
        }
    } else {
        //if y1 == y2 {
        // horizontal pipes (&& diagonal pipes)

        for xi in 0..=dx.abs() as i64 {
            let y = (m * xi as f64) as i64 + y1 as i64;
            let x = x1 as i64 + xi;
            // dbg!(m, x, y);
            assert![(0..M).contains(&(y as usize))];
            pipe_map[x as usize][y as usize] += 1;
        }
    }

    pipe_map
}

fn get_xy_from_str(in_str: &str) -> Vec<u32> {
    let out: Vec<u32> = in_str
        .split(',')
        .map(|f| {
            let t: String = f.chars().filter(|c| c.is_digit(10)).collect(); // remove garbage
            t.parse::<u32>().unwrap()
        })
        .collect();
    out
}

fn add_vec_to_vec(vec1: Vec<Vec<u32>>, vec2: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    // add vec1 to vec2
    // consumes vec1
    let mut vec_new = vec2;
    for (x, vecu) in vec1.iter().enumerate() {
        for (y, val) in vecu.iter().enumerate() {
            vec_new[x][y] += val;
        }
    }
    vec_new
}

/*
fn add_vec_to_dm9x9(vecvec: Vec<Vec<u32>>, ground_map: &mut Matrix9x9) {
    for (x, vecu) in vecvec.iter().enumerate() {
        for (y, val) in vecu.iter().enumerate() {
            ground_map[(y, x)] += val;
        }
    }
}

fn vecvec_to_m9x9(vecvec: Vec<Vec<u32>>) -> Matrix9x9 {
    let flatvec = vecvec.into_iter().flatten().collect::<Vec<u32>>();
    Matrix9x9::from_vec(flatvec)
}
*/

#[allow(dead_code)]
fn get_test_input() -> String {
    "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2"
        .to_string()
}
