fn main() {
    let a = [1, 2, 3, 5, 8];

    for element in a {
        println!("The current element is {element}");
    }

    for number in (1..8).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}
