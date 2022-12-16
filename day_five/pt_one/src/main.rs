// use std::collections::HashMap;
use std::collections::{VecDeque, HashMap};
use anyhow::Result;
use itertools::{Itertools, enumerate};
use scan_fmt::scan_fmt_some;


#[derive(Debug)]
struct CrateCommands {
    quantity: u32,
    source: u32,
    destination: u32
}

fn main() -> Result<()> {
    let input = include_str!("../test");

    let (crates, ins) = input.split("\n\n").into_iter().collect_tuple().unwrap();
    
    let crates = crates
        .lines()
        .flat_map(|l| {
            return l
                .chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|(_,c)| c.is_alphabetic())
        })
        .into_grouping_map()
        .collect::<VecDeque<char>>();
        
    
    let mut parsed_ins = vec![];
    
    for line in ins.lines() {
        let (qty, src, dest) = scan_fmt_some!(line, "move {} from {} to {}", u32, u32, u32);
        parsed_ins.push(CrateCommands {
            quantity:qty.unwrap(),
            source:src.unwrap(),
            destination: dest.unwrap()
        });
    }


    println!("{:?}", parsed_ins);

    return Ok(());
}

