// use std::collections::HashMap;
use std::collections::{VecDeque, HashMap};
use anyhow::Result;
use itertools::{Itertools};
use scan_fmt::scan_fmt_some;


#[derive(Debug)]
struct CrateCommands {
    quantity: usize,
    source: usize,
    destination: usize
}

fn main() -> Result<()> {
    let input = include_str!("../test");

    let (crates, ins) = input.split("\n\n").into_iter().collect_tuple().unwrap();
    
    let mut crates = crates
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
        let (qty, src, dest) = scan_fmt_some!(line, "move {} from {} to {}", usize, usize, usize);
        parsed_ins.push(CrateCommands {
            quantity:qty.unwrap(),
            source:src.unwrap() - 1,
            destination: dest.unwrap() - 1
        });
    }

    for ins in &parsed_ins {
        // loop through number to move
        for _i in 0..ins.quantity {
            let removed = crates.get_mut(&ins.source).unwrap().pop_front(); 
            crates.get_mut(&ins.destination).unwrap().push_front(removed.unwrap());
            dbg!(&removed);
            dbg!(&crates);
        }
    }


    let mut top_crates = vec![];

    for key in crates.keys().sorted() {
        dbg!(key);
        top_crates.push(crates.get(key).unwrap()[0]);
    }

    println!("{:?}", top_crates);

    return Ok(());
}

