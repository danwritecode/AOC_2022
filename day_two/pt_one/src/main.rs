use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;
use std::path::Path;


fn main() {
    let input = include_str!("../input");

    let lines = input.split("\n");

    let veclines = lines.collect()::<Vec<&str>>();

    for l in lines {
        println!("{}", l);
    }

    // println!("{:?}", lines);
}

