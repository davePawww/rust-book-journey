// this function is also a statement
// calling a function is not a statement because statements do not return values

// calling a function is an expression
// calling a macro is an expression
// a new scope block created with curly braces is an expression
fn main() {
    // this is a statement
    // let x = 5;

    let y = {
        let x = 3;
        // expressions do not end with a semicolon
        x + 1
    };

    println!("The value of y is: {y}");

    // another_function(5, "hadada");
}

// parameters or arguments must be typed
// char is used for single characters and is passed using single quotes
// &str is used for strings and is passed using double quotes
// fn another_function(x: i32, label: &str) {
//     println!("Another function. {}:{}", x, label);
// }