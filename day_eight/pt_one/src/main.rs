use anyhow::Result;
use std::collections::HashMap;
// use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../input");

    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            return l
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();

    let mut total_seen = 0;
    let num_rows = input.len();


    for (ri, row) in input.iter().enumerate() {
        println!("{:?}", &row);
        let row_len = row.len();
        // check if first or last row
        if ri == 0 || ri == num_rows - 1 {
            total_seen += row_len;
        } else {
            // loop through trees now
            for (ti, tree) in row.iter().enumerate() {
                dbg!(&tree);
                // check if 1st or last tree in row
                if ti == 0 || ti == row.len() - 1 {
                    total_seen += 1;
                } else {
                    let right = &row[ti + 1..];
                    let right_len = right.len();

                    let left = &row[..ti];
                    let left_len = left.len();

                    let up = &input[..ri];
                    let up_len = up.len();

                    let down = &input[ri + 1 ..];
                    let down_len = down.len();

                    let mut seen_ct = 0;

                    for (ri, rt) in right.iter().enumerate() {
                        if rt >= tree {
                            println!("Tree {} cannot be seen from right", tree);
                            break;
                        } 

                        if ri == right_len - 1 {
                            seen_ct += 1;
                            println!("Tree {} is seen from right", tree);
                        }
                    }

                    for (li, lt) in left.iter().enumerate() {
                        if lt >= tree {
                            println!("Tree {} cannot be seen from left", tree);
                            break;
                        } 

                        if li == left_len - 1 {
                            seen_ct += 1;
                            println!("Tree {} is seen from left", tree);
                        }
                    }

                    for (ui, ut) in up.iter().enumerate() {
                        if &ut[ti] >= tree {
                            println!("Tree {} cannot be seen from above", tree);
                            break;
                        } 

                        if ui == up_len - 1 {
                            seen_ct += 1;
                            println!("Tree {} is seen from above", tree);
                        }
                    }

                    for (di, dt) in down.iter().enumerate() {
                        if &dt[ti] >= tree {
                            println!("Tree {} cannot be seen from below", tree);
                            break;
                        } 

                        if di == down_len - 1 {
                            seen_ct += 1;
                            println!("Tree {} is seen from below", tree);
                        }
                    }

                    if seen_ct > 0 {
                        total_seen += 1;
                    }
                }
            }
        }
    }

    dbg!(total_seen);
    return Ok(());
}

