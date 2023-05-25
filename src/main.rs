use std::io::{stdin, stdout, Write};
use rand::Rng;
use std::collections::{ HashSet, VecDeque };

/**
 * TODO: 
 *  - place a flag
 *  - Run a full game
 *      - Check for death state
 *      - Check for success state
 *  - Build the UI
 * 
 *  Could be nice:
 *      - Hide the mine locations in memory some how? (Could be way harder than necessary for this scope)
 */

fn main() {
    let (mut n, mut m) = (0u8,0u8);

    get_field_size(&mut n, &mut m);
    let mut field: Vec<Vec<i8>> = vec![vec![0;n as usize];m as usize];
    let mut user_visible_field: Vec<Vec<bool>> = vec![vec![false;n as usize]; m as usize];
    let mut placed_flags: Vec<Vec<bool>> = vec![vec![false;n as usize]; m as usize];
    let no_mines = ask_user_mines_no(n, m);
    let mut guessed_mines: u8 = 0;
    let mut game_running:bool = true;

    print_opening(n, m);
    let (mut y, mut x) = ask_user_selection(n, m);
    generate_mines(&mut field, n, m, no_mines, x, y);
    let _ = handle_user_guess(x as usize, y as usize, &field, &mut user_visible_field);
    print_field(&field, &user_visible_field, &placed_flags);

    while game_running { //or while guessed_mines < no_mines? (Need to check for valid mines)
        (y,x) = ask_user_selection(n, m);
        print_field(&field, &user_visible_field, &placed_flags);
        
        let result = handle_open_or_flag(x, y, &field, &mut user_visible_field, 
                                                    &mut placed_flags, &mut guessed_mines);
        print!("Guess state was a: ");
        match result {
            GuessState::Exploded => { println!("Death!"); game_running = false; },
            GuessState::AlreadySeen => println!("Retry!"),
            GuessState::Success => println!("Safe!")
        };
        print_field(&field, &user_visible_field, &placed_flags);
    }
}

/**
 * Asks user for placing a flag or revealing the spot
 * Flag -> Places or removes a flag, increments or decrements mine count
 * Reveal -> Reveals the space on the field
 * 
 * Returns success of the guess, flag always returns success
 * 
 * ASSUMES VALID COORDINATES
 */
fn handle_open_or_flag(x: u8, y:u8, field: &Vec<Vec<i8>>, seen: &mut Vec<Vec<bool>>, flags: &mut Vec<Vec<bool>>, guessed_mines: &mut u8) -> GuessState {
    print!("Reveal (r) or flag (f): ");
    let _ = stdout().flush();
    let mut input_buffer = String::new();

    loop {
        input_buffer.clear();
        stdin().read_line(&mut input_buffer).expect("Failed to read line.");
        match &*input_buffer.trim_end().to_lowercase() {
            "reveal" => return handle_user_guess(x as usize, y as usize, field, seen),
            "r" => return handle_user_guess(x as usize, y as usize, field, seen),
            "flag" => return handle_place_flag(x as usize, y as usize, flags, guessed_mines),
            "f" => return handle_place_flag(x as usize, y as usize, flags, guessed_mines),
            _ => { println!("Please respond reveal (r) or flag (f)."); continue; } 
        };
    }
}

/**
 * Sets the bool of the flags vector to true
 * increments or decrements mine count
 * Returns success 
 */
fn handle_place_flag(x: usize, y:usize, flags: &mut Vec<Vec<bool>>, guessed_mines: &mut u8) -> GuessState{
    if flags[x][y] {
        *guessed_mines -= 1;
        flags[x][y] = false;
    }else {
        *guessed_mines += 1;
        flags[x][y] = true;
    }
    return GuessState::Success
}

/**
* Checks the space of the users reveal guess.
* Has it already been revealed?
* Is it a mine?
*/
fn handle_user_guess(x:usize,y:usize, field: &Vec<Vec<i8>>, seen: &mut Vec<Vec<bool>>) -> GuessState {
    //Check spot:
    if field[x][y] == -1{
        seen[x][y] = true;
        return GuessState::Exploded;
    }else if seen[x][y]{
        return GuessState::AlreadySeen;
    }

    seen[x][y] = true;
    reveal_neighbours(x as i32, y as i32, field, seen);
    GuessState::Success
}

