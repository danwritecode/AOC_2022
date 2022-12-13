use anyhow::Result;
use petgraph::visit::IntoNodeIdentifiers;
use std::collections::HashMap;
use petgraph::Graph;
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
    let edges: Vec<(u32, u32)> = create_edges(&input);

    let g = Graph::<u32, ()>::from_edges(&edges);
    let node_map = dijkstra(&g, 1.into(), Some(4.into()), |_| 1);
    
    let nodes = g.node_identifiers();

    for node in nodes {
        println!("{:?}",g.node_weight(node));
    }

    println!("{:?}", g);


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

fn create_edges(input: &Vec<Vec<char>>) -> Vec<(u32, u32)> {
    let mut edges: Vec<(u32, u32)> = vec![];

    for (li,line) in input.iter().enumerate() {
        for (ci, char) in line.iter().enumerate() {
            // left
            if ci > 0 {
                edges.push((*char as u32, line[ci - 1] as u32));
            }
            // right
            if ci < line.len() - 1 {
                edges.push((*char as u32, line[ci + 1] as u32));
            }
            // up
            if li > 0 {
                edges.push((*char as u32, input[li - 1][ci] as u32));
            }
            // down
            if li < input.len() - 1 {
                edges.push((*char as u32, input[li + 1][ci] as u32));
            }
        }
    }
    return edges;
}

