use std::collections::{HashMap, HashSet};
use std::str::FromStr;
extern crate nalgebra as na;
// use na::{Matrix, SMatrix};

fn main() {
    // let mut inputstr = include_str!("input_test").lines();
    let mut inputstr = include_str!("input").lines();

    // get number random_number input string
    let calls: Vec<i64> = inputstr
        .next()
        .unwrap()
        .split(',')
        .map(|f| i64::from_str(f).unwrap())
        .collect();
    inputstr.next();

    // let mut called: Vec<HashSet<(i32, i32)>> = Vec::new();

    let mut boards: Vec<HashMap<i64, (usize, usize)>> = vec![]; // first contains numbers, second marks for bingo
    let mut y = 0;
    let mut board: HashMap<i64, (usize, usize)> = HashMap::new();
    for line_s in inputstr.into_iter() {
        if line_s.is_empty() {
            y = 0;
            boards.push(board.clone());
            board.clear(); // clear rows for each board
        } else {
            let row_of_numbers: Vec<i64> = line_s
                .split_ascii_whitespace()
                .map(|f| i64::from_str(f).unwrap())
                .collect();

            for (x, num) in row_of_numbers.iter().enumerate() {
                assert!(x < 5 && y < 5);
                assert!(board.len() < 25);
                board.insert(*num, (x, y));
            }
            y += 1;
        }
    }

    dbg!(boards.len());

    // now mark calls
    let mut boards_markings: Vec<HashSet<(usize, usize)>> = vec![]; // here to note what has been marked
    let mut boards_ratings: Vec<(usize, i64)> = Vec::new();
    for (_idx, board) in boards.iter().enumerate() {
        let mut markings: HashSet<(usize, usize)> = HashSet::new();
        //dbg!(idx);
        for (callid, call) in calls.iter().enumerate() {
            // dbg!(call);
            if let Some((x, y)) = board.get(call) {
                markings.insert((*x, *y));
                if check_bingo(&markings, false) || check_bingo(&markings, true) {
                    let score = rate_board(board, &markings) * call;
                    boards_ratings.push((callid, score));
                    //dbg!(callid, call, score);
                    break;
                }
            }
        }
        boards_markings.push(markings);
    }
    //dbg!(boards_markings.len());
    boards_ratings.sort_by(|a, b| b.0.cmp(&a.0));
    dbg!(boards_ratings.last()); // solution part 1
    dbg!(boards_ratings.first()); // solution part 2

    // dbg!(check_bingo(boards_markings.pop().unwrap()));
}

fn rate_board(board: &HashMap<i64, (usize, usize)>, markings: &HashSet<(usize, usize)>) -> i64 {
    let mut score = 0;
    for (num, (x, y)) in board.iter() {
        if !markings.contains(&(*x, *y)) {
            score += num;
        }
    }
    score
}

fn check_bingo(markings: &HashSet<(usize, usize)>, is_cols: bool) -> bool {
    let mut bingo = false;

    for x in 0..5_usize {
        let mut f = true;
        for y in 0..5_usize {
            let check: bool;
            if is_cols {
                check = !markings.contains(&(y, x));
            } else {
                check = !markings.contains(&(x, y));
            }
            if check {
                f = false;
                break;
            }
        }
        if f {
            bingo = true;
        }
    }

    bingo
}

/* Why is it that structs are most of the times useless in comparison to a simple Hashmap ???
// get 5x5 number boards, conceptionally like to do this with a Matrix: //> Not working
struct Board {
    rows: [[u8; 5]; 5],
    rows_marked: [[u8; 5]; 5],
}

impl Board {
    pub fn get(&self, row: usize, col: usize) -> Option<u8> {
        let val = self.rows.get(row)?.get(col)?;
        Some(*val)
    }

    pub fn get_marked(&mut self, row: usize, col: usize) -> Option<u8> {
        let val = self.rows_marked.get(row)?.get(col)?;
        Some(*val)
    }

    pub fn new(&mut self, rows: [[u8; 5]; 5]) -> &Board {
        self.rows_marked = [[0, 0, 0, 0, 0]; 5]; // nothing should be marked
        self.rows = rows;
        self
    }

    pub fn check_bingo(&self) -> bool {
        // bingo checking here
        false
    }

    pub fn dbg_board(&self) {
        dbg!(self.rows);
        dbg!(self.rows_marked);
    }
}
*/
