use anyhow::Result;
use itertools::Itertools;
// use itertools::Itertools;

fn main() -> Result<()> {
    let input = include_str!("../input");

    let input:Vec<Vec<&str>> = input
        .lines()
        .map(|l| {
            return l
                .split_whitespace()
                .collect_vec()
        })
        .collect();

    let addx = [0,1];
    let mut cur_addx = 1;

    let mut cycle_count = 0;
    let cycle_markers = [20, 60, 100, 140, 180, 220];

    let mut register_sum = 0;

    for line in &input {
        // determine instruction type
        if line[0] == "addx" {
            for (i,_cycle) in addx.iter().enumerate() {
                cycle_count += 1;
                // if we're on the last part of the cycle we recalc
                if i == 1 {
                    cur_addx += &line[1].parse::<i32>().unwrap();
                }
                if cycle_markers.contains(&cycle_count) {
                    register_sum += cycle_count * cur_addx;
                }
            }
        } else {
            // add one to cycle count
            cycle_count += 1;
            if cycle_markers.contains(&cycle_count) {
                register_sum += cycle_count * cur_addx;
            }
        }
    }

    dbg!(register_sum);

    return Ok(());
}

