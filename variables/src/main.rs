// const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;

fn main() {
    // Variables and Mutability

    // in Rust, variables are immutable by default
    // the code below will throw an error
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // to fix the error, we can use the mut keyword
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    
    // Constants

    // are always immutable
    // can be declared in any scope, including global scope
    // may only be set to a constant expression, not the result of a function call or any other value that could only be computed at runtime
    // naming covention is UPPER_CASE with underscores between words
    // println!("The value of THREE_HOURS_IN_SECONDS is: {}", THREE_HOURS_IN_SECONDS);
    
    // Shadowing

    // shadowing is when you reuse the name of a variable to create a new variable with the same name
    // shadowing is different from mutability
    // by shadowing a variable, we can reuse the name of a variable and change its type
    let z = 5;
    let z = z + 1;

    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }

    println!("The value of z is: {z}");
}
