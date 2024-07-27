use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};
// use std::process::exit;
fn main() {
    println!("Guess The Number, Einstein :)");

    let secret_number = rand::thread_rng()
                                .gen_range(1..=100);

    loop {

        print!("Please input the secret number guess ::: ");

        io::stdout().flush().expect("Stdout error");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Stdin error");

        let guess= match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter the valid number");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("successfully found out the number, Wooha Einstein");
                break;
            },
            Ordering::Greater => println!("too high"),
            Ordering::Less => println!("too low")
        }

    }
}

fn alternate_impl() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        print!("Please Input the Number:::  ");

        io::stdout()
            .flush()
            .expect("OS error, when printing to the terminal");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Std Error failed to give output");

        let guess = guess.trim().parse::<i32>().expect("please input a number");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You have successfully found out the secret number, ::) Wooha");
                break;
            }
            Ordering::Less => println!("too low, guess again"),
            Ordering::Greater => println!("toow high, guess again"),
        }
    }
}
