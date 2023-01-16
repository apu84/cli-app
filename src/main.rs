use std::io::{stdin};

#[allow(dead_code)]
fn fibonacci(n: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;
    let mut sum = 0;

    if n == 1 {
       return first
    }
    else if n == 2 {
       return second
    }

    for _ in 2..n {
        sum = first + second;
        first = second;
        second = sum;
    }
    sum
}
#[allow(dead_code)]
fn prompt_for_number() {
    println!("Enter a number: ");
    let mut num_s = String::new();
    stdin().read_line(&mut num_s)
        .expect("Failed to read from stdin");

    match num_s.trim().parse::<i32>() {
        Ok(num) => {
            if num > 47  {
                println!("Too large, enter a number smaller than 48");
                prompt_for_number();
            }
            else if num <= 0 {
                println!("Too small, enter a number bigger than 0");
                prompt_for_number()
            }
            println!("entered {} and {}th fibonacci number is {}", num, num, fibonacci(num))
        },
        Err(..) => println!("Entered number was not an Integer")
    }
    prompt_for_number();
}

fn main() {
    // println!("Calculate Fibonacci. Enter Ctrl-c to exit");
    // prompt_for_number()
    setup_board();
}

enum Direction {
    DiagonalLeft,
    DiagonalRight,
    Right,
    Down
}
struct TraverseMatch {
    direction: Direction,
    matched: i32
}

#[allow(dead_code)]
fn setup_board() {
    let board:Vec<Vec<i32>> = vec![vec![1,0,1], vec![0,1,0], vec![1,1,1]];

    for (i , row) in board.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            
            let mut tr_down = TraverseMatch {
                direction: Direction::Down,
                matched: 0
            };

            let mut tr_right = TraverseMatch {
                direction: Direction::Right,
                matched: 0
            };
            let mut tr_diagonal_right = TraverseMatch {
                direction: Direction::DiagonalRight,
                matched: 0
            };
            let mut tr_diagonal_left = TraverseMatch {
                direction: Direction::DiagonalLeft,
                matched: 0
            };

            if traverse(i, j, &board, &mut tr_down) {
                println!("↓ [{}, {}] => {:?}", i, j, col);
            }

            if traverse(i, j, &board, &mut tr_right) {
                println!("→ [{}, {}] => {:?}", i, j, col);
            }

            if traverse(i, j, &board, &mut tr_diagonal_right) {
                println!("↘ [{}, {}] => {:?}", i, j, col);
            }

            if traverse(i, j, &board, &mut tr_diagonal_left) {
                println!("↙ [{}, {}] => {:?}", i, j, col);
            }
        }
    }
}

fn traverse(x: usize, y: usize, board: &Vec<Vec<i32>>, traverse_match: &mut TraverseMatch) -> bool {
    let one:i32 = 1;
    
    if x > 2 || y > 2 {
        return false;
    }

    if board.get(x).unwrap().get(y).unwrap() == &one {
        // println!("({}, {})", x, y);
        traverse_match.matched = traverse_match.matched + 1;
        if traverse_match.matched == 3 {
            return true;
        }
        let mut _x = x; 
        let mut _y = y;

        match traverse_match.direction {
            Direction::DiagonalRight => {
                _x += 1;
                _y += 1;
            }, 
            Direction::DiagonalLeft => {
                _x += 1;
                match _y.checked_sub(1) {
                    Some(n) => {
                        _y = n;
                    },
                    None => {
                        return false;
                    }
                }
            },
            Direction::Down => {
                _x += 1;    
            },
            Direction::Right => {
                _y += 1
            }  
        }
        return traverse(_x, _y, board, traverse_match);
    }

    false
}