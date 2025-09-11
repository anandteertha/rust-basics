pub fn variable1() {
    // number is an int.
    let number: i8 = 10;
    println!("number is {}", number);
    // if we try to convert number directly to string it won't allow us.
    // number = "10";
    // number is now a string..
    let number: &str = "11";
    println!("number is {}", number);
}
