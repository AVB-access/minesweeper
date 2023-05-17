use std::io::{stdin, stdout, Write};
use rand::Rng;
use std::collections::HashSet;

/**
 * TODO: 
 *  - Ask the user for spots to click
 *  - Render the open spots
 *  - Build the UI
 *  - Only generate mines after first spot check
 * 
 *  Could be nice:
 *      - Hide the mine locations in memory some how? (Could be way harder than nessiary for this scope)
 */

/**
 * Dev ideas:
 *  - Field is covered in ?s for text version (working)
 *  - Keep a second Vec to keep track of what the user has reveiled (working)
 *      - Fill with boolean 0/1 for visible/not (working)
 *  - Now when user selects a spot reveal it
 *      - Later also reveal other connected spots
 * 
 */
fn main() {
    let (mut n, mut m) = (0u8,0u8);

    get_field_size(&mut n, &mut m);
    println!("Field sizes: {n}, {m}");
    let mut field: Vec<Vec<i8>> = vec![vec![0;n as usize];m as usize];
    let mut user_visible_field: Vec<Vec<bool>> = vec![vec![false;n as usize]; m as usize];
    let no_mines = ask_user_mines_no();
    generate_mines(&mut field, n, m, no_mines);

    println!("-----------------------"); //Seperating debug print from above call
    debug_print_field(&field);
    println!("-----------------------"); //Seperating debug print from above call
    print_field(&field, &user_visible_field);

    let (x,y) = ask_user_selection(n, m);
    println!("User requested (x,y) = ({},{})", x,y);
}

/**
 * Ask the user for the number of requested mines, place them on the field, then return that number
 * 
 * Params: 
 *  - field: The mine field to place the mines in (set to -1)
 *  - n: the length of the field
 *  - m: the width of the field
 * 
 * Returns: The number of mines put on the field
 */
fn generate_mines(field: &mut Vec<Vec<i8>>, n: u8, m: u8, no_mines: u8) -> u8 {
    let mut set_mines: HashSet<(u8,u8)> = HashSet::new();
    let mut curr_mines = 0;
    let mut rng = rand::thread_rng();

    while curr_mines < no_mines {
        //Generate 2 random numbers
        let y = rng.gen_range(0..n);
        let x = rng.gen_range(0..m);

        //Check if the spot is already a mine
        if set_mines.insert((x,y)) {
            curr_mines += 1;
            field[x as usize][y as usize] = -1;
            update_neighbours(field, m, n, &(x,y))
        }else { continue; /*try again*/ } 
    }

    debug_print_field_raw(&field);

    return no_mines;
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


//TODO limit to n*m - 1
fn ask_user_mines_no() -> u8 {
    let mut mines = 0;

    while mines == 0 {
        print!("Enter number of desired mines (min 1): ");
        let _ = stdout().flush();
        read_u8(&mut mines);

        if mines != 0 { break; }
        print!("Invalid number of mines! Try again: ");
        let _ = stdout().flush();
    }

    return mines;
}

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
        if y < n { break; }

        print!("Out of bounds! Try again: ");
        let _ = stdout().flush();
    }

    return (x,y);
}

fn print_field(field: &Vec<Vec<i8>>, seen: &Vec<Vec<bool>>) {
    for (i, row) in field.iter().enumerate() {
        for (j, colmn) in row.iter().enumerate() {
            if seen[i][j] {
                if *colmn == -1 { print!("X "); }
                else { print!("{} ", colmn); }
            } else {
                print!("? ");
            }
        }
        println!();
    }
}

fn debug_print_field(field: &Vec<Vec<i8>>) {
    for (_i, row) in field.iter().enumerate() {
        for (_j, colmn) in row.iter().enumerate() {
            if *colmn == -1 { print!("X "); }
            else { print!("{} ", colmn); } 
        }
        println!();
    }
}

fn debug_print_field_raw(field: &Vec<Vec<i8>>) {
    for (_i, row) in field.iter().enumerate() {
        for (_j, colmn) in row.iter().enumerate() {
            print!("{} ", colmn);
        }
        println!();
    }
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