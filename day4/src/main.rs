use std::fs::File;
use std::vec::Vec;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

fn main() {
    let fname = "input.txt";
    //let fname = "practice.txt";
    let input = File::open(fname).unwrap();
    let buffered = BufReader::new(input);
    let mut values = Vec::new();
    let mut inclusives = 0;
    let mut partials = 0;
    for line in buffered.lines() {
        let l = line.unwrap();
        if l.eq("") { continue; }
        let (e1,e2) = l.split_once(",").expect("bad parse");
        //println!("{} {}",e1,e2);
        let (l1, h1) = e1.split_once("-").expect("bad parse");
        //println!("{} {}",l1,h1);
        let (l2, h2) = e2.split_once("-").expect("bad parse");
        //println!("{} {}",l2,h2);
        let l1 = l1.parse::<i32>().unwrap();
        let h1 = h1.parse::<i32>().unwrap();
        let l2 = l2.parse::<i32>().unwrap();
        let h2 = h2.parse::<i32>().unwrap();
        let r1 = (l1..=h1);
        let r2 = (l2..=h2);
        let tmp1 = r1.contains(&l2);
        let tmp2 = r1.contains(&h2);
        let r1_inclusive = tmp1 && tmp2;
        let tmp3 = r2.contains(&l1);
        let tmp4 = r2.contains(&h1);
        let r2_inclusive = tmp3 && tmp4;
        if r1_inclusive || r2_inclusive { inclusives += 1; }
        if tmp1 || tmp2 || tmp3 || tmp4 { partials += 1; }
        
        //println!("Shared item is {}, prio {}", shared_item, priority);
        values.push(0);
    }

    //part 1
    //let total_priority : i32 = values.iter().sum();
    println!("Part 1 Total Overlap count: {}", inclusives);
    println!("Part 2 Total Parital overlap count: {}", partials);
    //println!("Part 2 Total Badge Priority is: {}", total_priority2);
}
