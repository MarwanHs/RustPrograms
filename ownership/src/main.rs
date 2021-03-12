



fn main() {

    let var1 = "Hello There";

    let mut s1 = String::from(var1);
    s1.push_str(", General Kenobi");

    println!("s1 as made of String type has requested memory allocation");
    println!("this is the last usage of s1 data -> {}, meaning at scope end it will be invalid", s1);

    let s2 = String::from("a");
    let mut s3 =s2; // moving`

    s3.push('b');
    println!("s2 cannot be used as we moved the value to s3: {}", s3);
    println!("so now only s3 points to data, when scope ends it will be freed, s2 is already invalid when we moved");
    
    println!("this can be prevented with deep copy, called clone in rust");
    let s2 = String::from("a");
    let s3 = s2.clone();

    println!("this is content of new s2: {}, this is content of new s3: {}, they have same value but they are not same memory", s2, s3);


    let s = String::from("a moving string value");
    println!("{} was creatd", s);
    takes_ownership(s);
    println!("the moving string value that was moved cannot be used in this scope any more as ownership passed to another scope");

    println!("this can be prevented by using references");

    let s = String::from("a moving string value by reference");
    let s_len = calculate_length(&s);

    println!("the size of the string s wich was created in this scope and passed by reference is {}", s_len);
}

fn takes_ownership(some_string: String) {
    println!("{} was moved to takes_ownership", some_string);
}

fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}
