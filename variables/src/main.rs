mod variables1;
mod variables2;
mod shadowing;

fn main() {
    variables1::variable1();
    variables2::variable2();
    shadowing::not_shadow();
    shadowing::shadow();
}