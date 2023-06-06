use std::{io::{stdin, stdout, Write}};
use rand::Rng;
use std::collections::{ HashSet, VecDeque };

/**
 * TODO:
 *  - Build the UI (Move to a new project?)
 * 
 *  Could be nice:
 *      - Hide the mine locations in memory some how? (Could be way harder than necessary for this scope)
 */

fn main() {
    let (mut n, mut m) = (0u8,0u8);

    get_field_size(&mut n, &mut m);
    let mut field: Vec<Vec<i8>> = vec![vec![0;n as usize];m as usize];
    let mut user_visible: HashSet<(u8,u8)> = HashSet::new();
    let mut placed_flags: HashSet<(u8,u8)> = HashSet::new();
    let no_mines = ask_user_mines_no(n, m);
    let (mut guessed_mines, mut real_mines) = (0u8,0u8);
    let mut game_running:bool = true;

    print_opening(n, m);
    let (mut x, mut y) = ask_user_selection(n, m);
    generate_mines(&mut field, n, m, no_mines, x, y);
    let _ = handle_user_guess(x, y, &field, &mut user_visible, &placed_flags);
    print_field(&field, &user_visible, &placed_flags);

    while game_running && real_mines < no_mines {
        println!("Remaining mines: {}", no_mines - guessed_mines);
        (x,y) = ask_user_selection(n, m);
        
        let result = handle_open_or_flag(x, y, &field, &mut user_visible, 
                                                    &mut placed_flags, &mut guessed_mines, &mut real_mines);
        print!("Guess state was a: ");
        match result {
            GuessState::Exploded => { println!("Death!"); game_running = false; },
            GuessState::AlreadySeen => println!("Retry!"),
            GuessState::Success => println!("Safe!")
        };
        print_field(&field, &user_visible, &placed_flags);
    }

    if real_mines == no_mines {
        println!("You Win!");
    }else {
        println!("Oops, you died. Try again?");
    }
}

/**
 * Asks user for placing a flag or revealing the spot
 * Flag -> Places or removes a flag, increments or decrements mine count
 * Reveal -> Reveals the space on the field
 * 
 * Returns success of the guess
 * 
 * ASSUMES VALID COORDINATES
 */
fn handle_open_or_flag(x: u8, y:u8, field: &Vec<Vec<i8>>, seen: &mut HashSet<(u8, u8)>, 
                        flags: &mut HashSet<(u8, u8)>, guessed_mines: &mut u8,
                        real_mines: &mut u8) -> GuessState {
    print!("Reveal (r) or flag (f): ");
    let _ = stdout().flush();
    let mut input_buffer = String::new();

    loop {
        input_buffer.clear();
        stdin().read_line(&mut input_buffer).expect("Failed to read line.");
        match &*input_buffer.trim_end().to_lowercase() {
            "reveal" => return handle_user_guess(x, y, field, seen, flags),
            "r" => return handle_user_guess(x, y, field, seen, flags),
            "flag" => return handle_place_flag(x as usize, y as usize, field, flags, guessed_mines, real_mines, seen),
            "f" => return handle_place_flag(x as usize, y as usize, field, flags, guessed_mines, real_mines, seen),
            _ => { println!("Please respond reveal (r) or flag (f)."); continue; } 
        };
    }
}

/**
 * Sets the bool of the flags vector to true
 * increments or decrements mine count
 * Returns success 
 */
fn handle_place_flag(x: usize, y:usize, field: &Vec<Vec<i8>>, flags: &mut HashSet<(u8, u8)>, 
                        guessed_mines: &mut u8, real_mines: &mut u8, seen: &HashSet<(u8, u8)>) -> GuessState{
    if seen.contains(&(x as u8, y as u8)) { return GuessState::AlreadySeen; }

    if flags.contains(&(x as u8, y as u8)) {
        *guessed_mines -= 1;
        if field[y][x] == -1 { *real_mines -= 1; }
        flags.remove(&(x as u8, y as u8));
    }else {
        *guessed_mines += 1;
        if field[y][x] == -1 { *real_mines += 1; }
        flags.insert((x as u8, y as u8));
    }
    return GuessState::Success
}

