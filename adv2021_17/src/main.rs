use std::cmp::Ordering;

fn main() {
    // let input = include_str!("../input/input");
    //  target area: x=241..275, y=-75..-49
    // so max y vel is n=74, reaching a max height of n/2*(n+1) = 2775
    // now for part 2.
    //
    // now we need to find all possible x and y velocities, which will end
    // up being withing x=241...275, y=-75...-49.
    // with each step:
    // The probe's x,y position starts at 0,0. 
    // Then, it will follow some trajectory by moving in steps. 
    // On each step, these changes occur in the following order:
    // The probe's x position increases by its x velocity.
    // The probe's y position increases by its y velocity.
    // Due to drag, the probe's x velocity changes by 1 toward the value 0; 
    // that is, it decreases by 1 if it is greater than 0, 
    // increases by 1 if it is less than 0, or does not change if it is already 0.
    // Due to gravity, the probe's y velocity decreases by 1.

    // the max y velocity is n=74, so the max height is n/2*(n+1) = 2775
    // now we need to find all possible x and y velocities

    // for y > 0 and x > 0 : the y velocity is equal to the starting (y velocity) * -1
    // for y < 0 and x > 0 : the y velocity is equal to the starting (y velocity) * -1
    // since the x target area is positive, x velocity is always positive
    // the max x velocity is when for the first step x is 275.
    // the max y positive velocity is 74. (each step decreases by 1) -> resulting in -1*74 -1 = -75
    // when the height reaches 0 again.
    // the max y negative velocity is -75.
    // the minimum x velocity is given by 241 = (xmin/2)*(xmin+1) -> xmin*xmin + xmin = 241*2 = 482
    // 482**0.5 = 22.5     since velocities have to be integers, we need to round down to 22.
    // so xmin = 22 -> xmin/2 *(xmin+1) = 243 (because for xmin=21 we would just miss the min x
    // range of 241)
    // xmin = 22, xmax = 275, ymin = -75, ymax = 74
    // now we can step through all possible x and y velocities within these ranges. and see if they
    // have a step where the probe reaches the target area.


    let xmin = 22;
    let xmax = 275;
    let ymin = -75;
    let ymax = 74;

    // let taget_area_corner_start = (241, -49);
    // let target_area_corner_end = (275, -75);

    let mut velocities:(i32, i32);
    let mut position:(i32, i32); // start position

    let mut count = 0;
    // let mut step_index:i32;
    for vx in xmin..=xmax{
        for vy in ymin..=ymax{
            velocities = (vx.clone(), vy.clone());
            position = (0, 0);
            // step_index = 0; 
            'step_run: for si in 0..1005 {
                // step_index += 1;
                do_step(&mut position, &mut velocities);
                if position.0 >= 241 && position.0 <= 275 && position.1 <= -49 && position.1 >= -75 {
                    println!("step {}: position {:?}", si, position);
                    println!("step {}: velocities {:?}", si, (vx, vy));
                    count += 1;
                    break 'step_run;
                }
            }
        }
    }
    println!("count {}", count);
    // lets play through a simple example 
    // the probe starts at 0,0
    // it moves to 22,0 (x velocity is 22, y velocity is 0)
    // velocity changes with each step (x decreases by 1 toward 0 and y decreases by 1)
    // new velocity at pos 22,0 is (21, -1)
    // next position is (43, -1) 

}

fn do_step(position: &mut (i32, i32), velocity: &mut (i32, i32)) {
    let (x, y) = *position;
    let (xv, yv) = *velocity;

    let new_position = (x + xv, y + yv);
    let new_velocity : (i32, i32);

    match xv.cmp(&0) {
         Ordering::Greater => new_velocity = (xv - 1, yv - 1),
         Ordering::Less =>new_velocity = (xv + 1, yv - 1) ,
         Ordering::Equal => new_velocity = (xv, yv - 1),
    }

    *position = new_position; // thanks to mutable reference, we can change the position
    *velocity = new_velocity; // good catch Copilot! 
}

