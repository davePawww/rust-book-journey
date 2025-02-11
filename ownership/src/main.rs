fn main() {
    // This will not work because string literals are immutable
    // let mut s = "hello"
    // s.push_str(", world")
    // println!("{}", s)

    // This will work because we are using a mutable string
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    move_example();
    clone_example();
    copy_example();
}

// Variables and Data interacting with Move
fn move_example() {
    // This will work fine becase integers are simple values with a known fixed size
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // What is happening above will not be the same for strings
    // This will cause double free error because when s1 and s2 go out of scope, they will both try to free the same memory
    // This is a sample of a move
    // when s1 was assigned to s2, the value of s1 was moved to s2 and s1 is no longer valid
    let s1 = String::from("hello");
    let s2 = s1;

    println!("s = {}", s2);

    // Note: Rust will never automatically create deep copies of your data
}

fn clone_example() {
    // if we want to create a deep copy of the data, not just the stack data, but the heap data as well, we can use the clone method
    let s1 = String::from("hello");
    let s2 = s1.clone();

    // anything with clone could be considered as expensive
}

fn copy_example() {
    // Copy trait is used to create a shallow copy of the data
    // Copy trait is used for types that are simple values with a known fixed size
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // Copy trait is used for types that are simple values with a known fixed size
    // Types that implement the Copy trait are:
    // - Integers
    // - Booleans
    // - Floating point numbers
    // - Characters
    // - Tuples if their elements implement the Copy trait
    
}

fn ownership_and_functions() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.



fn return_values_and_scope() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

  fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

    a_string  // a_string is returned and moves out to the calling function
}