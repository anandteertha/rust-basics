mod parameterized_functions;
mod return_early;

fn main() {
    println!("use parameters with type mention");
    println!("need to add a return type to the fn");
    println!("no need to mention return keyword when returning implicitly!!");
    println!("The sum of 4 and 5 is {}\n",parameterized_functions::add(4, 5));
    println!("...return early example...");
    println!("is 3 in [1,2,3,4]? \n=> {}",return_early::return_early([1,2,3,4], 3));
    println!("is 10 in [1,2,3,4]? \n=> {}",return_early::return_early([1,2,3,4], 10));
}

