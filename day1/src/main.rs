use std::fs::File;
use std::vec::Vec;
use std::io::{BufRead, BufReader};

fn main() {
    let fname = "input.txt";
    //let fname = "practice.txt";
    let input = File::open(fname).unwrap();
    let buffered = BufReader::new(input);
    let mut values = Vec::new();
    let mut elf_cals = 0;
    for line in buffered.lines() {
        let l = line.unwrap();
        if l.eq("") { 
            values.push(elf_cals);
            elf_cals = 0;
            continue; 
        }
        elf_cals += l.parse::<i32>().unwrap();
    }

    //part 1
    values.sort();
    values.reverse();
    let max_cals = values.first().unwrap();
    println!("Part 1 Max Cals Are: {}", max_cals);
    let sum_3 : i32 = values[..3].iter().sum();
    println!("Part 2 Sum of Top 3: {}", sum_3);
}
