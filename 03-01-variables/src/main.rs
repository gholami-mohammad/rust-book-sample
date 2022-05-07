fn main() {
    // immutable var
    // type annotation is optional in variables
    let i:i32 = 6;
    println!("immutable i: {}", i);

    // mutable var
    let mut m: i32 = 32;
    println!("mutable m: {}", m);
    m = 95;
    println!("mutable m: {}", m);
    
    // shadowing
    let sh = 13000;
    println!("shadowing sh: {}", sh);
    let sh = "changed to string";
    println!("shadowing sh: {}", sh);

    // numeric literal
    let nlit : u32 = 120_000_000;
    println!("numeric literal: {}", nlit);


    // const
    // must be type annotated
    const APP_NAME:&str= "Rust tutorial";
    println!("APP_NAME is: {}", APP_NAME);

}
