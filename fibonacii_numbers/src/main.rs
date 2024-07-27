use std::io::Write;


fn main() {

    let mut input_string = String::new();

    print!("Please input fibonacci sequence number: ");

    std::io::stdout().flush().expect("Stdout error when flushing contents to stdout");

    std::io::stdin().read_line(&mut input_string).expect("StdIn input error");

    let input_number: u32 = input_string.trim().parse().expect("Please Input valid number");

    const PREVIOUS_TWO_NUMBERS: (usize, usize) = (0, 1);

    let (mut previous_second_number, mut previous_first_number) = PREVIOUS_TWO_NUMBERS;

    println!("The fibonacci number sequence for the input number of {input_number}");
    
    print!("{previous_second_number}, {previous_first_number}, ");
    
    let mut start_sequence = 2;

    'fibonacci_label: loop {
        if input_number >= start_sequence {

            let current_number = previous_first_number + previous_second_number;
            print!("{current_number}, ");
            previous_first_number = current_number;
            previous_second_number = previous_first_number;
            start_sequence += 1;
        }
        else {
            break 'fibonacci_label;
         }
    }
}
