pub fn not_shadow() {
    println!("...not shadow...");
    let mut x: i8 = 67;
    {
        x += 1;
        println!("in scope x is {}", x);
    }
    println!("out scope x is {}\n", x);
}

pub fn shadow() {
    println!("...shadow...");
    let x: i8 = 67;
    {
        let x: &str = "hello";
        println!("x in scope is {}", x);
    }
    println!("x out of scope is {}\n", x);
}