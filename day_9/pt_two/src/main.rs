use anyhow::Result;
use itertools::Itertools;


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
    let mut crt: Vec<Vec<&str>> = vec![vec![]];

    let mut cur_sprite_indices = vec![0,1,2];
    let mut cur_crt_row = 0;
    let mut cur_crt_row_pos = 0;


    for line in &input {
        // determine instruction type
        if line[0] == "addx" {
            for (i,_cycle) in addx.iter().enumerate() {
                // denotes end of line
                if crt[cur_crt_row].len() == 40 {
                    // create new row in vec
                    cur_crt_row += 1;
                    crt.push(vec![]);

                    // push pixel and reset row position
                    cur_crt_row_pos = 0;
                    if cur_sprite_indices.contains(&cur_crt_row_pos) {
                        crt[cur_crt_row].push("#");
                    } else {
                        crt[cur_crt_row].push(".");
                    }
                    cur_crt_row_pos += 1;
                } else {
                    if cur_sprite_indices.contains(&cur_crt_row_pos) {
                        crt[cur_crt_row].push("#");
                    } else {
                        crt[cur_crt_row].push(".");
                    }
                    cur_crt_row_pos += 1;
                }

                if i == 1 {
                    cur_addx += &line[1].parse::<i32>().unwrap();
                    cur_sprite_indices = vec![cur_addx - 1, cur_addx, cur_addx + 1];
                }
            }
        } else {
            // denotes end of line
            if crt[cur_crt_row].len() == 40 {
                // create new row in vec
                cur_crt_row += 1;
                crt.push(vec![]);

                // push pixel and reset row position
                cur_crt_row_pos = 0;
                if cur_sprite_indices.contains(&cur_crt_row_pos) {
                    crt[cur_crt_row].push("#");
                } else {
                    crt[cur_crt_row].push(".");
                }
                cur_crt_row_pos += 1;
            } else {
                if cur_sprite_indices.contains(&cur_crt_row_pos) {
                    crt[cur_crt_row].push("#");
                } else {
                    crt[cur_crt_row].push(".");
                }
                cur_crt_row_pos += 1;
            }
        }
    }

    for row in &crt {
        println!("{:?}", row.join("").replace("."," "));
    }

    return Ok(());
}

