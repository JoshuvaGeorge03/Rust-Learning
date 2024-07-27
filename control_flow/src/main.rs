fn main() {
    println!("Hello, world!");
    if_expression(9);
    if_expression(4);
    if_else_expression(393);
    if_else_expression(3);

    let if_assignement = if if_expression(4) > 2 { 5 } else { 8 };
    println!("if_assignment  {}", if_assignement);

    let else_if = else_if_expressions(4);
    let else_if = else_if_expressions(40);
    let else_if = else_if_expressions(14);
    let else_if = else_if_expressions(65);

}

fn if_expression(x: i32) -> i32 {
    if x % 2 == 0 {
        println!("number is divisible by 2 {x}");
        return x;
    }
    println!("number is not divisible by 2 {x}");
    x
}

fn if_else_expression(x: i32) -> () {
    if x > 18 {
        println!("This movies is not allowed {x}");
    } else {
        println!("This movies is allowed {}", x)
    }
}

fn else_if_expressions(age: isize) {
    if age <= 5 {
        println!("You are babyx  {}", age);
    } else if age <= 18 {
        println!("you are teen {}", age);
    } else if age <= 40 {
        println!("you are unclce {}", age);
    } else if age <= 60 {
        println!("you are grandpa {}", age);
    } else {
        println!("please die already {age}");
    }
}
