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

        user_input.push(input.trim().parse::<i32>().unwrap());
    }
    user_input.sort();
    println!("User input: {:?}", user_input);

    let mut sum: i32 = 0;

    for int in user_input.iter(){
        sum += int;
    }

    println!("The sum is: {}", sum)


    // stdin.read_line(&mut user_input);

    // println!("Your input: {}", user_input);
     
    // for element in user_input.chars(){
    //     println!("your input {}", element);
    // }
}
