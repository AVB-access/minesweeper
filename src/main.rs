use std::io::{stdin, stdout, Write};
use rand::Rng;
use std::collections::HashSet;
use std::collections::VecDeque;

/**
 * TODO: 
 *  - Only generate mines after first spot check
 *  - Render the open spots
 *  - Build the UI
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
    let no_mines = ask_user_mines_no(n, m);
    generate_mines(&mut field, n, m, no_mines);

    println!("-----------------------"); //Seperating debug print from above call
    debug_print_field(&field);
    println!("-----------------------"); //Seperating debug print from above call
    print_field(&field, &user_visible_field);

    let (x,y) = ask_user_selection(n, m);
    println!("User requested (x,y) = ({},{})", x,y);
    handle_user_guess(x as usize, y as usize, &field, &mut user_visible_field);
    print_field(&field, &user_visible_field);
}

/**
 * TODO: Check for first turn, maybe as bool in main
* Check spot:
*  - Has it been revealed already?
*  - Is it on a mine?
*  - Otherwise reveal it
*/
fn handle_user_guess(x:usize,y:usize, field: &Vec<Vec<i8>>, seen: &mut Vec<Vec<bool>>) {
    //Check spot:
    if field[x][y] == -1{
        return; //TODO return End game
    }else if seen[x][y]{
        return; //TODO return try again
    }

    seen[x][y] = true;
    reveal_neighbours(x as i32, y as i32, field, seen);
}

/**
* Revealing: (seperate function)
*  - Check spot as visible(seen)
*  - neighbour is 0, reveal that spot as well
*      - breadth first search of neighbours?
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
            if neighbours[i] == -2 { continue; }

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
 * Returns array structured as: [NW, N, NE,E,W,SW,S,SE]
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
 */
fn generate_mines(field: &mut Vec<Vec<i8>>, n: u8, m: u8, no_mines: u8) {
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