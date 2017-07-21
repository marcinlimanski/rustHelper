use std::io::stdin;

fn main() {
    println!("Hello, world!");

    rust_data_types();
}

fn rust_data_types() {
    //By default varibles are inmutable
    let num = 10;
    println!("num: {}", num);

    //Keywoard "mut" means it a mutable data type
    let mut age: i32 = 25;
    println!("age: {}", age);

    let check_flag: bool = true;
    println!("bool: {}", check_flag);

    let char_x: char = 'x';
    println!("char: {}", char_x);

    let (l_name, l_surname) = ("Marcin", "Limanski");
    println!("Name: {0}, surname: {1}", l_name, l_surname);

}

