fn main() {
    // let inputstr = include_str!("input_test");
    let inputstr = include_str!("input");

    let numbers: Vec<u32> = inputstr
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect();

    let lenn = 11;
    let gamma = get_gamma(&numbers, lenn);
    // let epsil = gamma ^ 0b11111; // xor to get epsilon
    let epsil = gamma ^ 0b111111111111; // xor to get epsilon
    println!("gamma {:b}, epsilon {:b}", &epsil, &gamma);
    println!("g {} * e {} = {}", gamma, epsil, gamma * epsil); // solution for Part 1

    let oxy = get_oxy_co2(numbers.clone(), false, lenn); // get o2 rating
    let co2 = get_oxy_co2(numbers, true, lenn); // get co2 rating

    println!("oxy {} * co2 {} = {}", oxy, co2, oxy * co2);
}

#[allow(dead_code)]
fn get_oxy_co2(numbers: Vec<u32>, is_co2: bool, lenn: usize) -> u32 {
    let mut numb = numbers;
    let mut gamma: u32;

    for n in 0..=lenn {
        let ni = lenn - n;
        gamma = get_gamma(&numb, lenn);

        if is_co2 {
            // gamma ^= 0b11111; // xor to get epsilon
            gamma ^= 0b111111111111; // xor to get epsilon
        }

        // println!("gamma {:b}", &gamma);
        let search_bit = get_bit_at(gamma, ni);
        numb = filter_numbers_matching_bit(numb, search_bit, ni);
        if numb.len() == 1 {
            break;
        }
    }
    numb[0]
}

fn filter_numbers_matching_bit(numbers: Vec<u32>, search_bit: usize, n: usize) -> Vec<u32> {
    let ret_v: Vec<u32> = numbers
        .iter()
        .filter(|&&x| search_bit == get_bit_at(x, n))
        .copied()
        .collect();
    ret_v
}

fn get_gamma(num_vec: &[u32], lenn: usize) -> u32 {
    let mut gamma_bits = "".to_string();
    let half_length_vec: f32 = num_vec.len() as f32 / 2.0;

    for n in 0..=lenn {
        let m = lenn - n; // flip iteration direction
        let cc: usize = num_vec.iter().map(|&num| get_bit_at(num, m)).sum();
        let cc = cc as f32;
        if cc < half_length_vec {
            gamma_bits.push('0'); // chars are nicer to handle ?
        } else if cc == half_length_vec {
            gamma_bits.push('1');
        } else {
            gamma_bits.push('1');
        }
    }
    u32::from_str_radix(&gamma_bits, 2).unwrap() // turn string into number
}

fn get_bit_at(input: u32, bit_position: usize) -> usize {
    /*
        let textnum = 0b101100011011;
        let tt = get_bit_at(textnum, 11);
        dbg!(tt);  // tt = 1
    */
    if bit_position < 32 {
        (input & (1 << bit_position) != 0) as usize
    } else {
        0
    }
}

fn pvec_u32_as_bin(in_vec: &Vec<u32>) {
    for i in in_vec {
        println!("{0:12b}", i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit_at() {
        let x: u32 = 0b101100011011;
        assert_eq!(get_bit_at(x, 0), 1);
        assert_eq!(get_bit_at(x, 1), 1);
        assert_eq!(get_bit_at(x, 2), 0);
        assert_eq!(get_bit_at(x, 3), 1);
        assert_eq!(get_bit_at(x, 4), 1);
        assert_eq!(get_bit_at(x, 5), 0);
        assert_eq!(get_bit_at(x, 7), 0);
        assert_eq!(get_bit_at(x, 8), 1);
        assert_eq!(get_bit_at(x, 9), 1);
        assert_eq!(get_bit_at(x, 10), 0);
        assert_eq!(get_bit_at(x, 11), 1);
    }

    #[test]
    fn test_get_gamma_simple2() {
        let x = vec![0b101100, 0b000000, 0b010100];
        assert_eq!(get_gamma(&x, 5), 0b000100);
    }

    #[test]
    fn test_get_gamma_simple() {
        let x = vec![0b101100, 0b010100];
        assert_eq!(get_gamma(&x, 5), 0b111100);
    }

    #[test]
    fn test_get_gamma_4_numbers() {
        let x = vec![
            0b101100011011,
            0b111111111111,
            0b000000000000,
            0b010100101010,
        ];
        assert_eq!(get_gamma(&x, 11), 0b111100111011);
    }

    #[test]
    fn test_filter_numbers_matching_bit_1() {
        let x = vec![0b101100, 0b010100];
        let numb = filter_numbers_matching_bit(x.clone(), 1, 5);
        assert_eq!(numb[0], x[0]);
    }

    #[test]
    fn test_filter_numbers_matching_bit_2() {
        let x = vec![
            0b101100011011,
            0b111111111111,
            0b000000000000,
            0b010100101010,
        ];
        let numb = filter_numbers_matching_bit(x.clone(), 1, 5);
        let res = vec![x[1], x[3]];
        assert_eq!(numb, res);
    }

    #[test]
    fn test_filter_numbers_matching_bit_3() {
        let x = vec![
            0b101100011011,
            0b111111111111,
            0b000000000000,
            0b010100101010,
        ];
        let numb = filter_numbers_matching_bit(x.clone(), 0, 2);
        let res = vec![x[0], x[2], x[3]];
        assert_eq!(numb, res);
    }
}
