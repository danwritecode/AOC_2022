// use std::collections::HashMap;
use std::collections::VecDeque;
use anyhow::Result;
use itertools::Itertools;


fn main() -> Result<()> {
    let input = include_str!("../test");

    let (crates, instructions) = input.split("\n\n").into_iter().collect_tuple().unwrap();


    // let crates = crates
    //     .lines()
    //     .flat_map(|l|
    //         
    //     )
    //     .into_grouping_map()
    //     .collect::<VecDeque<char>>();

    // let crates = crates
    //     .lines()
    //     .flat_map(|l|
    //         l.chars()
    //         .skip(1)
    //         .step_by(4)
    //         .filter(|c| c.is_alphabetic())
    //     )
    //     .collect::<Vec<char>>();

    // let crates = crates
    //     .lines()
    //     .flat_map(|l|
    //         l.chars()
    //         .skip(1)
    //         .step_by(4)
    //         .enumerate()
    //         .filter(|(_,c)| c.is_alphabetic())
    //     )
    //     .into_grouping_map()
    //     .collect::<VecDeque<char>>();

    println!("{:?}", crates);

    return Ok(());
}

