use anyhow::Result;
use std::collections::HashMap;
// use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../input");
    let mut trees = HashMap::new();

    for (i, line) in input.lines().enumerate() {
        trees.insert(i, line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>());
    }

    let mut total_seen = 0;
    let num_rows = trees.keys().len();

    for (key, row) in trees {
        // if it's first or last row then we don't want to loop over it
        // instead we just add the length of the row to total seen
        if key == 0 || key == num_rows - 1 {
            total_seen += row.len();
        } else {
            // loop over row for x axis
            let row_len = row.len(); // get length for testing
            // loop over row and compare trees
            for (i, tree) in row.iter().enumerate() {
                dbg!(i);
                // only do work if we're not on the ends of the rows otherwise we can just add one
                // since it means on the ends
                if i != 0 && i + 1 != row_len {
                    // logic to check for X and Y neighbors
                    let right = &row[&row_len - i..];
                    let left = &row[..&row_len - i - 1];

                    for tr in right {
                        if row 
                    }
                } else {
                    total_seen += 1;
                }
            }
        }
    }

    
    dbg!(total_seen);
    return Ok(());
}

