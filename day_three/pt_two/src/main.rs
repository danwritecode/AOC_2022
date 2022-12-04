use std::collections::HashMap;
use itertools::Itertools;


fn main() {
    let input = include_str!("../input");
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut score = 0;

    // loop through lines
    // keep track of times looped
    // add line to new array of current_lines
    // when reach 3, send current_lines to function which will
    // calculate the score
    
    let mut cur_group: Vec<&str> = vec![];

    for (i, line) in lines.iter().enumerate() {
        if !line.is_empty() {
            // we want to push to this current group until we hit mod/3
            cur_group.push(line);
            // if mod 3 = 0 then that means were at a num divisible by 3
            if (i + 1) % 3 == 0 {
                println!("{:?}", cur_group);
                score += find_group_score(&cur_group);
                cur_group.clear();
            }
        }
    }
    println!("{}", score);
}

fn find_group_score(group: &Vec<&str>) -> i32 {
    let mut letter_member = HashMap::new();

    for (i, row) in group.iter().enumerate() {
        if i == 0 {
            row.chars().for_each(|l| { letter_member.insert(l,vec![i]); });
        } else {
            row.chars().for_each(|l| { 
                match letter_member.get_mut(&l) {
                    Some(ct) => ct.push(i),
                    _ => ()
                }
            });
        }
    }
    
    let mut score: i32 = 0;

    for (key, value) in letter_member.into_iter() {
        let mut new_val = value.clone();
        new_val.dedup();

        // if the array is length three, that means it was found in all three groups
        if new_val.len() == 3 {
            score = get_letter_weight(key);
        }
    }

    return score;
}


fn get_letter_weight(letter: char) -> i32 {
    let letter_weights = HashMap::from([
        ('a', 1),
        ('b', 2),
        ('c', 3),
        ('d', 4),
        ('e', 5),
        ('f', 6),
        ('g', 7),
        ('h', 8),
        ('i', 9),
        ('j', 10),
        ('k', 11),
        ('l', 12),
        ('m', 13),
        ('n', 14),
        ('o', 15),
        ('p', 16),
        ('q', 17),
        ('r', 18),
        ('s', 19),
        ('t', 20),
        ('u', 21),
        ('v', 22),
        ('w', 23),
        ('x', 24),
        ('y', 25),
        ('z', 26),
        ('A', 27),
        ('B', 28),
        ('C', 29),
        ('D', 30),
        ('E', 31),
        ('F', 32),
        ('G', 33),
        ('H', 34),
        ('I', 35),
        ('J', 36),
        ('K', 37),
        ('L', 38),
        ('M', 39),
        ('N', 40),
        ('O', 41),
        ('P', 42),
        ('Q', 43),
        ('R', 44),
        ('S', 45),
        ('T', 46),
        ('U', 47),
        ('V', 48),
        ('W', 49),
        ('X', 50),
        ('Y', 51),
        ('Z', 52)
    ]);
    
    return *letter_weights.get(&letter).unwrap();
}
