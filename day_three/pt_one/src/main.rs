use std::collections::HashMap;


fn main() {
    let input = include_str!("../input");
    let lines = input.split('\n').collect::<Vec<&str>>();
    let mut score = 0;

    for line in lines {
        score += find_match_score(line);
    }

    println!("{}", score);
}

fn find_match_score(line: &str) -> i32 {
    let len = line.len();
    let first_half = &line[0..len/2];
    let second_half = &line[len/2..];

    for l_f in first_half.chars() {
        for l_s in second_half.chars() {
            if l_f == l_s {
                return get_letter_weight(l_f);
            }
        }
    }
    return 0;
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
