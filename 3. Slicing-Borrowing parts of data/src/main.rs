use crate::text_utils::{get_first_word, get_last_word};

mod text_utils;

fn main() {
    let owned_string = String::from("rust makes borrowing explicit");
    let first_word = get_first_word(&owned_string);
    println!("main: first word of `{}` is `{}`", owned_string, first_word);

    let last_word = get_last_word(&owned_string);
    println!("main: last word of `{}` is `{}`", owned_string, last_word);

    let random_slice = &owned_string[5..10];
    println!(
        "main: random slice of `{}` from 5..10 is `{}`",
        owned_string, random_slice
    );

    let numeric_array = [1, 2, 3, 4];
    println!(
        "main: middle two elements of `{:?}` is `{:?}`",
        numeric_array,
        &numeric_array[1..3]
    );
}
