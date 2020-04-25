//! https://open.kattis.com/problems/10kindsofpeople
//!
//! 
//! DOESN'T WORK AS EXPECTED:
//! 
//! Users of binary have to stay in the zones marked with a zero. 
//! Users of decimal have to stay in the zones marked with a one. 
//! You have to figure out if it is possible for either type of person to get between various locations of interest. 
//! People can move north, south, east or west, but cannot move diagonally.
//! 
//! REQUIRES MORE COMPLEX CHECKING!
//! 
//! 



use std::io;

struct Distance {
    r1: u16,
    c1: u16,
    r2: u16,
    c2: u16,
}

fn main() {

    let error_msg = "Invalid input.".to_string();

    // get the number of lines and the length of a single line

    let mut chars: u16 = 0;
    let mut rows: u16 = 0;
    
    let mut input = String::new();

    println!("insert number of lines and length of line as integers separated with a space");

    io::stdin().read_line(&mut input).expect(&error_msg);

    let dims: Vec<&str> = input.split(' ').collect();

    chars = dims[1].trim().parse().expect(&error_msg);
    rows = dims[0].trim().parse().expect(&error_msg);

    // println!("line length is {}, number of lines is {}", chars, rows);


    // get the lines data in binary format, invalid inputs will be treated as zeros

    let mut data = Vec::new();
    
    for _ in 0..rows {
        let mut input = String::new();
        let parsed: u32;
        println!("enter data in binary format: ");
        io::stdin().read_line(&mut input).expect("invalid input.");
                
        match u32::from_str_radix(&input.trim(), 2_u32) {
            Ok(n) => parsed = n,
            Err(_e) => parsed = 0,
        }

        data.push(parsed);
    };

 
    let mut queries: u16 = 0;

    let mut input = String::new();

    println!("insert the required number of queries: ");

    io::stdin().read_line(&mut input).expect(&error_msg);

    queries = input.trim().parse().expect(&error_msg);

    // get the queries

    let mut points = Vec::new();
    
    for _ in 0..queries {

        let mut input = String::new();

        println!("enter query as points coordinates, separated with spaces: ");
        io::stdin().read_line(&mut input).expect("invalid input.");

        let query: Vec<&str> = input.split(' ').collect();


        let dist = Distance {
            r1: query[0].trim().parse().unwrap(),
            c1: query[1].trim().parse().unwrap(),
            r2: query[2].trim().parse().unwrap(),
            c2: query[3].trim().parse().unwrap(),
        };

        points.push(dist);

    }

    let mut outcome = String::new();

    for p in points.iter() {

        let mut val_start: u16 = 0;
        let mut val_end: u16 = 0;

        let word1 = data[(p.r1-1) as usize]; // get the correct row 
        let word2 = data[(p.r2-1) as usize]; // get the correct row 

        let bit1 = p.c1 - 1; // get the correct bit index
        let bit2 = p.c2 - 1; // get the correct bit index

        let factor1 = (chars - 1 - bit1) as u32;
        let factor2 = (chars - 1 - bit2) as u32;

        let val_start = (word1 / 2_u32.pow(factor1))%2;
        let val_end = (word2 / 2_u32.pow(factor2))%2;

        if val_start == val_end {
            if val_start == 0 {
                outcome = "binary".to_string();
            } else {
                outcome = "decimal".to_string();
            } 
        } else 
            { 
                outcome = "neither".to_string();
            }

        println!("{}", outcome);
        

    }

}
