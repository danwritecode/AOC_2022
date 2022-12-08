use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../test");
    let mut cur_four:Vec<char> = vec![];

    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            if cur_four.len() < 14 {
                cur_four.push(c);
            } else {
                cur_four.drain(0..1);
                cur_four.push(c);

                let length = cur_four.clone().into_iter().unique().collect::<Vec<char>>().len();
                if length == 14 {
                    println!("Index: {}, Unique Length: {}, {:?}", &i + 1, length, cur_four);
                    // return Ok(());

                }
            }
        }
        cur_four.clear();
    }

    return Ok(());
}

