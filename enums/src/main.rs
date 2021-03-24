
enum Media {
    Game(String, String),
    Movie(String, u8),
    Song(String, String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}


fn main() {
    let my_game = Media::Game(String::from("game_a"), String::from("developer_a"));
    let my_movie = Media::Movie(String::from("movie_a"), 8);
    let my_song = Media::Song(String::from("song_a"), String::from("singer0_a"));

    let some_number = Some(5);
    let some_string = Some("a string");

    
    let mut absent_number: Option<i32>; // this line can be omitted, type will be understood from 'Some(5)' as Option<i32>
    absent_number = None;

    match absent_number.as_mut() {
        Some(v) => println!("{}", v),
        None => println!("None value"),
    }

    absent_number = Some(5);

    match absent_number.as_mut() {
        Some(v) => println!("{}", v),
        None => {},
    }
}
