use anyhow::Result;
use std::iter;
use itertools::Itertools;
use ndarray::{arr2, ArrayBase, ArrayView, Array, OwnedRepr, Dim, Axis};
use std::ops::Range;

#[derive(Debug)]
struct Movement {
    dir: char,
    dist: i32
}

struct Coordinate {
    x: usize,
    y: usize
}

fn main() -> Result<()> {
    let input = include_str!("../input");
    let movements = create_movements(input);

    let mut grid = arr2(&[
        [0]
    ]);

    // for _ in 0..5 {
    //     let len_y = grid.len_of(Axis(0));
    //
    //     let col = iter::repeat(0).take(len_y).collect::<Vec<i32>>();
    //
    //     grid.push_column(ArrayView::from(&col)).unwrap();
    // }
    //
    // dbg!(&grid);

    let mut head_loc = Coordinate {
        x: 0,
        y: 0
    };

    // for mv in &movements {
    //     // loop through range of distance
    //     // match on direction
    //     // For right, push a 0 to the current row (the space where the head will be)
    //     // Increment the space where the head IS (was) + 1
    //     // Update head position (wait until after we need to reference it's coordinate)
    //
    //     match mv.dir {
    //         'R' => move_right(&mut grid, mv.dist, &mut head_loc),
    //         'L' => move_left(&mut grid, &mv.dist, &mut head_loc),
    //         'U' => move_up(&mut grid, &mv.dist, &mut head_loc),
    //         'D' => move_down(&mut grid, &mv.dist, &mut head_loc),
    //         _ => ()
    //     }
    // }

    return Ok(());
}

/// Function moves the head and tail right a number of positions.
fn move_right(
    grid: &mut ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>,
    num_moves: i32,
    head_loc: &mut Coordinate
) {
    // First loop through each number of moves (will loop 5 times if num_moves = 5)
    for _n in 0..num_moves {
        // Determine the current length of the x axis
        let len_x = grid.len_of(Axis(1));
        // Determine current length of the y axis
        let len_y = grid.len_of(Axis(0));

        // if the cur point is equal to length of X axis, then we're
        // at the END and need to add a new column
        if head_loc.x == len_x - 1 {
            // This creates a new column shaped like [0,0,0,0,0] if len_y = 5 for example
            // This is required because the new column needs to match the current shape
            // of the existing matrix. Otherwise it throws an error
            let col = iter::repeat(0).take(len_y).collect::<Vec<i32>>();
            // now we can add this new column to the matrix
            grid.push_column(ArrayView::from(&col)).unwrap();
        }        
        // move the tail which is following the head
        grid.row_mut(head_loc.y)[head_loc.x - 1] += 1;

        // move head position 1 to the right
        head_loc.x = head_loc.x + 1;
        head_loc.y = head_loc.y;
    }
}

fn move_left(
    grid: &mut ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>,
    num_moves: i32,
    head_loc: &mut Coordinate
) {

}

fn move_up(
    grid: &mut ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>,
    num_moves: i32,
    head_loc: &mut Coordinate
) {

}

fn move_down(
    grid: &mut ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>,
    num_moves: i32,
    head_loc: &mut Coordinate
) {

}

fn create_movements(input: &str) -> Vec<Movement> {
    let mut movements = vec![];

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let direction: char = iter.next().unwrap().parse().unwrap();
        let distance: i32 = iter.next().unwrap().parse().unwrap();

        movements.push({
            Movement {
                dir: direction,
                dist: distance
            }
        });
    }
    
    return movements;
}
