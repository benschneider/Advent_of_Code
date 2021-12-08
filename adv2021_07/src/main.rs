use std::str::FromStr;

fn main() {
    let input = include_str!("input");
    let mut t: Vec<_> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|num_str| i64::from_str(num_str).unwrap())
        .collect();
    // println!("{:?}", t);

    // for debugging:
    // let mut t = vec![16,1,2,0,4,2,7,1,2,14];

    // get median
    t.sort();
    let idx: usize = t.len()/2; // rounds down
    let mut median = t[idx];
    if t.len()%2 == 0{
        median += t[idx+1];
        median /= 2;
    }
    
    // get mean
    let tsum: i64 = t.iter().sum();
    let mean: f64 = tsum as f64 / t.len() as f64;


    dbg!("{}", median); 
    dbg!(mean.ceil());

    
    // ideal pos is somewhere between median and mean
    // costs are high enough that the mean should be the position

    let mean_cei = mean.ceil() as i64;
    dbg!(mean_cei);
    dbg!(mean as i64); // should be the position;
    let mut fuel_vec: Vec<(i64, usize)> = vec![];
    for pos in 0..=t.len(){
        let fuel = cal_fuel_costs(&t, &(pos as i64));
        fuel_vec.push((fuel, pos));
    }

    fuel_vec.sort_by(|a,b| a.0.cmp(&b.0));
    dbg!(fuel_vec.first());

}


fn cal_fuel_costs(in_vec: &Vec<i64>, pos:&i64) -> i64 {
    let mut cost: i64 = 0;
    for d_pos in in_vec {
        let distance = (*d_pos - pos).abs();
        let fuel_cost = distance*(distance+1)/2;
        cost += fuel_cost;
    }
cost
}
