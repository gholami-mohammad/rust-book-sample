fn main() {
    // If statement
    let x = 1;
    if x == 1 {
        println!("It's Sunday");
    } else if x == 2 {
        println!("It's Monday");
    }

    // single line if to set value in a variable
    let e = true;
    let score = if e { 13 } else { 2 };
    println!("score is {}", score);

    // Loops

    // loop with break
    let mut counter = 0;
    loop {
        println!("loop counter: {}", counter);
        counter += 1;
        if counter == 3 {
            println!("loop ended");
            break;
        }
    }

    // loop result to a variable
    let mut x = 12;
    let loop_result = loop {
        x -= 1;
        if x % 3 == 0 {
            break x; // break the loop and return the value of x to the loop_result
        }
    };
    println!("loop_result: {}", loop_result);

    // while loop
    let mut i = 5;
    while i >= 0 {
        println!("while loop index: {}", i);
        i -= 1;
    }

    // for loop to iterate over arrays and collections
    let codes = [200, 400, 422];
    for c in codes.iter() {
        println!("For {}", c);
    }

    // for loop to iterate over ranges
    for c in 1..5 {
        println!("range loop: {}", c);
    }

    // infinit loop. uncomment to see the result
    // loop {
    //     println!("infinit loop");
    // }
}
