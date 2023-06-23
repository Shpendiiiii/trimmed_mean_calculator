//work in progress
use std::io::StdinLock;
use std::io::{self, BufRead};
use std::fmt::Display;
use std::process::exit;
use std::vec;

fn main() {
    let mut user_input = Vec::new();
    let mut handle = user_input_handle();

    loop {
        let mut input = String::new();

        handle.read_line(&mut input).expect("Could not read input. Try again.");

        if input.trim().is_empty() {
            break;
        }

        user_input.push(input.trim().parse::<f64>().unwrap());
    }
    user_input.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("User input: {:?}", user_input);

    let vec_len: usize = user_input.len();

    let mut n_count = n_count_checker(&mut handle, &vec_len);

    let half_vec_length = (vec_len / 2).try_into().unwrap();

    while n_count > half_vec_length {
        println!("\nAt most, you can remove up to {} values. Try again", half_vec_length);
        n_count = n_count_checker(&mut handle, &vec_len);
        if n_count <  half_vec_length{
            break;
        }
    }

    println!("{}", n_count);
    println!("half of vec len {}", half_vec_length);

    let range = 0..n_count as usize;
    println!("the ncount after the first slice {}", n_count);

    // println!("the length of the vector is {}", vec_len);

    let final_vec: Vec<f64> = user_input.drain(range).collect();
    let end_index = user_input.len().saturating_sub(n_count.try_into().unwrap());
    user_input.truncate(end_index);
    println!("Removed elements: {:?}", final_vec);
    // println!("Removed elements: {:?}", final_vec2);

    println!("Updated vector: {:?}", user_input);
    println!("second updated vector {:?}", user_input);


}

fn n_count_checker(mut handle: &mut StdinLock<'static>, vec_len: &usize) -> i64 {
    let n_count: i64 = remove_n_values(&mut handle);

    if n_count > ((vec_len / 2).try_into().unwrap()) {
        print!("Not possible");
        // exit(0)
    } else { println!("Possible") }
    n_count
}

fn remove_n_values(handle: &mut StdinLock<'static>) -> i64 {
    let mut remove_n_values: String = String::new();

    handle.read_line(&mut remove_n_values).expect("Failed to read value, try again");

    let mut n_count: i64 = 0;

    match remove_n_values.trim().parse::<i64>() {
        Ok(i) => n_count = i,
        Err(..) => println!("Parse Int Error")
    }
    n_count
}

fn user_input_handle() -> StdinLock<'static> {
    let stdin: io::Stdin = io::stdin();
    let handle: StdinLock<'_> = stdin.lock();
    handle
}
