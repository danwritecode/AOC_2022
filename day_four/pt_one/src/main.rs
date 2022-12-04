use std::collections::HashMap;


fn main() {
    let input = include_str!("../input");
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut num_contained_ranges = 0;

    for line in lines {
        let mut first_lower: i32 = 0;
        let mut first_upper: i32 = 0;
        let mut sec_lower: i32 = 0;
        let mut sec_upper: i32 = 0;

        if !line.is_empty() {
            let halves = line.split(",").collect::<Vec<&str>>();

            // value of Halves at this point ["2-6", "4-8"]
            for (i, half) in halves.iter().enumerate() {
                // half = "2-6"
                let str_nums = half.split("-").collect::<Vec<&str>>();

                // strNums = ["2", "6"]
                for (ni, str_num) in str_nums.iter().enumerate() {
                    let num = str_num.parse::<i32>().unwrap();
                    if i == 0 { // lower bound
                        if ni == 0 { // first num 
                            first_lower = num;
                        } else { // second num
                            first_upper = num;
                        }
                    } else {
                        if ni == 0 { // first num 
                            sec_lower = num;
                        } else { // second num
                            sec_upper = num;
                        }
                    }
                }
            }
            // println!("{} {} {} {}", first_lower, first_upper, sec_lower, sec_upper);

            if (first_lower <= sec_lower && first_upper >= sec_upper) ||
                (first_lower >= sec_lower && first_upper <= sec_upper)
            {
                // println!("Overlap");
                // println!("{:?}", halves);
                num_contained_ranges += 1;
            }
        }
    }

    println!("{}", num_contained_ranges);
}

