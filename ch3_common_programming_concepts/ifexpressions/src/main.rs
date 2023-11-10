fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was not 0!");
    }

    if number % 2 == 0 {
        println!("divisible by 2");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else {
        println!("not divisible by 2,3");
    }

    let condition = true;

    let number = if condition { 4 } else { 5 };

    println!("number is {number}");
}
