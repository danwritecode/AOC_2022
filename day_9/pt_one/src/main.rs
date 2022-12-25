use anyhow::Result;
use std::iter;
use itertools::Itertools;
use ndarray::{arr2, ArrayBase, ArrayView, Array, Array2, OwnedRepr, Dim, Axis};
use std::ops::Range;
use num_traits::float::Float;

#[derive(Debug)]
struct Movement {
    dir: char,
    dist: i32
}

#[derive(Debug)]
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

    let mut head_loc = Coordinate {
        x: 0,
        y: 0
    };

    let mut tail_loc = Coordinate {
        x: 0,
        y: 0
    };

    for mv in &movements {
        // loop through range of distance
        // match on direction
        // For right, push a 0 to the current row (the space where the head will be)
        // Increment the space where the head IS (was) + 1
        // Update head position (wait until after we need to reference it's coordinate)

        match mv.dir {
            'R' => move_right(&mut grid, mv.dist, &mut head_loc, &mut tail_loc),
            'L' => move_left(&mut grid, mv.dist, &mut head_loc, &mut tail_loc),
            'U' => move_up(&mut grid, mv.dist, &mut head_loc, &mut tail_loc),
            'D' => move_down(&mut grid, mv.dist, &mut head_loc, &mut tail_loc),
            _ => ()
        }
        // println!("{}", grid);
        // println!("head: {:?}, tail: {:?}", head_loc, tail_loc);
        // println!("");
    }


    let visited = &grid.iter().filter(|c| **c > 0).count();

    println!("{}", visited);

    return Ok(());
}

/// Function moves the head and tail right a number of positions.
fn move_right(
    grid: &mut ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>,
    num_moves: i32,
    head_loc: &mut Coordinate,
    tail_loc: &mut Coordinate
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
        // move head position 1 to the right
        head_loc.x = head_loc.x + 1;

        // if they're both on same, y, we can just move to the right
        // we also have to add 1 to the location for future tracking

        if head_loc.y == tail_loc.y && head_loc.x == tail_loc.x {
            continue;
        }

        if head_loc.y == tail_loc.y {
            grid.row_mut(head_loc.y)[head_loc.x - 1] += 1;
            tail_loc.x = head_loc.x - 1;
            continue;
        } 

        if &head_loc.x - &tail_loc.x > 1 {
            grid.row_mut(head_loc.y)[head_loc.x - 1] += 1;
            tail_loc.x = head_loc.x - 1;
            tail_loc.y = head_loc.y;
            continue;
        }
    }
}

fn move_left(
    grid: &mut ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>,
    num_moves: i32,
    head_loc: &mut Coordinate,
    tail_loc: &mut Coordinate
) {
    // First loop through each number of moves (will loop 5 times if num_moves = 5)
    for _n in 0..num_moves {
        // Determine current length of the y axis
        let len_y = grid.len_of(Axis(0));

        // if the cur point is equal to length of X axis, then we're
        // at the START and need to PREPEND a new column
        if head_loc.x == 0 {
            // need to create a new grid then assign to existing after
            let mut grid_new = Array2::<i32>::zeros((len_y, 1));
            
            for col in grid.columns() {
                // now we can add this new column to the matrix
                grid_new.push_column(ArrayView::from(&col)).unwrap();
            }
            *grid = grid_new;
            head_loc.x = 0;
            // we need to add to the tail location because we expanded the grid to the left
            tail_loc.x += 1;
        } else {
            // move head position 1 to the left
            head_loc.x -= 1;
        } 

        // if they're both on same, y, we can just move to the left
        // we also have to add 1 to the location for future tracking
        if head_loc.y == tail_loc.y && head_loc.x == tail_loc.x {
            continue;
        }

        if head_loc.y == tail_loc.y {
            grid.row_mut(head_loc.y)[head_loc.x + 1] += 1;
            tail_loc.x = head_loc.x + 1;
            continue;
        }

        if head_loc.x.abs_diff(tail_loc.x) > 1 {
            grid.row_mut(head_loc.y)[head_loc.x + 1] += 1;
            tail_loc.x = head_loc.x + 1;
            tail_loc.y = head_loc.y;
            continue;
        }
    }
}