enum GuessState {
    Exploded,
    AlreadySeen,
    Success
}

/**
* Revealing:
*  - Check spot as visible(seen)
*  - neighbour is 0, reveal that spot as well
*/
fn reveal_neighbours(x:i32, y:i32, field: &Vec<Vec<i8>>, seen: &mut Vec<Vec<bool>>) {
    let mut deq = VecDeque::from([(x,y)]); // put current spot in queue
    let mut explored: HashSet<(i32,i32)> = HashSet::new();

    // check each neighbour if it is 0
    while !deq.is_empty() {
        let (x, y) = deq.pop_front().expect("Error popping from deq.");
        let neighbours = get_neighbours(x as usize, y as usize, field);
        explored.insert((x,y));

        for (i, spot) in neighbours.iter().enumerate() {
            if neighbours[i] == -2 || neighbours[i] == -1 { continue; } //ignore out of bounds and mines

            let (diff_x, diff_y) = translate_to_grid(i as i32);
            let curr_spot = ((x + diff_x), (y+diff_y));

            if *spot == 0 && !explored.contains(&curr_spot) { deq.push_back(curr_spot); }
            seen[curr_spot.0 as usize][curr_spot.1 as usize] = true; // mark as visible
        }
    }
}

fn translate_to_grid(i:i32) -> (i32, i32) {
    match i {
        0 => (-1, -1),
        1 => (-1, 0),
        2 => (-1, 1),
        3 => (0, -1),
        4 => (0, 1),
        5 => (1, -1),
        6 => (1, 0),
        7 => (1, 1),
        _ => panic!()
    }
}

/**
 * Get the neighbouring spots of a given point
 * Out of bounds are set to -2
 * 
 * Returns array structured as: [NW, N, NE, E, W, SW, S, SE]
 */
fn get_neighbours(x:usize, y:usize, field: &Vec<Vec<i8>>) -> [i8;8] {
    let mut neighbours: [i8;8] = [0;8];

    //top row
    neighbours[0] = if x == 0 || y == 0 { -2 } else { field[x-1][y-1] };
    neighbours[1] = if x == 0 { -2 } else { field[x-1][y] };
    neighbours[2] = if x == 0 || y + 1 >= field[0].len() { -2 } else { field[x-1][y+1] };
    
    //same row
    neighbours[3] = if y == 0 { -2 } else { field[x][y-1] };
    neighbours[4] = if y + 1 >= field[0].len() { -2 } else { field[x][y+1] };

    //bottom row
    neighbours[5] = if x + 1 >= field.len() || y == 0 { -2 } else { field[x+1][y-1] };
    neighbours[6] = if x + 1 >= field.len() { -2 } else { field[x+1][y] };
    neighbours[7] = if x + 1 >= field.len() || y + 1 >= field[0].len() { -2 } else { field[x+1][y+1] };

    return neighbours;
}

/**
 * Place requested mines on the field, updating the neighbours as we go
 * 
 * Params: 
 *  - field: The mine field to place the mines in (set to -1)
 *  - n: the length of the field
 *  - m: the width of the field
 *  - no_minds: # of minds to generate
 *  - x: X coord of the starting position
 *  - y: Y coord of the starting position
 */
fn generate_mines(field: &mut Vec<Vec<i8>>, n: u8, m: u8, no_mines: u8, x_user:u8, y_user:u8) {
    let mut set_mines: HashSet<(u8,u8)> = HashSet::new();
    let mut curr_mines = 0;
    let mut rng = rand::thread_rng();

    while curr_mines < no_mines {
        //Generate 2 random numbers
        let y = rng.gen_range(0..n);
        let x = rng.gen_range(0..m);
        if x == x_user && y == y_user { continue; }

        //Check if the spot is already a mine
        if set_mines.insert((x,y)) {
            curr_mines += 1;
            field[x as usize][y as usize] = -1;
            update_neighbours(field, m, n, &(x,y))
        }else { continue; /*try again*/ } 
    }

    debug_print_field_raw(&field);
}

