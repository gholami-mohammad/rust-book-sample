enum OS {
    GenuLinux(String),
    Windows(String),
    MacOS(f32),
    Others(String),
    NoOS,
}

impl OS {
    // defining a function for enum
    fn string(&self) -> String {
        // pattern matching on enum
        let res = match self {
            OS::GenuLinux(dist) => format!("Genu/Linux {}", dist),
            OS::Windows(version) => format!("Microsoft Windows {}", version),
            OS::MacOS(version) => {
                if version > &10.11 {
                    return format!("MacOS {}", version);
                }
                return format!("OSX {}", version);
            }
            OS::Others(other) => String::from(other),
            OS::NoOS => String::from("No OS used"),
        };

        return res;
    }

    // How to avoid matching for all cases
    fn is_linux(&self) ->bool {
        match self {
            OS::GenuLinux(_) => true,
            _ => false, // all other matches will be handled in this case
        }
    }
}

fn main() {
    let ubuntu = OS::GenuLinux(String::from("Ubuntu"));
    let win7 = OS::Windows(String::from("7"));
    let macos = OS::MacOS(10.5);
    let other_os = OS::Others(String::from("Solaris"));
    let no = OS::NoOS;
    println!(
        "ubuntu: {}, win7: {}, macos: {}, other: {}, no OS: {}",
        ubuntu.string(),
        win7.string(),
        macos.string(),
        other_os.string(),
        no.string()
    );

    println!("is_linus: {}", win7.is_linux());
    println!("is_linus: {}", ubuntu.is_linux());

    // using Option<T>
    let age : Option<i32> = Some(10);
    let none_age: Option<i32> = None; // no value
    println!("age: {:?}, none_age: {:?}", age, none_age);

    // Option<T> and unwrap_or
    let x: i32 = 10;
    let y: Option<i32> = Some(5);
    let sum = x + y.unwrap_or(0);
    println!("Sum is {}", sum);


    // Option<T> and match expression
    let n1 = tell_next_number(None);
    println!("n1: {}", n1);
    let n2 = tell_next_number(Some(33));
    println!("n2: {}", n2);


    // matchin specific case using if let
    if let OS::GenuLinux(dist) = OS::GenuLinux(String::from("CentOS")) {
        println!("selected dist is: {}", dist);
    }

    if let OS::NoOS = win7 {
        println!("selected laptop has no os");
    } else {
        println!("selected laptop has os");
    }
}


fn tell_next_number(n: Option<i32>)-> i32 {
    return match n {
        None => 0,
        Some(i) => i+1,
    };
}