fn move_up(
    grid: &mut ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>,
    num_moves: i32,
    head_loc: &mut Coordinate,
    tail_loc: &mut Coordinate
) {
    // First loop through each number of moves (will loop 5 times if num_moves = 5)
    for _n in 0..num_moves {
        // Determine the current length of the x axis
        let len_x = grid.len_of(Axis(1));
        // Determine current length of the y axis
        let len_y = grid.len_of(Axis(0));

        // check to see if were at upper most row
        if head_loc.y == 0 {
            // need to create a new grid then assign to existing after
            let mut grid_new = Array2::<i32>::zeros((1, len_x));
            
            for row in grid.rows() {
                // now we can add this new column to the matrix
                grid_new.push_row(ArrayView::from(&row)).unwrap();
            }
            *grid = grid_new;
            head_loc.y = 0;
            // we need to add to the tail location because we expanded the grid to the left
            tail_loc.y += 1;
        } else {
            // move head position 1 position upwards
            head_loc.y -= 1;
        } 

        // if they're both on same, x, we can just move up
        // we also have to add 1 to the location for future tracking
        if head_loc.y == tail_loc.y && head_loc.x == tail_loc.x {
            continue;
        }

        if head_loc.x == tail_loc.x {
            grid.row_mut(head_loc.y + 1)[head_loc.x] += 1;
            tail_loc.y = head_loc.y + 1;
            continue;
        }

        if head_loc.y.abs_diff(tail_loc.y) > 1 {
            grid.row_mut(head_loc.y + 1)[head_loc.x] += 1;
            tail_loc.y = head_loc.y + 1;
            tail_loc.x = head_loc.x;
            continue;
        }
    }
}

