use std::fs::File;
use std::vec::Vec;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let fname = "input.txt";
    //let fname = "practice.txt";
    let input = File::open(fname).unwrap();
    let buffered = BufReader::new(input);
    let mut values = Vec::new();
    let mut badges = Vec::new();
    let mut set : HashSet<u32> = HashSet::new();
    let mut iter = 0;
    for line in buffered.lines() {
        let l = line.unwrap();
        if l.eq("") { continue; }
        let length = l.len();
        let (con1,con2) = l.split_at(length/2);
        let mut shared_item = '\0';
        for c in con1.chars() {
            if con2.contains(c) {
                shared_item = c;
                break;
            }
        }
        let calc_priority = |item : u32| -> u32 {
            if item >= 'a' as u32 {
                //println!("{} < 'A'", shared_item);
                item - 'a' as u32 + 1
            } else {
                //println!("{} >= 'A'", shared_item);
                item - 'A' as u32 + 27
            }
        };
        let priority = calc_priority(shared_item as u32);
        //println!("Shared item is {}, prio {}", shared_item, priority);
        values.push(priority as i32);
        let char_set : HashSet<u32> = l.chars().map(|c| c as u32).collect();
        if iter == 0 {
            set = char_set;
        } else {
            set = &char_set & &set;
        }
        
        if iter == 2 {
            for x in &set {
                println!("Triplet shared {}",unsafe { char::from_u32_unchecked(*x)});
                let prio = calc_priority(*x);
                badges.push(prio as i32);
            }
            iter = 0;
        } else {
            iter += 1;
        }
    }

    //part 1
    let total_priority : i32 = values.iter().sum();
    println!("Part 1 Total Priority is: {}", total_priority);
    let total_priority2 : i32 = badges.iter().sum();
    println!("Part 2 Total Badge Priority is: {}", total_priority2);
    // part 2
    // X - Lose
    // Y - Draw
    // Z - Win
}
