use std::collections::HashMap;  // import HashMap

const ROLLFREQ: [(i64, i64); 7] = [(3,1),(4,3),(5,6),(6,7),(7,6),(8,3),(9,1)]; // roll frequency for thre dirac dice ;)

type Pos = i64;
type Pscore = i64;
type State = (Pos, Pscore, Pos, Pscore); // active turn is for the first Pos and Pscore

fn main() {
    // no need to parse the input
    // for the test input: Player 1 starts with position 4, Player 2 starts with position 8
    // for the real input: Player 1 starts with position 5, Player 2 starts with position 6 
    part1(5, 8);

    let state: State = (5, 0, 6, 0);
    let mut memo: HashMap<State, (i64, i64)> = HashMap::new();
    let res = part2(state, &mut memo);
    println!("p1 {}", res.0);
    println!("p2 {}", res.1);
}

fn part1(mut player1_position: i64, mut player2_position: i64){
    // we need a deterministic die roll to test the game
    // the board only has position from 1 to 10.
    let mut player1_turn = true; // player 1 starts
    let mut player1_score: i64 = 0;
    let mut player2_score: i64 = 0;
    let mut val: i64;
    let mut i = 0;
    loop {
        // get 3x dice roll combined
        val = roll_dice_det(&mut i);
        val +=roll_dice_det(&mut i);
        val +=roll_dice_det(&mut i);
        if player1_turn {
            player1_position += val -1;
            player1_position %= 10;
            player1_position += 1;
            player1_score += player1_position;
            player1_turn = false;
            }
        else {
            player2_position += val -1;
            player2_position %= 10;
            player2_position += 1;
            player2_score += player2_position;
            player1_turn = true;
        }
        if player1_score >= 1000 || player2_score >= 1000 {
            println!("P1 score: {}, position: {}", &player1_score, &player1_position);
            println!("P2 score: {}, position: {}", &player2_score, &player2_position);
            let lower_score = player1_score.min(player2_score);
            println!("lower_score {} * number of dice runs {} = {}", lower_score, i, lower_score * i);
            break;
        }
    }

}

fn part2(state: State, memo: &mut HashMap<State, (i64, i64)>) -> (i64, i64) {
    // state.0 is the position of player 1
    // state.1 is the score of player 1
    // state.2 is the position of player 2
    // state.3 is the score of player 2

    if let Some(res) = memo.get(&state) {
        return *res;
    }

    if state.3 >= 21 {
        return (0, 1);
    }

    let mut new_state: State = (0, 0, 0, 0);
    let mut w1 = 0;
    let mut w2 = 0;
    let mut pos: i64;
    for (r,f) in ROLLFREQ {
        pos = state.0 + r - 1;
        pos %= 10;
        pos += 1;
        let score = state.1 + pos;

        // new state switches player 1 and player 2
        new_state.0 = state.2;
        new_state.1 = state.3;
        new_state.2 = pos;
        new_state.3 = score;
        
        let (c2, c1) = part2(new_state, memo);  // recursive call, we check if score is >= 21
        memo.entry(new_state).or_insert((c2, c1)); // update the memo
        w1 += c1 * f;
        w2 += c2 * f;
     }
    return (w1, w2);
}

/* Not needed
fn dirak_dice_3x(i : &mut i64) -> Vec<(i64, i64)> {
    // return 1, 2 or 3
    // here are all the possible outcomes
    // 1, 1, 1 = 3
    // 1, 1, 2 = 4
    // 1, 2, 1 = 4
    // 1, 2, 2 = 5
    // 2, 1, 1 = 4
    // 2, 1, 2 = 5
    // 2, 2, 1 = 5
    // 2, 2, 2 = 6
    // 1, 1, 3 = 5
    // 1, 3, 1 = 5
    // 1, 3, 3 = 7
    // 3, 1, 1 = 5
    // 3, 1, 3 = 7
    // 3, 3, 1 = 7
    // 3, 3, 3 = 9
    // 2, 2, 3 = 7
    // 2, 3, 2 = 7
    // 2, 3, 3 = 8
    // 3, 2, 2 = 7
    // 3, 2, 3 = 8
    // 3, 3, 2 = 8
    // 3, 3, 3 = 9
    // 1, 2, 3 = 6
    // 1, 3, 2 = 6
    // 2, 1, 3 = 6
    // 2, 3, 1 = 6
    // 3, 1, 2 = 6
    // 3, 2, 1 = 6
    // in total we have 1x3, 3x4, 6x5, 7x6, 6x7, 3x8, 1x9
    *i += 3;
    vec![(3,1),(4,3),(5,6),(6,7),(7,6),(8,3),(9,1)]
}
*/

fn roll_dice_det(i: &mut i64) -> i64 {
    // increases each time it is called until it reaches 100 and then restarts at 1
    let retval = *i % 100 + 1;
    *i += 1;
    retval
}


