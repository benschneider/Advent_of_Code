#![allow(unused_imports)]
use itertools::Itertools;
use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    // println!("{}", include_str!("../input/day10.txt"));

    // first test with the example
    let test_input = include_str!("../input/test_input");
    part1(test_input);
    part2(test_input);

    println!("Now for the real deal...");
    // now solve for real
    let input = include_str!("../input/input");
    part1(input);
    part2(input);
}

fn part1(input_content: &str) {
    // check that all type of brackets are closed
    // find the first bracket that is not closed
    // we go from left to right and check that any closing bracket was opened before

    let mut score = 0;

    let mut brackets_matching = HashMap::new();
    brackets_matching.insert('(', ')');
    brackets_matching.insert(')', '(');
    brackets_matching.insert('{', '}');
    brackets_matching.insert('}', '{');
    brackets_matching.insert('[', ']');
    brackets_matching.insert(']', '[');
    brackets_matching.insert('<', '>');
    brackets_matching.insert('>', '<');

    for input in input_content.lines() {
        // dbg!(&input);
        let mut brackets = HashMap::new();
        brackets.insert('(', 0);
        brackets.insert(')', 0);
        brackets.insert('[', 0);
        brackets.insert(']', 0);
        brackets.insert('{', 0);
        brackets.insert('}', 0);
        brackets.insert('>', 0);
        brackets.insert('<', 0);

        let mut last_opened_bracket: Vec<char> = vec![];
        let mut _expected_closing_bracket: char = ' ';
        'check_chars: for c in input.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                brackets.insert(c, brackets[&c] + 1);
                last_opened_bracket.push(c);
                _expected_closing_bracket = brackets_matching[&c];
            } else if c == ')' || c == ']' || c == '}' || c == '>' {
                let matching_bracket = brackets_matching[&c];
                if brackets[&matching_bracket] == 0
                    || last_opened_bracket.last().unwrap() != &matching_bracket
                {
                    // println!("expected {} but found {}", _expected_closing_bracket, &c);
                    score += get_syntax_score(c);
                    break 'check_chars;
                }
                brackets.insert(c, brackets[&c] - 1);
                last_opened_bracket.pop();
            }
        }
    }

    println!("Part 1: score: {}", score);
}

fn get_syntax_score(in_char: char) -> i64 {
    // depending on the missing closing bracket,
    // we get a score which is depending on the following table:
    //
    //    ): 3 points.
    //    ]: 57 points.
    //    }: 1197 points.
    //    >: 25137 points.
    //
    let mut bracket_score_table = HashMap::new();
    bracket_score_table.insert(')', 3);
    bracket_score_table.insert(']', 57);
    bracket_score_table.insert('}', 1197);
    bracket_score_table.insert('>', 25137);

    bracket_score_table[&in_char]
}

