use std::str::Chars;

fn main() {
    println!("Hello, world!");
    joshuva_rocks("joshuva");
    return_expressions(8);
}

fn joshuva_rocks(name: &str) -> () {
    print!("Aloways joshuva rocks {name}");
}

fn return_expressions(x: i8) -> i32 {
    println!("{x}");
  x.into()
}

fn slices_study(s: &String) -> usize {
    let ite = s.chars();
    let mut index = 0;
    for char in ite {
        if char == ' ' {
            return index
        }
        index += 1
    }
    // ite.enumerate().count()
    index
}

fn is_it_possible_to_clone_reference(s: &String) -> String {
    let s1 = s.clone();
    let chars: Chars = s1.chars();
    let mut str = String::from("");
    for char in chars {
        if char == ' ' {
            return str;
        }
        str.push(char);
    }
    str
}