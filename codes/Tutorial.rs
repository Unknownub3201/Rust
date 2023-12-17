// Like c++ or Java, Rust code starts at main func 
// #[allow(unused_variables)] allows variables that are initiliazed but not used.
// Another way to allow initiliazed but unused variables is by adding '_' before the variable name.
fn main(){
    // Outer Scope 
    let _i:i32;

    //can add various numbers in different number formats 
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; 
    assert!(v == 1597);

    //Variables in Rust are immutable by default, meaning their value cant be changed. 
    //mut defines the variables as mutable, meaning the value can be changed in later stage of the
    //code.
    let mut _test:i32 = 5; 
    let x:i32 = 2;
    {
        // Inner scope 
        println!("Value of x: {x}"); // prints out 2 
    }
    
    //Shadowing(reassigning values to an already initiliazed variable).
    let _x:&str = "This is a test run";

    //Changing value of an existing variable 
    _test = 3; 
    
    //calling another function 
    data_type();
    loop_def();
}

//You can define your own specific functions(It must be called in main function for it to be able
//to execute)
//Function name must follow snake_case naming convention
fn data_type(){

    //Integers can be signed(denoted with i) or unsigned(denoted with u)
    //unsigned integers cant be assigned negative values 
    //32 refers to the bits, can be 8,16,32,64,128 
    let _x:i32 = 5;
    
    //Float 32,64
    let _y:f64 = 2.7; 
    
    //Numeric Operations 
    //Addition
    let _sum = 5 + 10;
    //Subtraction
    let _diff = 42.0-1.34;
    //division
    let _quo = 4.34/2.1;
    let _q = 5/2;
   
    //remainder/Modulus 
    let _mo = 43 % 2;

    //Bool 
    let _t:bool = true;
    let _f:bool = false; 

    //Character 
    let _c:char = 's';

    //String 
    let _string:&str = "Hello, World!";
}

fn loop_def(){
    let mut _x:i32 = 0;
    // A simple for loop, the i takes on values -5,-4....3,4;
    for i in -5..5{
        _x += i;
    }
    
    let mut _sum:u32 = 0;
    // a for loop that iterates through a to z, '=' is used to include the last iteration
    for i in 'a'..='z'{
        _sum += i as u32;
    }
}
