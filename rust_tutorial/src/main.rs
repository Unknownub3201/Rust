//Allows Ununsed variables to be included in the program
#![allow(unused)]

//Importing/Referencing the libraries to be used in the program.
//rand isn't part of std library so it must be defined in cargo.toml for it to be used
use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::process::Command;
//Nested import, to import multiple modules from a library
use std::io::{self, BufRead, BufReader, ErrorKind, Write};

//Like C++/Java fn main is where Rust programs start
fn main() {
    // Calling Functions
    basic_one();
    data_types();
    basic_operations();
}

// Functions must be called in main function in order for them to be accessed/executed by rust
fn basic_one() {
    let mut choice: String = String::new();
    println!("What is your name?");

    // variables in Rust are Immutable by default, meaning their values cant be changed over time
    // Variables can be turned into mutable by defining with mut keyboard
    let mut name: String = String::new();
    let greetings: &str = "Nice to meet you";
    io::stdin()
        .read_line(&mut name)
        .expect("Didnt Receive Input.");
    println!("Hello {}! {}", name.trim_end(), greetings);
    // constant variables are declared in Upper case to differentiate them
    // ',' in big values can be replaced with '_' this makes it easier to read
    const ONE_VALUE: u32 = 1_000_000;
    let age: &str = "20";
    // trim here removes whitespace whereas parse converts the age from string to int
    // .expect is builtin way of handling exception
    let mut age: u32 = age.trim().parse().expect("Age wasn't defined as a number.");
    age = age + 1;
    // Value of ONE_VALUE will not be printed with '_'
    println!("I'm {} and I want ${}", age, ONE_VALUE);

    io::stdin()
        .read_line(&mut choice)
        .expect("Didn't Receive any input.");
}

fn data_types() {
    println!("-----Data Types-----\n");
    let mut choice: String = String::new();

    println!("----Scalar Types----\n");
    //Rust has four primary Scalar Types(which represents a single value)
    // -> Integers
    // -> Floating-point numbers
    // -> Booleans
    // -> Character
    println!("---Integers---");
    // Integer Types
    // Signed Integers - i8, i16, i32, i64, isize
    // Unsigned Integers - u8, u16, u32, u64, usize
    // Signed Integers can take up negative values, Unsigned Integers can't
    // Signed Integers can store numbers from -(2)^n-1 to 2^n-1 - 1 (uses two's complement
    // representation)

    println!("Max i8: {}", i8::MAX);
    println!("Max u8: {}", u8::MAX);
    println!("Max i16: {}", i16::MAX);
    println!("Max u16: {}", u16::MAX);
    println!("Max i32: {}", i32::MAX);
    println!("Max u32: {}", u32::MAX);
    println!("Max i64: {}", i64::MAX);
    println!("Max u64: {}", u64::MAX);

    // Rust has f32 and f64 types of Floating-point numbers
    // default is f64 as it is roughly the same speed as f32 but capable of more precision
    println!("---Float---");
    let float: f32 = 0.52;
    let float1: f64 = -1.67;
    println!("Float 32: {}", float);
    println!("Float 64: {}", float1);

    // Rust has two types of Boolean values,
    // -> True
    // -> False
    println!("---Boolean---");
    let is_true: bool = true;
    let is_false: bool = false;
    println!("Bool: {}", is_true);
    println!("Bool: {}", is_false);

    println!("---String & Character---");
    // String defined with "" and characters defined with ''
    // Character
    // represents a unicode scalar value (can represents a lot more than just ASCII)
    let grade: char = '^'; // Use single quotes for characters
    println!("Character: {}", grade);

    // Both String and &str have different uses
    // String is mutable(can be append, insert, remove characters), size of the String is unknownub
    // or may change during runtime
    let test: String = String::from("Hello,");
    println!("String: {}", test);
    // whereas &str is a constant, fixed-length string used to avoid heap allocation for better
    // performance
    let test1: &str = "World!";
    println!("&str: {}\n", test1);

    println!("----Compound Types----\n");
    //Compound Types
    // groups multiple values into one type
    // Rust has two primitve Compound types
    // -> Tuples
    // -> Arrays

    println!("---Tuples---");
    // Tuples
    // hass different types which don't have to be the same data type
    let tup: (i32, f64, u8) = (34, 9.5, 1);
    // printing individual values from a tuple
    let (x, y, z) = tup;
    println!("printing values from a tuple");
    println!("x:{},\ny:{},\nz:{}", x, y, z);
    // values from a tuple can be printed directly using the index
    println!("Value at 2nd index: {}", tup.2);

    println!("---Array---");
    // must have all elements of the same type
    // arrays in rust have fixed-length, they cant grow or shrink in size
    let a = [1, 2, 5, 7, 9, 10];
    println!("Accessing 4th Array element: {}", a[3]);

    io::stdin()
        .read_line(&mut choice)
        .expect("Didn't receive any input.");
}

fn basic_operations() {
    println!("-----Basic Operations-----");
    let mut choice: String = String::new();

    // Both Variables must be of the same data type in order to perform operations on them
    let num1: f32 = 5.0;
    let num2: f32 = 9.0;
    println!("Addition: {}", num1 + num2);
    println!("Subtraction: {}", num1 - num2);
    println!("Division: {}", num1 / num2);
    println!("Multiplication: {}", num1 * num2);
    println!("Modulus: {}", num1 % num2);

    io::stdin()
        .read_line(&mut choice)
        .expect("Didn't receive any input.");
}
