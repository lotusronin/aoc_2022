use std::fs::File;
use std::vec::Vec;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader};

fn main() {
    let fname = "input.txt";
    //let fname = "practice.txt";
    let input = File::open(fname).unwrap();
    let buffered = BufReader::new(input);
    let mut stacks : Vec<VecDeque<String>> = Vec::new();
    let mut tmp = Vec::new();
    let mut moves = Vec::new();
    let mut part = 1;
    for line in buffered.lines() {
        let l = line.unwrap();
        if l.eq("") { continue; }
        if part == 1 {
            if l.contains("[") {
                //println!("cargo line...");
                tmp.push(l);
            } else {
                //println!("stack label line...");
                let stack_labels : Vec<&str> = l.split(' ').filter(|s| !(*s).eq("")).collect();
                let num_stacks = stack_labels.len();
                for _ in 0..num_stacks { stacks.push(VecDeque::<String>::new()) }
                part = 2;
            }
        } else {
            //println!("move line...");
            moves.push(l);
        }
    }
    for r in tmp.iter() {
        let mut idx = 0;
        let mut stack_num = 0;
        for c in r.chars() {
            //[A] [B] [C]
            // 1   5   9
            if idx >=1 && (idx-1)%4 == 0 {
                if c != ' ' { stacks.get_mut(stack_num).expect("Can't get mut stack").push_front(c.to_string()); }
                stack_num += 1;
            }
            idx += 1;
        }
    }
    /*
    let print_stacks =|| {
    for s in stacks.iter() {
        println!("Stack: ");
        for v in s.iter() {
            println!("   {}",v);
        }
    }
    };
    print_stacks();
    */
    /*
    println!("Starting state");
    for s in stacks.iter() {
        let mut tmp : String = "Stack: ".to_string();
        for v in s.iter() { tmp = tmp + v + " "; }
        println!("{}", tmp);
    }
    println!("");
    */

    let mut cnt = 0;
    for m in moves.iter() {
        //println!("{}", m);
        let parts : Vec<&str> = m.split(' ').collect();
        //move 1 from 2 to 1
        let num = parts.get(1).expect("Can't get num to move").parse::<i32>().unwrap();
        let src = parts.get(3).expect("Can't get src").parse::<u32>().unwrap()-1;
        let dest = parts.get(5).expect("Can't get dest").parse::<u32>().unwrap()-1;

        // Part 1
        /*
        for _ in 0..num {
            let src_stack = stacks.get_mut(src as usize).expect("Getting src stack");
            let cargo = src_stack.pop_back().expect("Getting cargo");
            stacks.get_mut(dest as usize).expect("Getting dest stack").push_back(cargo);
        }
        */

        // Part 2
        let mut v9001 : Vec<String> = Vec::new();
        for _ in 0..num {
            let src_stack = stacks.get_mut(src as usize).expect("Getting src stack");
            let cargo = src_stack.pop_back().expect("Getting cargo");
            v9001.push(cargo);
        }
        v9001.reverse();
        for v in v9001.iter() { stacks.get_mut(dest as usize).expect("Getting dest stack").push_back(v.to_string()); }
        /*
        for s in stacks.iter() {
            let mut tmp : String = "Stack: ".to_string();
            for v in s.iter() { tmp = tmp + v + " "; }
            println!("{}", tmp);
        }
        println!("");
        if cnt == 4 { break; }
        cnt += 1;
        */
    }

    let mut res = "".to_string();
    for s in stacks.iter() {
        res = res + s.back().expect("Top of stack");
    }

    //part 1
    //let total_priority : i32 = values.iter().sum();
    println!("Part 1 top of stack: {}", res);
    //println!("Part 2 Total Parital overlap count: {}", partials);
    //println!("Part 2 Total Badge Priority is: {}", total_priority2);
}
