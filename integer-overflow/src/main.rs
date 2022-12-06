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


    let q = if true { () };

    println!("The value of q = {:?}", q);

    let w = 42;
    f(w);
    println!("The value of w = {w}");

    let mut xs = [1, 2, 3, 4, 5];
    g(&mut xs);
    println!("The value of xs = {:?}", xs);

    let x = 5;
    let y = x;

    println!("The value of x = {x}");
    println!("The value of y = {y}");
}

fn f(mut x: i32) -> i32 {
    x = x + 1;
    x
}

fn g(xs: &mut [i32; 5]){
    xs[0] = 100;
    println!("xs = {:?}", xs);
}