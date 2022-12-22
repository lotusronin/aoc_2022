use std::fs::File;
use std::vec::Vec;
use std::collections::HashSet;
use std::io::{BufRead, BufReader};

fn main() {
    let fname = "input.txt";
    //let fname = "practice.txt";
    let input = File::open(fname).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        let l = line.unwrap();
        let mut count = 0;
        for x in l.as_bytes().windows(4) {
            let set : HashSet<u8> = x.iter().cloned().collect();
            if set.len() == 4 {
                println!("Part 1: Found End at {}",count+4);
                break;
            }
            count += 1;
        }
        count = 0;
        for x in l.as_bytes().windows(14) {
            let set : HashSet<u8> = x.iter().cloned().collect();
            if set.len() == 14 {
                println!("Part 2: Found Message at {}",count+14);
                break;
            }
            count += 1;
        }
    }

    //part 1
    //let total_priority : i32 = values.iter().sum();
    //println!("Part 1 top of stack: {}", res);
    //println!("Part 2 Total Parital overlap count: {}", partials);
    //println!("Part 2 Total Badge Priority is: {}", total_priority2);
}
