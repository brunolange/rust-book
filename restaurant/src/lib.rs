pub mod front_of_house;

pub use front_of_house::hosting;
pub use front_of_house::serving;

pub fn visit() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    serving::serve_order();
    serving::take_payment();
}
