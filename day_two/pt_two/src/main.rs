use std::collections::HashMap;

// Check what your move is
// write if statement for each scenario
// look up accordingly

fn main() {
    let input = include_str!("../input");
    let lines = input.split("\n");

    let mut total_score = 0;
    let move_scoring = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3)
    ]);

    // need to map which move loses to what
    // A (rock) loses to Y (paper)
    let winning_moves = HashMap::from([
        ("A", "Y"),
        ("B", "Z"),
        ("C", "X"),
    ]);

    let losing_moves = HashMap::from([
        ("A", "Z"),
        ("B", "X"),
        ("C", "Y"),
    ]);

    let draw_moves = HashMap::from([
        ("A", "X"),
        ("B", "Y"),
        ("C", "Z"),
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
            println!("{} vs {}", move_mapping.get(moves[0]).unwrap(), move_mapping.get(moves[1]).unwrap());
            let your_move = moves[1];

            if your_move == "Y" {
                // must draw
                let draw_move = draw_moves.get(moves[0]).unwrap();
                total_score += move_scoring.get(draw_move).unwrap();
                total_score += 3
            }

            if your_move == "X" {
                // must lose
                let lose_move = losing_moves.get(moves[0]).unwrap();
                total_score += move_scoring.get(lose_move).unwrap();
            }

            if your_move == "Z" {
                // must win
                let win_move = winning_moves.get(moves[0]).unwrap();
                total_score += move_scoring.get(win_move).unwrap();
                total_score += 6
            }
            
            println!("{}", total_score);
            println!("---------------------");
        }
    }

    println!("{}", total_score);
}