fn move_down(
    grid: &mut ArrayBase<OwnedRepr<i32>, Dim<[usize; 2]>>,
    num_moves: i32,
    head_loc: &mut Coordinate,
    tail_loc: &mut Coordinate
) {
    // First loop through each number of moves (will loop 5 times if num_moves = 5)
    for _n in 0..num_moves {
        // Determine the current length of the x axis
        let len_x = grid.len_of(Axis(1));
        // Determine current length of the y axis
        let len_y = grid.len_of(Axis(0));

        // if the cur point is equal to length of X axis, then we're
        // at the END and need to add a new column
        if head_loc.y == len_y - 1 {
            // This creates a new row shaped like [0,0,0,0,0] if len_x = 5 for example
            // This is required because the new column needs to match the current shape
            // of the existing matrix. Otherwise it throws an error
            let row = iter::repeat(0).take(len_x).collect::<Vec<i32>>();
            // now we can add this new column to the matrix
            grid.push_row(ArrayView::from(&row)).unwrap();
        } 

        // move head down by 1
        head_loc.y = head_loc.y + 1;

        // if they're both on same, y, we can just move down
        // we also have to add 1 to the location for future tracking
        if head_loc.y == tail_loc.y && head_loc.x == tail_loc.x {
            continue;
        }

        if head_loc.x == tail_loc.x {
            grid.row_mut(head_loc.y - 1)[head_loc.x] += 1;
            tail_loc.y = head_loc.y - 1;
            continue;
        }

        if &head_loc.y - &tail_loc.y > 1 {
            grid.row_mut(head_loc.y - 1)[head_loc.x] += 1;
            tail_loc.y = head_loc.y - 1;
            tail_loc.x = head_loc.x;
            continue;
        }
    }

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


#[test]
fn move_left_simple_test() {
    // arrange
    let mut grid = arr2(&[[0]]);
    let expected_grid = arr2(&[[0, 1, 1, 1, 1]]);

    let mut head_loc = Coordinate { x: 0, y: 0 };
    let exp_head = Coordinate { x: 0, y: 0 };

    let mut tail_loc = Coordinate { x: 0, y: 0 };
    let exp_tail = Coordinate { x: 1, y: 0 };

    // act
    move_left(&mut grid, 4, &mut head_loc, &mut tail_loc);

    // assert
    assert_eq!(grid, expected_grid);
    assert_eq!(head_loc.x, exp_head.x);
    assert_eq!(head_loc.y, exp_head.y);
    assert_eq!(tail_loc.x, exp_tail.x);
    assert_eq!(tail_loc.y, exp_tail.y);
}

#[test]
fn move_left_diag_move_test() {
    // arrange
    let mut grid = arr2(&[
        [0,0], 
        [0,0], 
        [1,1]
    ]);
    let expected_grid = arr2(&[
        [0,0,0], 
        [0,1,0], 
        [0,1,1]
    ]);

    let mut head_loc = Coordinate { x: 0, y: 1 };
    let exp_head = Coordinate { x: 0, y: 1 };

    let mut tail_loc = Coordinate { x: 1, y: 2 };
    let exp_tail = Coordinate { x: 1, y: 1 };

    // act
    move_left(&mut grid, 1, &mut head_loc, &mut tail_loc);

    // assert
    assert_eq!(grid, expected_grid);
    assert_eq!(head_loc.x, exp_head.x);
    assert_eq!(head_loc.y, exp_head.y);
    assert_eq!(tail_loc.x, exp_tail.x);
    assert_eq!(tail_loc.y, exp_tail.y);
}

#[test]
fn move_left_diag_move_noexpansion_test() {
    // arrange
    let mut grid = arr2(&[
        [0,0,0], 
        [0,0,0], 
        [0,1,1]
    ]);
    let expected_grid = arr2(&[
        [0,0,0], 
        [0,1,0], 
        [0,1,1]
    ]);

    let mut head_loc = Coordinate { x: 1, y: 1 };
    let exp_head = Coordinate { x: 0, y: 1 };

    let mut tail_loc = Coordinate { x: 2, y: 2 };
    let exp_tail = Coordinate { x: 1, y: 1 };

    // act
    move_left(&mut grid, 1, &mut head_loc, &mut tail_loc);

    // assert
    assert_eq!(grid, expected_grid);
    assert_eq!(head_loc.x, exp_head.x);
    assert_eq!(head_loc.y, exp_head.y);
    assert_eq!(tail_loc.x, exp_tail.x);
    assert_eq!(tail_loc.y, exp_tail.y);
}

#[test]
fn move_left_diag_stay_test() {
    // arrange
    let mut grid = arr2(&[
        [0,0], 
        [0,0], 
        [1,0]
    ]);
    let expected_grid = arr2(&[
        [0,0], 
        [0,0], 
        [1,0]
    ]);

    let mut head_loc = Coordinate { x: 1, y: 1 };
    let exp_head = Coordinate { x: 0, y: 1 };

    let mut tail_loc = Coordinate { x: 0, y: 2 };
    let exp_tail = Coordinate { x: 0, y: 2 };

    // act
    move_left(&mut grid, 1, &mut head_loc, &mut tail_loc);

    // assert
    assert_eq!(grid, expected_grid);
    assert_eq!(head_loc.x, exp_head.x);
    assert_eq!(head_loc.y, exp_head.y);
    assert_eq!(tail_loc.x, exp_tail.x);
    assert_eq!(tail_loc.y, exp_tail.y);
}

#[test]
fn move_right_simple_test() {
    // arrange
    let mut grid = arr2(&[[0]]);
    let expected_grid = arr2(&[[1, 1, 1, 1, 0]]);

    let mut head_loc = Coordinate { x: 0, y: 0 };
    let exp_head = Coordinate { x: 4, y: 0 };

    let mut tail_loc = Coordinate { x: 0, y: 0 };
    let exp_tail = Coordinate { x: 3, y: 0 };

    // act
    move_right(&mut grid, 4, &mut head_loc, &mut tail_loc);

    // assert
    assert_eq!(grid, expected_grid);
    assert_eq!(head_loc.x, exp_head.x);
    assert_eq!(head_loc.y, exp_head.y);
    assert_eq!(tail_loc.x, exp_tail.x);
    assert_eq!(tail_loc.y, exp_tail.y);
}

#[test]
fn move_right_diag_move_test() {
    // arrange
    let mut grid = arr2(&[
        [0,0], 
        [0,0], 
        [1,0]
    ]);
    let expected_grid = arr2(&[
        [0,0,0], 
        [0,1,0], 
        [1,0,0]
    ]);

    let mut head_loc = Coordinate { x: 1, y: 1 };
    let exp_head = Coordinate { x: 2, y: 1 };

    let mut tail_loc = Coordinate { x: 0, y: 2 };
    let exp_tail = Coordinate { x: 1, y: 1 };

    // act
    move_right(&mut grid, 1, &mut head_loc, &mut tail_loc);

    // assert
    assert_eq!(grid, expected_grid);
    assert_eq!(head_loc.x, exp_head.x);
    assert_eq!(head_loc.y, exp_head.y);
    assert_eq!(tail_loc.x, exp_tail.x);
    assert_eq!(tail_loc.y, exp_tail.y);
}

#[test]
fn move_right_diag_stay_test() {
    // arrange
    let mut grid = arr2(&[
        [0,0], 
        [0,0], 
        [1,0]
    ]);
    let expected_grid = arr2(&[
        [0,0], 
        [0,0], 
        [1,0]
    ]);

    let mut head_loc = Coordinate { x: 0, y: 1 };
    let exp_head = Coordinate { x: 1, y: 1 };

    let mut tail_loc = Coordinate { x: 0, y: 2 };
    let exp_tail = Coordinate { x: 0, y: 2 };

    // act
    move_right(&mut grid, 1, &mut head_loc, &mut tail_loc);

    // assert
    assert_eq!(grid, expected_grid);
    assert_eq!(head_loc.x, exp_head.x);
    assert_eq!(head_loc.y, exp_head.y);
    assert_eq!(tail_loc.x, exp_tail.x);
    assert_eq!(tail_loc.y, exp_tail.y);
}

#[test]
fn move_up_simple_test() {
    // arrange
    let mut grid = arr2(&[[0]]);
    let expected_grid = arr2(&[[0],[1],[1],[1],[1]]);

    let mut head_loc = Coordinate { x: 0, y: 0 };
    let exp_head = Coordinate { x: 0, y: 0 };

    let mut tail_loc = Coordinate { x: 0, y: 0 };
    let exp_tail = Coordinate { x: 0, y: 1 };

    // act
    move_up(&mut grid, 4, &mut head_loc, &mut tail_loc);

    // assert
    assert_eq!(grid, expected_grid);
    assert_eq!(head_loc.x, exp_head.x);
    assert_eq!(head_loc.y, exp_head.y);
    assert_eq!(tail_loc.x, exp_tail.x);
    assert_eq!(tail_loc.y, exp_tail.y);
}

#[test]
fn move_up_diag_move_test() {
    // arrange
    let mut grid = arr2(&[
        [0,0], 
        [1,1]
    ]);
    let expected_grid = arr2(&[
        [0,0], 
        [1,0], 
        [1,1]
    ]);

    let mut head_loc = Coordinate { x: 0, y: 0 };
    let exp_head = Coordinate { x: 0, y: 0 };

    let mut tail_loc = Coordinate { x: 1, y: 1 };
    let exp_tail = Coordinate { x: 0, y: 1 };

    // act
    move_up(&mut grid, 1, &mut head_loc, &mut tail_loc);

    // assert
    assert_eq!(grid, expected_grid);
    assert_eq!(head_loc.x, exp_head.x);
    assert_eq!(head_loc.y, exp_head.y);
    assert_eq!(tail_loc.x, exp_tail.x);
    assert_eq!(tail_loc.y, exp_tail.y);
}

#[test]
fn move_up_diag_move_noexpansion_test() {
    // arrange
    let mut grid = arr2(&[
        [0,0], 
        [0,0], 
        [1,1]
    ]);
    let expected_grid = arr2(&[
        [0,0], 
        [1,0], 
        [1,1]
    ]);

    let mut head_loc = Coordinate { x: 0, y: 1 };
    let exp_head = Coordinate { x: 0, y: 0 };

    let mut tail_loc = Coordinate { x: 1, y: 2 };
    let exp_tail = Coordinate { x: 0, y: 1 };

    // act
    move_up(&mut grid, 1, &mut head_loc, &mut tail_loc);

    // assert
    assert_eq!(grid, expected_grid);
    assert_eq!(head_loc.x, exp_head.x);
    assert_eq!(head_loc.y, exp_head.y);
    assert_eq!(tail_loc.x, exp_tail.x);
    assert_eq!(tail_loc.y, exp_tail.y);
}

#[test]
fn move_up_diag_stay_test() {
    // arrange
    let mut grid = arr2(&[
        [0,0], 
        [0,0], 
        [1,0]
    ]);
    let expected_grid = arr2(&[
        [0,0], 
        [0,0], 
        [1,0]
    ]);

    let mut head_loc = Coordinate { x: 1, y: 2 };
    let exp_head = Coordinate { x: 1, y: 1 };

    let mut tail_loc = Coordinate { x: 0, y: 2 };
    let exp_tail = Coordinate { x: 0, y: 2 };

    // act
    move_up(&mut grid, 1, &mut head_loc, &mut tail_loc);

    // assert
    assert_eq!(grid, expected_grid);
    assert_eq!(head_loc.x, exp_head.x);
    assert_eq!(head_loc.y, exp_head.y);
    assert_eq!(tail_loc.x, exp_tail.x);
    assert_eq!(tail_loc.y, exp_tail.y);
}

#[test]
fn move_down_simple_test() {
    // arrange
    let mut grid = arr2(&[[0]]);
    let expected_grid = arr2(&[[1],[1],[1],[1],[0]]);

    let mut head_loc = Coordinate { x: 0, y: 0 };
    let exp_head = Coordinate { x: 0, y: 4 };

    let mut tail_loc = Coordinate { x: 0, y: 0 };
    let exp_tail = Coordinate { x: 0, y: 3 };

    // act
    move_down(&mut grid, 4, &mut head_loc, &mut tail_loc);

    // assert
    assert_eq!(grid, expected_grid);
    assert_eq!(head_loc.x, exp_head.x);
    assert_eq!(head_loc.y, exp_head.y);
    assert_eq!(tail_loc.x, exp_tail.x);
    assert_eq!(tail_loc.y, exp_tail.y);
}

#[test]
fn move_down_diag_move_test() {
    // arrange
    let mut grid = arr2(&[
        [1,0], 
        [1,0]
    ]);
    let expected_grid = arr2(&[
        [1,0], 
        [1,1], 
        [0,0]
    ]);

    let mut head_loc = Coordinate { x: 1, y: 1 };
    let exp_head = Coordinate { x: 1, y: 2 };

    let mut tail_loc = Coordinate { x: 0, y: 0 };
    let exp_tail = Coordinate { x: 1, y: 1 };

    // act
    move_down(&mut grid, 1, &mut head_loc, &mut tail_loc);

    // assert
    assert_eq!(grid, expected_grid);
    assert_eq!(head_loc.x, exp_head.x);
    assert_eq!(head_loc.y, exp_head.y);
    assert_eq!(tail_loc.x, exp_tail.x);
    assert_eq!(tail_loc.y, exp_tail.y);
}

#[test]
fn move_down_diag_move_noexpansion_test() {
    // arrange
    let mut grid = arr2(&[
        [1,0], 
        [1,0], 
        [0,0]
    ]);
    let expected_grid = arr2(&[
        [1,0], 
        [1,1], 
        [0,0]
    ]);

    let mut head_loc = Coordinate { x: 1, y: 1 };
    let exp_head = Coordinate { x: 1, y: 2 };

    let mut tail_loc = Coordinate { x: 0, y: 0 };
    let exp_tail = Coordinate { x: 1, y: 1 };

    // act
    move_down(&mut grid, 1, &mut head_loc, &mut tail_loc);

    // assert
    assert_eq!(grid, expected_grid);
    assert_eq!(head_loc.x, exp_head.x);
    assert_eq!(head_loc.y, exp_head.y);
    assert_eq!(tail_loc.x, exp_tail.x);
    assert_eq!(tail_loc.y, exp_tail.y);
}

#[test]
fn move_down_diag_stay_test() {
    // arrange
    let mut grid = arr2(&[
        [0,0], 
        [0,0], 
        [1,0]
    ]);
    let expected_grid = arr2(&[
        [0,0], 
        [0,0], 
        [1,0]
    ]);

    let mut head_loc = Coordinate { x: 1, y: 1 };
    let exp_head = Coordinate { x: 1, y: 2 };

    let mut tail_loc = Coordinate { x: 0, y: 1 };
    let exp_tail = Coordinate { x: 0, y: 1 };

    // act
    move_down(&mut grid, 1, &mut head_loc, &mut tail_loc);

    // assert
    assert_eq!(grid, expected_grid);
    assert_eq!(head_loc.x, exp_head.x);
    assert_eq!(head_loc.y, exp_head.y);
    assert_eq!(tail_loc.x, exp_tail.x);
    assert_eq!(tail_loc.y, exp_tail.y);
}
