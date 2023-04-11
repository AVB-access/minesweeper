use std::io::{stdin, stdout, Write};

fn main() {
    let (mut n, mut m) = (0u8,0u8);

    get_user_in(&mut n, &mut m);
    println!("Field sizes: {n}, {m}");
    let mut field: Vec<Vec<u8>> = vec![vec![0;n as usize];m as usize];

    print_field(&field);
}

fn print_field(field: &Vec<Vec<u8>>) {
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