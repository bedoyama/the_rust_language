fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}");

    let five = five();

    println!("The value of five() is {five}");

    let plus1 = plus_one(five);

    println!("The value of plus_one() is {plus1}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
