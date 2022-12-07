use restaurant::front_of_house::hosting;
use restaurant::serving; // making use of pub-use
// use restaurant; // this is redundant

fn main() {
    hosting::add_to_waitlist();
    serving::serve_order();
    restaurant::visit();
    println!("done!");
}