use std::io;

use rand::Rng;

pub fn guess_num(){
    println!("try input a number");
    // Generates random numbers with ranges
    let result = rand::thread_rng().gen_range(0, 1000);
    // Similar to 'while true'
    loop {
        // Acquire new String space
        let mut input = String::new();
        // Read from input
        io::stdin().read_line(&mut input).expect("Failed to readLine()");
        // match ok err type
        match input.trim().parse::<i32>() {
            Ok(..)=>{
                let mut input:i32 = input.trim().parse::<i32>().unwrap();
                if input == result {
                    println!("Right");
                    break;
                } else if input < result {
                    println!("Try Bigger");
                }else if input > result {
                    println!("Try Smaller");
                }
            }
            Err(input)=>println!("Wrong input Type {}", input)
        }
        // expect() type
        // let mut int_input:i32 = input.trim().parse().expect("Please Input A Number");
        // if int_input > result {
        //     println!("try smaller");
        // }else if int_input < result {
        //     println!("try bigger");
        // }else {
        //     println!("bingo");
        //     break;
        // }
    }
}