use std::io;

fn main() {
    println!("Please input a temperature value.");

    let mut source_temp = String::new();

    io::stdin()
        .read_line(&mut source_temp)
        .expect("Failed to read line");

    let source_temp: f64 = match source_temp.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };

    println!("Your source_temp: {source_temp}");

    let final_farenheit = 32.0 + source_temp * (9.0 / 5.0);
    let final_celsius = (source_temp - 32.0) * (5.0 / 9.0);

    println!("{source_temp}C = {final_farenheit}F");
    println!("{source_temp}F = {final_celsius}C");
}
