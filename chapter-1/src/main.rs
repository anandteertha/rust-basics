mod temperature_conversion;
use temperature_conversion as tc;

fn main() {
    println!("so the temp in fah is {}", tc::cel_to_fah());
    println!("so the temp in cel is {}", tc::fah_to_cel());
}
