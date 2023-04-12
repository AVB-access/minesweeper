use std::io::{stdin, stdout, Write};
use rand::Rng;
use std::collections::HashSet;

fn main() {
    let (mut n, mut m) = (0u8,0u8);

    get_user_in(&mut n, &mut m);
    println!("Field sizes: {n}, {m}");
    let mut field: Vec<Vec<i8>> = vec![vec![0;n as usize];m as usize];

    print_field(&field);
    println!("Now asking for mines...");
    let _no_mines = generate_mines(&mut field, n, m);
    print_field(&field);
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
fn generate_mines(field: &mut Vec<Vec<i8>>, n: u8, m: u8) -> u8 {
    let mut set_mines: HashSet<(u8,u8)> = HashSet::new();
    let no_mines = ask_user_mines_no();
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
            update_neighbours(field, m, n, &(x,y)) //TEMP does nothing
        }else { continue; /*try again*/ } 
    }

    return no_mines;
}

/**
 * Increment all the neighbours of a mine by one
 */
fn update_neighbours(_field: &mut Vec<Vec<i8>>, m: u8, n: u8, point: &(u8,u8)) {
    if point.0 > 0 && point.1 > 0 { /* NW */}
    if point.0 > 0 { /* N */}
    if point.0 > 0 && point.1+1 < n { /* NE */}
    if point.1 > 0 { /* W */}
    if point.1+1 < n { /* E */}
    if point.0+1 < m && point.1 > 0 { /* SW */}
    if point.0+1 < m { /* S */}
    if point.0+1 < m && point.1+1 < n { /* SE */}
}

fn ask_user_mines_no() -> u8 {
    let mut mines = 0;
    let mut input_buffer = String::new();

    while mines == 0 {
        print!("Enter number of desired mines (min 1): ");
        let _ = stdout().flush();
        stdin().read_line(&mut input_buffer).expect("Failed to read from stdin.");
        mines = match input_buffer.trim_end().parse::<u8>() {
            Ok(num) => num,
            Err(err) => { eprintln!("Error parsing number in {input_buffer}: {err}"); 0 }
        }
    }

    return mines;
}
//TODO: Need to change mines to 'X's later
fn print_field(field: &Vec<Vec<i8>>) {
    for (_i, row) in field.iter().enumerate() {
        for (_j, colmn) in row.iter().enumerate() {
            print!("{} ", colmn);
        }
        println!();
    }
}

fn get_user_in(n:&mut u8, m:&mut u8) {
    let mut input_buffer = String::new();
    loop {
        print!("Enter the length of the field (min 3): ");
        let _ = stdout().flush();
        stdin().read_line(&mut input_buffer).expect("Failed to read line.");
        *n = match input_buffer.trim_end().parse::<u8>() { 
            Ok(num) => num, 
            Err(err) => { eprintln!("Error parsing number in {input_buffer}: {err}"); 0 }
        };
        input_buffer.clear();
        if *n != 0 { break; }
    }

    loop {
        print!("Enter the width of the field (min 3): ");
        let _ = stdout().flush();
        stdin().read_line(&mut input_buffer).expect("Failed to read input.");
        *m = match input_buffer.trim().parse::<u8>() { 
            Ok(num) => num, 
            Err(err) => { eprintln!("Error parsing number in {input_buffer}: {err}"); 0 }
        };
        input_buffer.clear();
        if *m != 0 { break; }
    }
}