/**
 * Increment all the neighbours of a new mine by one
 */
fn update_neighbours(field: &mut Vec<Vec<i8>>, m: u8, n: u8, point: &(u8,u8)) {
    if (point.0 > 0 && point.1 > 0) && field[(point.0-1) as usize][(point.1-1) as usize] != -1 
        { field[(point.0-1) as usize][(point.1-1) as usize] += 1; }                         /* NW */
    if point.0 > 0 && field[(point.0-1) as usize][point.1 as usize] != -1 
        { field[(point.0-1) as usize][point.1 as usize] += 1; }                             /* N */
    if (point.0 > 0 && point.1+1 < n) && field[(point.0-1) as usize][(point.1+1) as usize] != -1
        { field[(point.0-1) as usize][(point.1+1) as usize] += 1; }                         /* NE */
    if point.1 > 0 && field[point.0 as usize][(point.1-1) as usize] != -1
        { field[point.0 as usize][(point.1-1) as usize] += 1; }                             /* W */ 
    if point.1+1 < n && field[point.0 as usize][(point.1+1) as usize] != -1
        { field[point.0 as usize][(point.1+1) as usize] += 1; }                             /* E */
    if (point.0+1 < m && point.1 > 0) && field[(point.0+1) as usize][(point.1-1) as usize] != -1
        { field[(point.0+1) as usize][(point.1-1) as usize] += 1; }                         /* SW */
    if point.0+1 < m && field[(point.0+1) as usize][point.1 as usize] != -1
        { field[(point.0+1) as usize][point.1 as usize] += 1;}                              /* S */
    if (point.0+1 < m && point.1+1 < n) && field[(point.0+1) as usize][(point.1+1) as usize] != -1
        { field[(point.0+1) as usize][(point.1+1) as usize] += 1; }                         /* SE */
}

fn ask_user_mines_no(n:u8, m:u8) -> u8 {
    let mut mines = 0;

    loop {
        print!("Enter number of desired mines (min 1): ");
        let _ = stdout().flush();
        read_u8(&mut mines);

        if mines != 0 && mines <= (n*m)-1 { break; }
        println!("Invalid number of mines! Try again.");
        let _ = stdout().flush();
    }

    return mines;
}

/**
 * Get the x and y coordinates of the user's selection
 * 
 * Checks if the selection is out of bounds
 */
fn ask_user_selection(n:u8, m:u8) -> (u8, u8) {
    let (mut x, mut y) = (0u8, 0u8);

    loop {
        print!("Enter the x coordinate: ");
        let _ = stdout().flush();
        
        read_u8(&mut x);
        if x < n { break; }

        print!("Out of bounds! Try again: ");
        let _ = stdout().flush();
    }

    loop {
        print!("Enter the y coordinate: ");
        let _ = stdout().flush();
        
        read_u8(&mut y);
        if y < m { break; }

        print!("Out of bounds! Try again: ");
        let _ = stdout().flush();
    }

    return (x,y);
}

fn print_opening(n:u8, m:u8) {
    for _ in 0..n {
        for _ in 0..m {
            print!("? ");
        }
        println!();
    }
}

fn print_field(field: &Vec<Vec<i8>>, seen: &Vec<Vec<bool>>, flags: &Vec<Vec<bool>>) {
    for (i, row) in field.iter().enumerate() {
        for (j, colmn) in row.iter().enumerate() {
            if seen[i][j] {
                if *colmn == -1 { print!("X "); } //Maybe don't need this?
                else { print!("{} ", colmn); }
            } else if flags[i][j] {
                print!("# ");
            } else {
                print!("? ");
            }
        }
        println!();
    }
    println!("------------------------------");
}

