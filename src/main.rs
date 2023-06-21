//work in progress

use std::io::{self, BufRead};
use std::fmt::Display;
fn main() {
    let mut user_input = Vec::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        let mut input = String::new();

        handle.read_line(&mut input);

        if input.trim().is_empty() {
            break;
        }

        user_input.push(input.trim().parse::<f64>().unwrap());
    }
    user_input.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("User input: {:?}", user_input);

    let vecLen = user_input.len();

    let mut sum: i8 = 0;

    let mut removeNValues = String::new();
    
    handle.read_line(&mut removeNValues).expect("Failed to read value, try again");

    let nValue = match removeNValues.trim().parse::<i8>() {
        Ok(i % 2 == 0) => println!("{} elements will be taken out", i),
        Err(..) => println!("The input was not an int")
    };

   


    // stdin.read_line(&mut user_input);

    // println!("Your input: {}", user_input);
     
    // for element in user_input.chars(){
    //     println!("your input {}", element);
    // }
}
