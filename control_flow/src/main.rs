

fn main() {

    let condition = true;
    if condition {
        println!("if statement condition will only evaluate if true or false, wont convert from other
        types");
    }

    let my_num = 1;
    if my_num != 0 {
        println!("condition(1!=0) is true");
    }

    let mut number = if condition { 5 } else { 6 };
    println!("condition is {}, The value of number is: {}", condition, number);

    let condition = false;
    number = if condition { 5 } else { 6 };
    println!("condition is {}, The value of number is: {}", condition, number);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        print!("{} ", number);
        number -= 1;
    }
    println!("");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        print!("{} ", a[index]);
        index += 1;
    }
    println!("");

    for element in a.iter() {
        print!("{} ", element);
    }
    println!("");

    for number in (1..4).rev() {
        print!("{} ", number);
    }
    println!("");
}
