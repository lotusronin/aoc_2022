use std::fs::File;
use std::vec::Vec;
use std::io::{BufRead, BufReader};

fn main() {
    let fname = "input.txt";
    //let fname = "practice.txt";
    let input = File::open(fname).unwrap();
    let buffered = BufReader::new(input);
    let mut values = Vec::new();
    let mut values_pt2 = Vec::new();
    for line in buffered.lines() {
        let l = line.unwrap();
        if l.eq("") { continue; }

        let (opponent, you) = l[..3].split_at(1);
        let choice_score =  match &you[1..] {
            "X" => 1, //rock //loss
            "Y" => 2, //paper //draw
            "Z" => 3, //scissors //win
            _ => 0
        };
        let opponent_score =  match opponent {
            "A" => 1, //rock
            "B" => 2, //paper
            "C" => 3, //scissors
            _ => 0
        };
        let diff = choice_score - opponent_score;
        let result =
            if diff == 0 {
                3 //draw
            } else if diff == 1 || diff == -2 {
                6 // win
            } else {
                0
            };
        let round_score = result + choice_score;
        //part 2 calculations
        let req_outcome = choice_score;
        let outcome_score = match req_outcome {
            1 => 0,
            2 => 3,
            3 => 6,
            _ => 0
        };
        let required_move = //(opponent_score + (req_outcome-2)) % 3;
            if outcome_score == 3 {
                // same score as opponent
                opponent_score
            } else if outcome_score == 6 {
                //Must win
                if opponent_score == 3 { 1 } else { opponent_score+1 }
            } else {
                //Must Lose
                if opponent_score == 1 { 3 } else { opponent_score-1 }
            };
        let pt2_score = required_move + outcome_score;
        //println!("Opp: {}, Outcome: {}, Req move: {}, Score: {}", opponent, you, required_move, pt2_score);
        values.push(round_score);
        values_pt2.push(pt2_score);
    }

    //part 1
    let total_score : i32 = values.iter().sum();
    println!("Part 1 Total Score is: {}", total_score);
    let pt2_total_score : i32 = values_pt2.iter().sum();
    println!("Part 2 Total Score is: {}", pt2_total_score);
    // part 2
    // X - Lose
    // Y - Draw
    // Z - Win
}
