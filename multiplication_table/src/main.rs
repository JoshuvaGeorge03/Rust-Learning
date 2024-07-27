use std::io::{self, Write};
fn main() {
    let mut number_for_multiplication = String::new();

    print!("Please input number to do multiplication ::: ");

    let _ = io::stdout().flush();

    io::stdin()
        .read_line(&mut number_for_multiplication)
        .expect("Failed To Read By Terminal");

    let number_for_multiplication = number_for_multiplication
        .trim()
        .parse::<u32>()
        .expect("Please Enter valid number");


    print!("Please input number of sequence to do ::: ");

    let _ = io::stdout().flush();

    let mut number_of_times_todo_the_sequence = "".to_string();

    io::stdin()
        .read_line(&mut number_of_times_todo_the_sequence)
        .expect("Failed To handle the input from terminal");

    let number_of_times_todo_the_sequence: u32 = number_of_times_todo_the_sequence
        .trim()
        .parse()
        .expect("Please input valide number");


    let mut index = 1;
    
    loop {

        if index > number_of_times_todo_the_sequence {
            break;
        }

        let output_value: u32 = number_for_multiplication * index;

        println!("{index} * {number_for_multiplication} = {};", output_value);

        index += 1;

    };


}