fn part2(input_content: &str) {
    // Now, discard the corrupted lines. The remaining lines are incomplete.
    //
    // Incomplete lines don't have any incorrect characters - instead, they're missing some closing characters at the end of the line. To repair the navigation subsystem, you just need to figure out the sequence of closing characters that complete all open chunks in the line.
    //
    // You can only use closing characters (), ], }, or >), and you must add them in the correct order so that only legal pairs are formed and all chunks end up closed.
    //
    // In the example above, there are five incomplete lines:
    //
    //     [({(<(())[]>[[{[]{<()<>> - Complete by adding }}]])})].
    //     [(()[<>])]({[<{<<[]>>( - Complete by adding )}>]}).
    //     (((({<>}<{<{<>}{[]{[]{} - Complete by adding }}>}>)))).
    //     {<[[]]>}<{[{[{[]{()[[[] - Complete by adding ]]}}]}]}>.
    //     <{([{{}}[<[[[<>{}]]]>[]] - Complete by adding ])}>.
    //
    // Did you know that autocomplete tools also have contests? It's true! The score is determined by considering the completion string character-by-character. Start with a total score of 0. Then, for each character, multiply the total score by 5 and then increase the total score by the point value given for the character in the following table:
    //
    //     ): 1 point.
    //     ]: 2 points.
    //     }: 3 points.
    //     >: 4 points.
    //
    // So, the last completion string above - ])}> - would be scored as follows:
    //
    //     Start with a total score of 0.
    //     Multiply the total score by 5 to get 0, then add the value of ] (2) to get a new total score of 2.
    //     Multiply the total score by 5 to get 10, then add the value of ) (1) to get a new total score of 11.
    //     Multiply the total score by 5 to get 55, then add the value of } (3) to get a new total score of 58.
    //     Multiply the total score by 5 to get 290, then add the value of > (4) to get a new total score of 294.
    //
    // The five lines' completion strings have total scores as follows:
    //
    //     }}]])})] - 288957 total points.
    //     )}>]}) - 5566 total points.
    //     }}>}>)))) - 1480781 total points.
    //     ]]}}]}]}> - 995444 total points.
    //     ])}> - 294 total points.
    //
    // Autocomplete tools are an odd bunch: the winner is found by sorting all of the scores and then taking the middle score. (There will always be an odd number of scores to consider.) In this example, the middle score is 288957 because there are the same number of scores smaller and larger than it.
    //
    // Find the completion string for each incomplete line, score the completion strings, and sort the scores. What is the middle score?

    let mut score_vec = vec![];

    let mut brackets_matching = HashMap::new();
    brackets_matching.insert('(', ')');
    brackets_matching.insert(')', '(');
    brackets_matching.insert('{', '}');
    brackets_matching.insert('}', '{');
    brackets_matching.insert('[', ']');
    brackets_matching.insert(']', '[');
    brackets_matching.insert('<', '>');
    brackets_matching.insert('>', '<');

    #[allow(unused_labels)]
    'line_loop: for input in input_content.lines() {
        // dbg!(&input);
        let mut brackets = HashMap::new();
        brackets.insert('(', 0);
        brackets.insert(')', 0);
        brackets.insert('[', 0);
        brackets.insert(']', 0);
        brackets.insert('{', 0);
        brackets.insert('}', 0);
        brackets.insert('>', 0);
        brackets.insert('<', 0);

        let mut last_opened_bracket_vec: Vec<char> = vec![];
        let mut valid_line = true;
        // let mut expected_closing_bracket: char = ' ';
        'char_loop: for c in input.chars() {
            if c == '(' || c == '[' || c == '{' || c == '<' {
                brackets.insert(c, brackets[&c] + 1);
                last_opened_bracket_vec.push(c);
            } else if c == ')' || c == ']' || c == '}' || c == '>' {
                let matching_bracket = brackets_matching[&c];
                if brackets[&matching_bracket] == 0 || last_opened_bracket_vec.last().unwrap() != &matching_bracket
                {
                    // can ignore this line
                    // set score for this line to 0
                    // and break out of char loop
                    valid_line = false;
                    break 'char_loop;
                }
                brackets.insert(c, brackets[&c] - 1);
                last_opened_bracket_vec.pop();
            }
        }

        // now we count score points depending on the missing closing brackets
        // for this we need to know the expected closing brackets -> brackets_matching;
        if valid_line {
            score_vec.push(score_line_brackets(last_opened_bracket_vec));
        }
    }

    // get median of score_vec
    let mut score_vec_sorted = score_vec.clone();
    score_vec_sorted.sort();
    let mut median = score_vec_sorted[score_vec_sorted.len() / 2];
    if score_vec_sorted.len() % 2 == 0 {
        median = (score_vec_sorted[score_vec_sorted.len() / 2] + score_vec_sorted[score_vec_sorted.len() / 2 - 1]) / 2;
    }
    println!("median score: {}", median);


}


fn score_line_brackets(last_opened_bracket_vec: Vec<char>) -> i64 {

    let mut brackets_matching = HashMap::new();
    brackets_matching.insert('(', ')');
    brackets_matching.insert(')', '(');
    brackets_matching.insert('{', '}');
    brackets_matching.insert('}', '{');
    brackets_matching.insert('[', ']');
    brackets_matching.insert(']', '[');
    brackets_matching.insert('<', '>');
    brackets_matching.insert('>', '<');

    let mut score = 0;
    let mut closing_bracket_vec: Vec<char> = vec![];
    for c in last_opened_bracket_vec {
        closing_bracket_vec.push(brackets_matching[&c]);
    }
    closing_bracket_vec.reverse();

    for c in closing_bracket_vec {
        score *= 5;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => {dbg!(c); 0},
        }
    }

    score
}
