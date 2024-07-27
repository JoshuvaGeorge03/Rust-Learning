fn main() {
    println!("Hello, world!");
    let x = 3000;
    println!("x beofre mut shadowing {}", x);
    let x ;
    let y = 93;
    x = 939;
    let z; 
    z = 3839;
    const PI: f64 = 3.14;
    println!("what is x, y, z {x} {} {}", y, z);
    println!("what is PI {PI}");
}
