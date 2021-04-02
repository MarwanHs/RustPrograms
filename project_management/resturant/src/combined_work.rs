
pub mod serving;
pub mod cooking;

pub fn fix_order() {
    serving::take_back_wrong_order();
    cooking::cook_order();
    serving::serve_order();
}