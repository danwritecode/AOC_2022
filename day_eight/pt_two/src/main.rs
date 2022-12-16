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
    let mut high_score = 0;


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
                    let mut r_score = 0;

                    let left = &row[..ti];
                    let mut l_score = 0;

                    let up = &input[..ri];
                    let mut u_score = 0;

                    let down = &input[ri + 1 ..];
                    let mut d_score = 0;

                    let mut tree_score = 0;

                    for rt in right {
                        if rt >= tree {
                            r_score += 1;
                            break;
                        } else {
                            r_score += 1;
                        }
                    }

                    for lt in left.iter().rev() {
                        if lt >= tree {
                            l_score += 1;
                            break;
                        } else {
                            l_score += 1;
                        }
                    }

                    for ut in up.iter().rev() {
                        if &ut[ti] >= tree {
                            u_score += 1;
                            break;
                        } else {
                            u_score += 1;
                        }
                    }

                    for dt in down {
                        if &dt[ti] >= tree {
                            d_score += 1;
                            break;
                        } else {
                            d_score += 1;
                        }
                    }

                    tree_score = r_score * l_score * u_score * d_score;

                    if tree_score > high_score {
                        high_score = tree_score;
                    }
                    dbg!(tree_score);
                }
            }
        }
    }

    dbg!(high_score);
    return Ok(());
}

