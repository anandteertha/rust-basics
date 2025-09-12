pub fn return_early(numbers: [i32; 4], val: i32) -> bool {
    // perform linear search..
    for num in numbers {
        if num == val {
            return true;
        }
    }
    false
}