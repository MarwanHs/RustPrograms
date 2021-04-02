
pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }

    pub fn print(&self) {
        println!("A {} toast with {} seasonal", self.toast, self.seasonal_fruit)
    }
}

pub fn cook_order() {

    let bf = Breakfast::summer("crispy");
    bf.print();
}

