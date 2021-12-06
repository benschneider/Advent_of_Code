use std::str::FromStr;

fn main() {
    // let shape = (3, 2, 4);
    //let a = Array::<u32, _>::zeros(shape); // just not worth using yet..

    // let my_input = vec![3, 4, 3, 1, 2]; the test vector
    let mut input_string = include_str!("input").lines();
    #[allow(unused_variables)]
    let my_input: Vec<u8> = input_string
        .next()
        .unwrap()
        .split(',')
        .map(|f| u8::from_str(f).unwrap())
        .collect();

    let mut fish_nums: Vec<u128> = vec![0; 9];
    for num in my_input {
        fish_nums[num as usize] += 1;
    }
    dbg!(&fish_nums);

    let its = 256;
    for _i in 0..its {
        update_fish(&mut fish_nums);
    }
    dbg!(&fish_nums);
    let t: u128 = fish_nums.iter().sum();
    dbg!(t);
}

fn update_fish(fish_nums: &mut Vec<u128>) {
    let new_fishies: u128 = fish_nums[0];
    for i in 1..9 {
        fish_nums[i - 1] = fish_nums[i];
    }
    // add new fish here
    fish_nums[6] += new_fishies;
    fish_nums[8] = new_fishies;
}
