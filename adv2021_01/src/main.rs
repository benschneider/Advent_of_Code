fn main() {
    let inputstr = include_str!("../input.txt");
    let depths = inputstr
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let d_incs = depths.fwd_diff().iter().filter(|&d| d > &0).count();
    println!("individual increases (sol part 1) :{}", d_incs); // how many times it increased

    let m = depths
        .fwd_sum(3)
        .fwd_diff()
        .iter()
        .filter(|&d| d > &0)
        .count();
    println!("group increases (sol part 2) :{}", m); // how many times the sum of the window increased
}

// Just wanted to try making a trait for the vector type
pub trait FwdDiff {
    fn fwd_diff(&self) -> Self;
}

pub trait FwdSum {
    fn fwd_sum(&self, w: usize) -> Self;
}

impl FwdDiff for Vec<i64> {
    // fn fwd_diff(input_numbers: Vec<i64>) -> Vec<i64> {
    fn fwd_diff(self: &Vec<i64>) -> Vec<i64> {
        let input_numbers = self;
        let mut s: i64 = input_numbers[0]; // start with first entry
        let mut ret_vec: Vec<i64> = Vec::new();
        for &e in input_numbers {
            ret_vec.push(e - s);
            s = e;
        }
        ret_vec
    }
}

impl FwdSum for Vec<i64> {
    // moving window:
    fn fwd_sum(self: &Vec<i64>, w: usize) -> Vec<i64> {
        let mut w_vec: Vec<i64> = Vec::new();
        for i in 0..=self.len() - w {
            let k: i64 = self[i..i + w].iter().sum();
            w_vec.push(k);
        }
        w_vec
    }
}
