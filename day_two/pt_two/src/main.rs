use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;
use std::path::Path;


fn main() -> Result<(), Box<dyn Error>> {
    let mut highest_cal_elves: Vec<i32> = Vec::new();
    let mut cur_cal_total: i32 = 0;

    if let Ok(lines) = read_lines("input") {
        for line in lines {
            if let Ok(cals) = line {
                // if not empty, then we add to the current total for this set
                if !cals.is_empty() {
                    cur_cal_total += cals.parse::<i32>().unwrap();
                }
                // if empty, it means a new set of calories
                if cals.is_empty() {
                    // just push to the vec
                    highest_cal_elves.push(cur_cal_total);
                    cur_cal_total = 0;
                }
            }
        }
    }

    // sort totals
    // highest_cal_elves.sort();
    //
    // // reverse it
    // highest_cal_elves.reverse();

    highest_cal_elves.sort_by(|a,b| b.partial_cmp(a).unwrap());

    // create variable to hold the total
    let mut chad_elf_total: i32 = 0;

    // loop through a slice of the vec ([0..3]), top 3
    for cals in &highest_cal_elves[0..3] {
        // add to the existing total
        chad_elf_total += cals
    }

    println!("{:?}", chad_elf_total);
    Ok(())
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
