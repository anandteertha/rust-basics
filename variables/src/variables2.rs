pub fn variable2() {
    let num1: i8 = 10;
    // num1 = 12; will throw error as by default variables are immutable in rust.
    let mut num2: i8 = 12;
    num2 += 1;
    println!("num1 is {}", num1);
    println!("num2 is {}", num2);
}