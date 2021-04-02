

mod front_of_house;
mod back_of_house;

pub use crate::front_of_house::{hosting, serving};
pub use crate::back_of_house::cooking;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    cooking::cook_order();
    serving::serve_order();
}

