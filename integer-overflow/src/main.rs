use std::io;

fn main() {
    let x:u8 = 250;

    let mut y_str = String::new();

    println!("Gimme a number between 0 and 255");
    io::stdin()
        .read_line(&mut y_str)
        .expect("error");

    let y: u8 = y_str.trim().parse().expect("blaaaaa");
    
    if y > 5 {
        println!("Brace, brace, brace. z will overlflow.");
    }

    let z: u8 = x + y;

    println!("The value of z = {z}");
}
