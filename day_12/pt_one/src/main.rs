use anyhow::Result;
use petgraph::prelude::{GraphMap, UnGraphMap};
use petgraph::visit::IntoNodeIdentifiers;
use std::collections::HashMap;
use petgraph::algo::dijkstra;
// use itertools::Itertools;


fn main() -> Result<()> {
    let input = include_str!("../input").split("\n");

    let input: Vec<Vec<char>> = input
        .filter(|c| { return !c.is_empty() })
        .map(|l| {
            return l
                .chars()
                .collect()
        })
        .collect();
    
    // Identify positions of S and E
    let start_end = get_start_end(&input);
    let mut edges: Vec<(i32, i32)> = create_edges(&input);

    let g = UnGraphMap::<i32, ()>::from_edges(&edges);
    let node_map = dijkstra(&g, 83, Some(69), |_| 1);
    
    // let nodes = g.node_identifiers();

    println!("{:?}", edges);
    println!("{:?}", node_map);


    return Ok(());
}

fn get_start_end(input: &Vec<Vec<char>>) -> HashMap<char, Vec<usize>> {
    let mut start_end = HashMap::new();

    for (li,line) in input.iter().enumerate() {
        for (ci, char) in line.iter().enumerate() {
            if char == &'S' {
                start_end.insert('S', vec![ci, li]);
            }
            if char == &'E' {
                start_end.insert('E', vec![ci, li]);
            }
        }
    }
    return start_end;
}

fn create_edges(input: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    let mut edges: Vec<(i32, i32)> = vec![];

    for (li,line) in input.iter().enumerate() {
        for (ci, char) in line.iter().enumerate() {
            // left
            if ci > 0 {
                let dif = line[ci - 1] as i32 - *char as i32;
                if dif <= 1 || *char as i32 == 69 || *char as i32 == 83 {
                    edges.push((*char as i32, line[ci - 1] as i32));
                }
            }
            // right
            if ci < line.len() - 1 {
                let dif = line[ci + 1] as i32 - *char as i32;
                if dif <= 1 || *char as i32 == 69 || *char as i32 == 83 {
                    edges.push((*char as i32, line[ci + 1] as i32));
                }
            }
            // up
            if li > 0 {
                let dif = input[li - 1][ci] as i32 - *char as i32;
                if dif <= 1 || *char as i32 == 69 || *char as i32 == 83 {
                    edges.push((*char as i32, input[li - 1][ci] as i32));
                }
            }
            // down
            if li < input.len() - 1 {
                let dif = input[li + 1][ci] as i32 - *char as i32;
                if dif <= 1 || *char as i32 == 69 || *char as i32 == 83 {
                    edges.push((*char as i32, input[li + 1][ci] as i32));
                }
            }
        }
    }
    return edges;
}

