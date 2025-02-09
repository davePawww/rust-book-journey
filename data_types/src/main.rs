use std::io;

// Rust is a statically typed language, which means that the type of each variable is known at compile time

// There are two types of data types: Scalar and Compound

// Scalar types:
// - Integers
// - Floating-point numbers
// - Booleans
// - Characters

// Compound types:
// - Tuples
// - Arrays


// Scalar types

// Integers
// - Signed integers: i8, i16, i32, i64, i128
// - Unsigned integers: u8, u16, u32, u64, u128 (no negative numbers)

// Floating-point numbers
// - f32
// - f64 (default)

// Booleans
// - true
// - false

// Characters
// - 'a'
// - '1'
// - '💕'


// Compound types

// Tuples
// - ("Hello", 5, 'c')

// Arrays
// - [1, 2, 3]
// - ["Hello", "world"]

fn main() {

    // Type suffixes
    // let a = 57u8; // `a` is an unsigned 8-bit integer
    // let b = 42i32; // `b` is a signed 32-bit integer
    // let c = 3.14f64; // `c` is a 64-bit floating-point number

    // println!("a: {}, b: {}, c: {}", a, b, c);

    let numbers = [10, 20, 30, 40, 50];
    let index: usize = 2; // Using usize for indexing

    println!("The number at index {} is {}", index, numbers[index]);

    find_array_value();
}

// Create a program that would accept a user's input
// they will be asksed to input an array index
// the program will then print the value of the array at that index

fn find_array_value() {
    // the array values
    let numbers = [10, 20, 30, 40, 50];

    // ask the user to input an index
    println!("Enter an index to find the value of the array at that index");

    // create a mutable variable to store the user's input
    let mut index = String::new();

    // read the user's input
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    // convert the user's input to a number
    let index: usize = index.trim().parse().expect("Please type a number!");

    // print the value of the array at the user's input index
    println!("The value of the array at index {} is {}", index, numbers[index]);
}
