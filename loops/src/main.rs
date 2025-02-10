fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // break statement returns a value
            break counter * 2;
        }
    };

    println!("The result is {result}");

    nested_loops();

    while_loop();

    let a = [10, 20, 30, 40, 50, 60];
    for_loop(a)
}

fn nested_loops() {
    let mut count = 0;

    // loop labels can be used to break or continue an outer loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_loop(a: [i32; 6]) {
    for element in a {
        println!("the value is: {element}");
    }
}