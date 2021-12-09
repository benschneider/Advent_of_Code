use std::collections::HashMap;

fn main() {
    // let input = include_str!("input");
    let input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let t: Vec<_> = input
        .lines()
        .map(|line_str| {
            let a: Vec<&str> = line_str.split('|').collect();
            let kk: Vec<Vec<&str>> = a
                .iter()
                .map(|str_part| str_part.split_ascii_whitespace().collect())
                .collect();
            kk
        })
        .collect();

    let _total_num = count_4_nums(&t);
    let _u = identify_num(t.first().unwrap());
    // dbg!(total_num);

    println!("{:?}", t.first());
    println!("{:?}", t.first().unwrap()[0].len());
    part2(input);
}

#[allow(dead_code)]
fn identify_num(input_line: &[Vec<&str>]) -> Vec<u32> {
    // input is a line -> vect of 2 str sections/vecs
    // 0: the 8 digit identifiers,
    // 1: number to be returned later
    // lets first check 0
    let mut num_map = HashMap::new();
    let mut num_len_6: Vec<&str> = vec![];
    let mut num_len_5: Vec<&str> = vec![];

    for &num_str in &input_line[0] {
        match &num_str.len() {
            2 => {
                num_map.insert(1, num_str);
            }
            3 => {
                num_map.insert(7, num_str);
            }
            4 => {
                num_map.insert(4, num_str);
            }
            5 => {
                num_len_5.push(num_str);
            }
            6 => {
                num_len_6.push(num_str);
            }
            7 => {
                num_map.insert(8, num_str);
            }
            _ => {
                unreachable!();
            }
        }
    }

    // decode 9 -> contains(1,4,7), 0 -> contains(1,7,!4) and 6 -> contains(!1) len 6)
    dbg!(&num_map[&1]);
    for n in num_len_6 {
        //dbg!(&n);
        if n.contains(num_map[&1]) {
            dbg!(&n);
            dbg!("Match");
        }
    }
    vec![]
}

fn part2(_input_str: &str) {
    // code by https://github.com/timvisee/advent-of-code-2021/commits?author=timvisee
    println!(
        "{}",
        include_str!("input")
            .lines()
            .map(|line| {
                let (ex, digits) = line.split_once('|').unwrap();
                let ex = ex.split_ascii_whitespace().collect::<Vec<_>>();
                let one = ex.iter().find(|d| d.len() == 2).unwrap();
                let four = ex.iter().find(|d| d.len() == 4).unwrap();
                digits
                    .split_ascii_whitespace()
                    .map(|d| match d.len() {
                        2 => 1,
                        3 => 7,
                        4 => 4,
                        7 => 8,
                        len => match (
                            len,
                            d.bytes().filter(|&b| one.contains(b as char)).count(),
                            d.bytes().filter(|&b| four.contains(b as char)).count(),
                        ) {
                            (5, 1, 3) => 5,
                            (5, 2, 3) => 3,
                            (5, _, 2) => 2,
                            (6, 1, _) => 6,
                            (6, _, 3) => 0,
                            (6, _, 4) => 9,
                            _ => unreachable!(),
                        },
                    })
                    .enumerate()
                    .fold(0, |sum, (i, n)| sum + n * 10_u32.pow(3 - i as u32))
            })
            .sum::<u32>()
    );
}

fn count_4_nums(input_lines: &[Vec<Vec<&str>>]) -> u32 {
    let mut total_num: u32 = 0;
    for line in input_lines {
        let num: usize = line[1]
            .iter()
            .filter(|&&x| {
                let xl = x.len();
                xl == 2 || xl == 3 || xl == 4 || xl == 7
            })
            .count();
        total_num += num as u32;
    }
    total_num
}
