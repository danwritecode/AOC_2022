use std::collections::HashMap;

// read file input
// split the huge string by line "\n"

// create dicts for scoring
// rock = 1
// paper = 2
// scissors = 3
// loss = 0
// draw = 3
// win = 6

// Mapping for ABC/XYZ
// A = Rock
// B = Paper
// C = Scissors
// X = Rock
// Y = Paper
// Z = Scissors

// For each line, we need to split by space
// Then we can compare value A with value B
// Using our dict, we determine the score from the move played
// Calculate if we wont the round
// Add the Loss/Draw/Win score to the score that we already had from the move played

fn main() {
    let input = include_str!("../input");
    let lines = input.split("\n");
    // let veclines = lines.collect()::<Vec<&str>>();

    let mut total_score = 0;
    let move_scoring = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3)
    ]);

    // need to map which move loses to what
    // A (rock) loses to Y (paper)
    let move_power = HashMap::from([
        ("A", "Y"),
        ("B", "Z"),
        ("C", "X"),
    ]);

    let move_mapping = HashMap::from([
        ("A", "Rock"),
        ("B", "Paper"),
        ("C", "Scissors"),
        ("X", "Rock"),
        ("Y", "Paper"),
        ("Z", "Scissors"),
    ]);

    for l in lines {
        let moves = l.split(" ").collect::<Vec<&str>>();
        if moves.len() == 2 {
            println!("{} vs {}", move_mapping.get(moves[1]).unwrap(), move_mapping.get(moves[1]).unwrap());
            total_score += move_scoring.get(moves[1]).unwrap();

            if move_mapping.get(moves[0]).unwrap() == move_mapping.get(moves[1]).unwrap() {
                println!("Draw");
                total_score += 3;
            } 
            else if move_power.get(moves[0]).unwrap() == &moves[1] {
                println!("Winning move");
                total_score += 6;
            } 
            else {
                println!("Losing move :(");
                total_score += 0;
            }
            
            println!("---------------------");
        }
    }

    println!("{}", total_score);
}

