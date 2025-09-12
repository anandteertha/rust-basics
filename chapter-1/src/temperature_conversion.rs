use std::io::{self};

pub fn cel_to_fah() -> f32 {
    let mut cel_temp = String::new();
    let stdin = io::stdin();
    println!("Please enter temp in celcius!");
    stdin
        .read_line(&mut cel_temp)
        .expect("Failed to read line");

    let cel_temp: f32 = cel_temp.trim().parse().expect("cannot convert to float sorry!");
    let fah_temp: f32 = cel_temp * 9.0 / 5.0 + 32.0;
    fah_temp
}

pub fn fah_to_cel() -> f32 {
    let mut fah_temp = String::new();
    let stdin = io::stdin();
    println!("\nPlease enter temp in fah!");
    stdin
        .read_line(&mut fah_temp)
        .expect("Failed to read line");

    let fah_temp: f32 = fah_temp.trim().parse().expect("cannot convert to float sorry!");
    let cel_temp: f32 = (fah_temp - 32.0) * 5.0 / 9.0;

    cel_temp
}
