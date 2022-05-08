fn main() {
    // scalar data types:
    //      Integers
    //      Floating-point numbers
    //      Booleans
    //      Characters

    // compound types
    //      Tuples
    //      Arrays

    // Integers
    let a: i32 = 23_3232; // decimal
    println!("deciaml       a:{}", a);
    let b: i32 = 0xd; // hex 0xd=13
    println!("hex           b:{}", b);
    let c = 0o77; // octal
    println!("octal         c:{}", c);
    let d = 0b10_010_111_000; // binary
    println!("binary        d:{}", d);
    let e = b'M'; // byte (u8 only) - b'M' and 77 are equal
    println!("byte u8       e:{}", e);
    let f: u8 = 77;
    println!("u8            f:{}", f);

    // Floating-point numbers
    let g: f64 = 23.11;
    println!("float         g:{}", g);

    // Booleans
    let t = true;
    let f = false;
    println!("boolean       t: {} f: {}", t, f);

    // Characters
    let a = 'a';
    let z = 'Z';
    println!("chars         {}, {}", a, z);

    // Tuples
    let tup = ("Rust lang", 13, true); // tuples' elements can be multiple types
    let (name, id, active) = tup;
    println!(
        "tuple         name: {}, id: {}, active: {}",
        name, id, active
    );

    let id = tup.1;
    println!("id index      {}", id);

    // Arrays
    let arr_codes = [200, 404, 422, 403]; // array's element must be same type
    let not_found = arr_codes[1];
    println!("not_found     {}", not_found);

    // auto fill array
    let filled_array = [5, 12]; // an array with 12 elements and all elements are 5
    println!("filled_array  {:?}", filled_array);
}
