fn main() {
    say_hi("Mohammad");

    let s = add_with_return_keyword(45, 12);
    println!("add_with_return_keyword:      {}", s);

    let s = add_without_return_keyword(45, 12);
    println!("add_without_return_keyword:   {}", s);
}

fn say_hi(name: &str) {
    println!("Hi {}", name);
}

fn add_with_return_keyword(a: i32, b: i32) -> i32 {
    let sum = a + b;
    return sum;
}

fn add_without_return_keyword(a: i32, b: i32) -> i32 {
    let sum = a + b;

    // no return keyword and no ; at the end of the line to return
    sum
}
