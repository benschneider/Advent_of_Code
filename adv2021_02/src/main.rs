fn main() {
    let inputstr = include_str!("../input.txt");

    let mut x = 0; // horizontal position
    let mut aim = 0; // value used (depth) z = (x*aim)
    let mut z = 0; // depth

    /*
    let _part1: Vec<(i32, i32)> = inputstr
        .lines()
        .map(|line| {
            let mut pp = line.split_ascii_whitespace();
            let dir = pp.next().unwrap();
            let val: i32 = pp.next().unwrap().parse().unwrap();
            let (xu, zu) = position_update(dir, val);
            x += xu;
            z += zu;
            (x, z)
        })
        .collect();
        */

    let _part2: Vec<(i64, i64)> = inputstr
        .lines()
        .map(|line| {
            let mut pp = line.split_ascii_whitespace();
            let dir = pp.next().unwrap();
            let val: i64 = pp.next().unwrap().parse().unwrap();
            let (xu, aimu) = position_update(dir, val);
            aim += aimu;
            x += xu;
            z += xu * aim;
            (x, z)
        })
        .collect();

    println!("x {}, z {}, x*z {}", x, z, x * z);
}

fn position_update(dir: &str, val: i64) -> (i64, i64) {
    let mut x = 0; // horizontal position
    let mut zu = 0; // depth
    if dir == "forward" {
        x += val;
    } else if dir == "up" {
        zu -= val;
    } else if dir == "down" {
        zu += val;
    }
    (x, zu)
}
