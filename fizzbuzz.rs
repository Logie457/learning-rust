// FizzBuzz
// take a set of numbers, and for each number do the following work
// - if its divisible by 3, print out Fizz
// - if its divisible by 5, print out Buzz
// - if its divisible by both, print FizzBuzz
// if all fails, print out the number itself

// 1 2 3 4 5
// 1, 2, Fizz, 4, Buzz

fn main() {
    // < > + - * / == !=

    for index in 1..60 {
        if index % 3 == 0 && index % 5 == 0 { print!("fizzbuzz "); } 
        else if index % 3 == 0 { print!("fizz "); }
        else if index % 5 == 0 { print!("buzz "); }
        else { print!("{index} ") } 
    }
}
