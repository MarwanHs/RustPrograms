
use rand::Rng;

fn hello_world() -> String {
    let hw_string = String::from("Hello World!!");
    hw_string
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn my_bool() -> bool {
    true   
}

fn my_random_tuple() -> (i32, bool, f64) {
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    rand_tuple    
}

fn main() {
    let hw_string = hello_world();
    let z = add(50, 20);
    println!("{}, {}, {}", hw_string, z, my_bool());
    let my_tuple = my_random_tuple();
    println!("{}, {}, {}", my_tuple.0, my_tuple.1, my_tuple.2);
}