/**
* Checks the space of the users reveal guess.
* Has it already been revealed?
* Is it a mine?
*/
fn handle_user_guess(x:u8,y:u8, field: &Vec<Vec<i8>>, seen: &mut HashSet<(u8,u8)>, flags: &HashSet<(u8,u8)>) -> GuessState {
    //Check spot:
    if seen.contains(&(x, y)) || flags.contains(&(x, y)) {
        return GuessState::AlreadySeen;
    }else if field[x as usize][y as usize] == -1 {
        seen.insert((x,y));
        return GuessState::Exploded;
    }

    seen.insert((x,y));
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
fn reveal_neighbours(x:i32, y:i32, field: &Vec<Vec<i8>>, seen: &mut HashSet<(u8, u8)>) {
    let mut deq = VecDeque::from([(x,y)]); // put current spot in queue
    let mut explored: HashSet<(i32,i32)> = HashSet::new();
    explored.insert((x,y)); //insert the first spot
    let (mut diff_x, mut diff_y) = (-1, -1);

    // check each neighbour if it is 0
    while !deq.is_empty() {
        let (x, y) = deq.pop_front().expect("Error popping from deq.");
        let neighbours = get_neighbours(x as usize, y as usize, field);

        for (i, spot) in neighbours.iter().enumerate() {
            if i == 4 { diff_x += 1; } // skip over starting spot
            if neighbours[i] == -2 || neighbours[i] == -1 { 
                if i == 2 || i == 4 { diff_y += 1; }
                if diff_x == 1 { diff_x = -1; } else { diff_x += 1; }
                continue; 
            } //ignore out of bounds and mines
            let curr_spot = ((x + diff_x), (y + diff_y));

            if *spot == 0 && !explored.contains(&curr_spot) { deq.push_back(curr_spot); explored.insert(curr_spot); }
            seen.insert((curr_spot.0 as u8, curr_spot.1 as u8)); // mark as visible

            if i == 2 || i == 4 { diff_y += 1; }
            if diff_x == 1 { diff_x = -1; } else { diff_x += 1; }
        }
        (diff_x, diff_y) = (-1, -1);
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
    neighbours[0] = if y == 0 || x == 0 { -2 } else { field[y-1][x-1] };
    neighbours[1] = if y == 0 { -2 } else { field[y-1][x] };
    neighbours[2] = if y == 0 || x + 1 >= field[0].len() { -2 } else { field[y-1][x+1] };
    
    //same row
    neighbours[3] = if x == 0 { -2 } else { field[y][x-1] };
    neighbours[4] = if x + 1 >= field[0].len() { -2 } else { field[y][x+1] };

    //bottom row
    neighbours[5] = if y + 1 >= field.len() || x == 0 { -2 } else { field[y+1][x-1] };
    neighbours[6] = if y + 1 >= field.len() { -2 } else { field[y+1][x] };
    neighbours[7] = if y + 1 >= field.len() || x + 1 >= field[0].len() { -2 } else { field[y+1][x+1] };

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
    let spaces = " ".repeat((m as f32/10 as f32).ceil() as usize);

    for i in 0..n {
        if i < 10 { print!("{}:{}", i, spaces); }
            else { print!("{}: ", i); } 
        for _ in 0..m {
            print!("?{}", spaces);
        }
        println!();
    }
    print!("{}  ", spaces);
    for _ in 0..m { print!("-{}", spaces); }
    println!();
    print!("{}  ", spaces);
    
    for i in 0..m {
        if i < 10 {
            print!("{}{}", i, spaces);
        }else { print!("{} ", i); }
    }
    println!()
}

fn print_field(field: &Vec<Vec<i8>>, seen: &HashSet<(u8, u8)>, flags: &HashSet<(u8, u8)>) {
    let spaces = " ".repeat((field[0].len() as f32/10 as f32).ceil() as usize);

    for (i, row) in field.iter().enumerate() {
        if i < 10 { print!("{}:{}", i, spaces); }
            else { print!("{}: ", i); } 
        for (j, colmn) in row.iter().enumerate() {
            if seen.contains(&(j as u8, i as u8)) {
                if *colmn == -1 { 
                    print!("X{}", spaces);
                }else { 
                    print!("{}{}", colmn, spaces);
                }
            } else if flags.contains(&(j as u8, i as u8)) {
                print!("#{}", spaces);
            } else {
                print!("?{}", spaces);
            }
        }
        println!();
    }
    print!("{}  ", spaces);
    for _ in 0..field[0].len() { print!("-{}", spaces); }
    println!();
    print!("{}  ", spaces);
    
    for i in 0..field[0].len() {
        if i < 10 {
            print!("{}{}", i, spaces);
        }else { print!("{} ", i); }
    }
    println!()
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
    print!("Enter the height of the field (min 3, max 99): ");
    let _ = stdout().flush();
    loop { 
        read_u8(n); 
        if *n >= 3 && *n <= 99 { break; }
        print!("Invalid number, please try again: "); 
        let _ = stdout().flush();
    }

    print!("Enter the width of the field (min 3, max 99): ");
    let _ = stdout().flush();
    loop { 
        read_u8(m); 
        if *m >= 3 && *m <= 99 { break; }
        print!("Invalid number, please try again: ");
        let _ = stdout().flush();
    }
}

#[cfg(test)]
mod test {
    use std::{vec, collections::HashSet};

    use crate::{ get_neighbours, reveal_neighbours, handle_user_guess, handle_place_flag };
    //DEBUGGING TESTS
    #[test]
    fn check_seen_test() {
        let field: Vec<Vec<i8>> = vec![
            vec![0, 0, 1,-1, 1],
            vec![0, 0, 1, 1, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0]
        ];
        let mut seen: HashSet<(u8, u8)> = HashSet::new();
        let flags: HashSet<(u8, u8)> = HashSet::new();

        handle_user_guess(2, 2, &field, &mut seen, &flags);
        assert!(!seen.contains(&(3,0)));
        assert!(!seen.contains(&(4,0)));
    }


    #[test]
    fn not_showing_flag_test() {
        let field:Vec<Vec<i8>> = vec![  
                                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                                    vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
                                    vec![0, 0, 0, 0, 0, 0, 0, 1,-1, 1],
                                    vec![0, 0, 0, 0, 0, 0, 0, 1, 1, 1],
                                    vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]];
        let mut seen: HashSet<(u8, u8)> = HashSet::new();
        let mut flags: HashSet<(u8, u8)> = HashSet::new();
        let mut guesses: u8 = 0;
        let mut real: u8 = 0;

        handle_user_guess(5, 5, &field, &mut seen, &flags);
        assert!(!seen.contains(&(9, 7)));
        assert!(!seen.contains(&(8, 7)));

        let mut expected_flags: HashSet<(u8, u8)> = HashSet::new();
        expected_flags.insert((8, 7));
        handle_place_flag(8,7, &field, &mut flags, &mut guesses, &mut real, &seen);
        assert_eq!(expected_flags, flags);
    }

    //REVEAL NEIGHBOURS TESTS
    #[test]
    fn reveal_neighbours_basic_test() {
        let dummy_field: Vec<Vec<i8>> = vec![vec![1,2,3], vec![4,0,5], vec![6,7,8]];
        let mut dummy_seen: HashSet<(u8, u8)> = HashSet::new();
        let mut expected: HashSet<(u8, u8)> = HashSet::new();
        expected.insert((0,0)); expected.insert((1,0)); expected.insert((2,0));
        expected.insert((0,1)); expected.insert((2,1));
        expected.insert((0,2)); expected.insert((1,2)); expected.insert((2,2));

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
        let mut dummy_seen: HashSet<(u8, u8)> = HashSet::new();
        let mut expected: HashSet<(u8, u8)> = HashSet::new();
        expected.insert((1,0)); expected.insert((2,0)); expected.insert((3,0));
        expected.insert((0,1)); expected.insert((1,1)); expected.insert((2,1));expected.insert((3,1));expected.insert((4,1));
        expected.insert((0,2)); expected.insert((1,2)); expected.insert((2,2));expected.insert((3,2));expected.insert((4,2));
        expected.insert((0,3)); expected.insert((1,3)); expected.insert((2,3));expected.insert((3,3));expected.insert((4,3));
        expected.insert((1,4)); expected.insert((2,4)); expected.insert((3,4));

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
        let mut dummy_seen: HashSet<(u8, u8)> = HashSet::new();
        let mut expected: HashSet<(u8, u8)> = HashSet::new();
        expected.insert((1,1)); expected.insert((2,1));expected.insert((3,1));
        expected.insert((1,2)); expected.insert((3,2));
        expected.insert((1,3)); expected.insert((2,3));expected.insert((3,3));

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
        let mut dummy_seen: HashSet<(u8, u8)> = HashSet::new();
        let expected: HashSet<(u8, u8)> = HashSet::new();
        reveal_neighbours(2, 2, &dummy_field, &mut dummy_seen);
        assert_eq!(dummy_seen, expected);

    }

    #[test]
    fn reveal_neighbours_hidemines_cascade_test() {
        let dummy_field: Vec<Vec<i8>> = vec![vec![ 1, 1,-1, 1, 1], 
                                             vec![ 1, 1, 0, 1, 1], 
                                             vec![-1, 0, 1, 0,-1],
                                             vec![ 1, 1, 0, 1, 1],
                                             vec![ 1, 1,-1, 1, 1]];
        let mut dummy_seen: HashSet<(u8, u8)> = HashSet::new();

        reveal_neighbours(2, 2, &dummy_field, &mut dummy_seen);
        assert!(!dummy_seen.contains(&(0,0))); assert!(!dummy_seen.contains(&(2,0))); assert!(!dummy_seen.contains(&(4,0)));
        assert!(!dummy_seen.contains(&(0,2))); assert!(!dummy_seen.contains(&(4,2)));
        assert!(!dummy_seen.contains(&(0,4))); assert!(!dummy_seen.contains(&(2,4))); assert!(!dummy_seen.contains(&(4,4)));
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

        let result = get_neighbours(1, 0, &dummy_vec);
        assert_eq!(result, [-2,-2,-2,1,2,3,4,5]);
    }

    #[test]
    fn get_neighbours_outofbounds_left() {
        let dummy_vec: Vec<Vec<i8>> = vec![vec![1,2,8], vec![0,3,8], vec![4,5,8]];

        let result = get_neighbours(0, 1, &dummy_vec);
        assert_eq!(result, [-2,1,2,-2,3,-2,4,5]);
    }
    
    #[test]
    fn get_neighbours_outofbounds_right() {
        let dummy_vec: Vec<Vec<i8>> = vec![vec![8,1,2], vec![8,3,0], vec![8,4,5]];

        let result = get_neighbours(2, 1, &dummy_vec);
        assert_eq!(result, [1,2,-2,3,-2,4,5,-2]);
    }

    #[test]
    fn get_neighbours_outofbounds_lower() {
        let dummy_vec: Vec<Vec<i8>> = vec![vec![8,8,8], vec![1,2,3], vec![4,0,5]];

        let result = get_neighbours(1, 2, &dummy_vec);
        assert_eq!(result, [1,2,3,4,5,-2,-2,-2]);
    }
}