mod write_to_yaml;

//work in progress
use std::io::StdinLock;
use std::io::{self, BufRead};
use crate::write_to_yaml::*;



fn main() {
    loop {
        let (mut user_input, mut handle, mean_final_result) = test_values();
        let mut file = create_file();
        match file {
            Ok(ref _file) => { println!("YAML created successfully. Your input will be written into it")}
            Err(..) => { println!("YAML creation failed, the program will continue executing")}
        }
        insert_vector("input", &user_input, &mut file);
        let mut vec_len: usize = user_input.len();

        let mut n_count = n_count_checker(&mut handle, &vec_len);

        let half_vec_length = (vec_len / 2).try_into().unwrap();

        while n_count > half_vec_length {
            println!("\nAt most, you can remove up to {} values. Try again", half_vec_length);
            n_count = n_count_checker(&mut handle, &vec_len);
            if n_count < half_vec_length {
                break;
            }
        }

        println!("\n\n{} values from the top and bottom will be removed", n_count);

        slice_vec(&mut user_input, n_count);
        insert_vector("trimmed_values", &user_input, &mut file);
        println!("\nUpdated vector: {:?}", user_input);

        vec_len = user_input.len();

        let final_result = main_logic(&mut user_input, &mut vec_len);

        insert_f64("final_result", final_result, &mut file);

        println!("And the arithmetic mean was: {}", mean_final_result);
        insert_f64("arithmetic_mean", mean_final_result, &mut file);
    }
}

fn main_logic(user_input: &mut Vec<f64>, vec_len: &mut usize) -> f64 {
    let mut sum: f64 = 0.0;

    for i in user_input {
        sum = sum + *i as f64;
    }

    let final_result: f64 = sum / *vec_len as f64;
    println!("{}", "-".repeat(100));
    println!("{}The trimmed mean is: {}", " ".repeat(40), final_result);
    println!("{}", "-".repeat(100));
    final_result
}

fn slice_vec(user_input: &mut Vec<f64>, n_count: i64) {
    let range = 0..n_count as usize;
    user_input.drain(range).for_each(drop);
    let end_index = user_input.len().saturating_sub(n_count.try_into().unwrap());
    user_input.truncate(end_index);
}

fn test_values() -> (Vec<f64>, StdinLock<'static>, f64) {
    println!("\n\nEnter your values:");
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
    let mut sum: f64 = 0.0;
    for i in &user_input{
        sum = sum + i;
    }
    let mean_final_result: f64 = sum / user_input.len() as f64;
    (user_input, handle, mean_final_result)
}

fn n_count_checker(mut handle: &mut StdinLock<'static>, vec_len: &usize) -> i64 {
    let n_count: i64 = remove_n_values(&mut handle);

    if n_count > ((vec_len / 2).try_into().unwrap()) {
        print!("Not possible");
        // exit(0)
    }
    n_count
}

fn remove_n_values(handle: &mut StdinLock<'static>) -> i64 {
    println!("\nHow many values would you like to remove:");
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