fn debug_print_field_raw(field: &Vec<Vec<i8>>) {
    println!("*********DEBUG PRINT*********");
    for (_i, row) in field.iter().enumerate() {
        for (_j, colmn) in row.iter().enumerate() {
            print!("{} ", colmn);
        }
        println!();
    }
    println!("*********DEBUG PRINT*********");
}

/**
 * Read an int from stdin and return the value
 * Forces a retry on parsing, but no other checks
 */
fn read_u8(x: &mut u8) {
    let mut input_buffer = String::new();

    loop {
        input_buffer.clear();
        stdin().read_line(&mut input_buffer).expect("Failed to read line.");
        *x = match input_buffer.trim_end().parse::<u8>() {
            Ok(num) => num,
            Err(err) => { eprintln!("Error parsing number {input_buffer}: {err}, try again."); continue; }
        };
        break;
    }
}

fn get_field_size(n:&mut u8, m:&mut u8) {
    print!("Enter the height of the field (min 3): ");
    let _ = stdout().flush();
    loop { 
        read_u8(n); 
        if *n != 0 && *n >= 3 { break; }
        print!("Invalid number, please try again: "); 
        let _ = stdout().flush();
    }

    print!("Enter the width of the field (min 3): ");
    let _ = stdout().flush();
    loop { 
        read_u8(m); 
        if *m != 0 && *m >= 3 { break; }
        print!("Invalid number, please try again: ");
        let _ = stdout().flush();
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use crate::{get_neighbours, reveal_neighbours};

    //REVEAL NEIGHBOURS TESTS
    #[test]
    fn reveal_neighbours_basic_test() {
        let dummy_field: Vec<Vec<i8>> = vec![vec![1,2,3], vec![4,0,5], vec![6,7,8]];
        let mut dummy_seen: Vec<Vec<bool>> = vec![vec![false,false,false], 
                                            vec![false,false,false], 
                                            vec![false,false,false]];
        let expected: Vec<Vec<bool>> = vec![vec![true,true,true], 
                                            vec![true,false,true], 
                                            vec![true,true,true]];
        reveal_neighbours(1, 1, &dummy_field, &mut dummy_seen);
        assert_eq!(dummy_seen, expected);
    }
    #[test]
    fn reveal_neighbours_cascade_test() {
        let dummy_field: Vec<Vec<i8>> = vec![vec![1,1,1,1,1], 
                                             vec![1,1,0,1,1], 
                                             vec![1,0,1,0,1],
                                             vec![1,1,0,1,1],
                                             vec![1,1,1,1,1]];
        let mut dummy_seen: Vec<Vec<bool>> = vec![vec![false,false,false,false, false], 
                                                  vec![false,false,false,false, false], 
                                                  vec![false,false,false,false, false],
                                                  vec![false,false,false,false, false],
                                                  vec![false,false,false,false, false]];
        let expected: Vec<Vec<bool>> = vec![vec![false,true,true,true,false], 
                                            vec![true,true,true,true,true], 
                                            vec![true,true,true,true,true],
                                            vec![true,true,true,true,true],
                                            vec![false,true,true,true,false]];

        reveal_neighbours(2, 2, &dummy_field, &mut dummy_seen);
        assert_eq!(dummy_seen, expected);
    }
    
    #[test]
    fn reveal_neighbours_cascade_inverse_test() {
        let dummy_field: Vec<Vec<i8>> = vec![vec![1,1,1,1,1], 
                                             vec![1,1,1,1,1], 
                                             vec![1,1,1,1,1],
                                             vec![1,1,1,1,1],
                                             vec![1,1,1,1,1]];
        let mut dummy_seen: Vec<Vec<bool>> = vec![vec![false,false,false,false, false], 
                                                  vec![false,false,false,false, false], 
                                                  vec![false,false,false,false, false],
                                                  vec![false,false,false,false, false],
                                                  vec![false,false,false,false, false]];
        let expected: Vec<Vec<bool>> = vec![vec![false,false,false,false,false], 
                                            vec![false,true,true,true,false], 
                                            vec![false,true,false,true,false],
                                            vec![false,true,true,true,false],
                                            vec![false,false,false,false,false]];

        reveal_neighbours(2, 2, &dummy_field, &mut dummy_seen);
        assert_eq!(dummy_seen, expected);
    }

    #[test]
    fn reveal_neighbours_hide_mines_basic_test() {
        let dummy_field: Vec<Vec<i8>> = vec![vec![1,1,1,1,1], 
                                             vec![1,-1,-1,-1,1], 
                                             vec![1,-1,1,-1,1],
                                             vec![1,-1,-1,-1,1],
                                             vec![1,1,1,1,1]];
        let mut dummy_seen: Vec<Vec<bool>> = vec![vec![false,false,false,false, false], 
                                                  vec![false,false,false,false, false], 
                                                  vec![false,false,true,false, false],
                                                  vec![false,false,false,false, false],
                                                  vec![false,false,false,false, false]];
        let expected: Vec<Vec<bool>> = vec![vec![false,false,false,false,false], 
                                            vec![false,false,false,false,false], 
                                            vec![false,false,true,false,false],
                                            vec![false,false,false,false,false],
                                            vec![false,false,false,false,false]];
        reveal_neighbours(2, 2, &dummy_field, &mut dummy_seen);
        assert_eq!(dummy_seen, expected);

    }

    #[test]
    fn reveal_neighbours_hidemines_cascade_test() {
        let dummy_field: Vec<Vec<i8>> = vec![vec![1,1,-1,1,1], 
                                             vec![1,1,0,1,1], 
                                             vec![-1,0,1,0,-1],
                                             vec![1,1,0,1,1],
                                             vec![1,1,-1,1,1]];
        let mut dummy_seen: Vec<Vec<bool>> = vec![vec![false,false,false,false, false], 
                                                  vec![false,false,false,false, false], 
                                                  vec![false,false,false,false, false],
                                                  vec![false,false,false,false, false],
                                                  vec![false,false,false,false, false]];
        let expected: Vec<Vec<bool>> = vec![vec![false,true,false,true,false], 
                                            vec![true,true,true,true,true], 
                                            vec![false,true,true,true,false],
                                            vec![true,true,true,true,true],
                                            vec![false,true,false,true,false]];

        reveal_neighbours(2, 2, &dummy_field, &mut dummy_seen);
        assert_eq!(dummy_seen, expected);
    }

    //GET NEIGHBOURS TESTS
    #[test]
    fn get_neighbours_basic_test() {
        let dummy_vec: Vec<Vec<i8>> = vec![vec![1,2,3], vec![4,0,5], vec![6,7,8]];

        let result = get_neighbours(1, 1, &dummy_vec);
        assert_eq!(result, [1,2,3,4,5,6,7,8]);
    }

    #[test]
    fn get_neighbours_outofbounds_upper() {
        let dummy_vec: Vec<Vec<i8>> = vec![vec![1,0,2], vec![3,4,5], vec![-1,-1,-1]];

        let result = get_neighbours(0, 1, &dummy_vec);
        assert_eq!(result, [-2,-2,-2,1,2,3,4,5]);
    }

    #[test]
    fn get_neighbours_outofbounds_left() {
        let dummy_vec: Vec<Vec<i8>> = vec![vec![1,2,8], vec![0,3,8], vec![4,5,8]];

        let result = get_neighbours(1, 0, &dummy_vec);
        assert_eq!(result, [-2,1,2,-2,3,-2,4,5]);
    }
    
    #[test]
    fn get_neighbours_outofbounds_right() {
        let dummy_vec: Vec<Vec<i8>> = vec![vec![8,1,2], vec![8,3,0], vec![8,4,5]];

        let result = get_neighbours(1, 2, &dummy_vec);
        assert_eq!(result, [1,2,-2,3,-2,4,5,-2]);
    }

    #[test]
    fn get_neighbours_outofbounds_lower() {
        let dummy_vec: Vec<Vec<i8>> = vec![vec![8,8,8], vec![1,2,3], vec![4,0,5]];

        let result = get_neighbours(2, 1, &dummy_vec);
        assert_eq!(result, [1,2,3,4,5,-2,-2,-2]);
    }
}