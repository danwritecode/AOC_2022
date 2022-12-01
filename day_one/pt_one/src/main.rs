use std::fs::File;
use std::io::{self, BufRead};
use std::error::Error;
use std::path::Path;


fn main() -> Result<(), Box<dyn Error>> {
    let mut highest_cal: i32 = 0;
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
                    // check to see if the cur total beats the high score
                    if cur_cal_total > highest_cal {
                        highest_cal = cur_cal_total;
                        println!("New High Cal: {}", highest_cal);
                    }
                    // set back to zero means new line
                    cur_cal_total = 0;
                }
                println!("Current line cal: {}", cals);
                println!("Current cal total for set: {}", cur_cal_total);
            }
        }
    }

    println!("Highest calorie set: {}", highest_cal);
    Ok(())
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
