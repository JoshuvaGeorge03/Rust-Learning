fn main() {
    println!("Hello data_types");
    let x: i8 = 1_2_7;
    println!("what is x {x}");
    let hex: u32 = 0x23ab;
    println!("hax value {hex}");
    let oct: i32 = 0o1234563;
    println!("octal value {}", oct);
    let binary: u8 = 0b01010;
    println!("binary {binary}");
    let arch_bit:isize = 939;
    print!("archi bit siez {}", arch_bit);


    let character: char = 'x';
    let car = '$';

    let boolea: bool = true;

    let tuple = ("whd", 9.33, 2, '3', true );
    let destructure_typle = tuple.0;
    println!("tuple {destructure_typle}");

    let arr = [1,2,33];
    let str_arr = ["dkd", "dkd"];
    let char_arr = ['d', 'j'];

}
