fn main() {
    // mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 10;
    println!("The value of x is: {x}");

    // shadowing
    let y = 2;
    let y = y + 1;
    println!("The value of y is {y}");
}
