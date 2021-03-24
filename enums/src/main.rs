
enum Media {
    Game(String, String),
    Movie(String, u8),
    Song(String, String),
}

fn get_media_info(m: &Media) -> (String, String) {
    match m {
        Media::Game(s_1, s_2) => (String::from(s_1), String::from(s_2)),
        Media::Movie(s_1, u_8) => (String::from(s_1), u_8.to_string()),
        Media::Song(s_1, s_2) => (String::from(s_1), String::from(s_2)),
    }
}

fn print_game_song_only_info(m: &Media) {
    match m {
        Media::Game(s_1, s_2) => println!("{}, {}", s_1, s_2),
        Media::Song(s_1, s_2) => println!("{}, {}", s_1, s_2),
        _ => (),
    }
}

enum State {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn get_state(s: &State) -> String {
    match s {
        State::Alabama => String::from("Alabama"),
        State::Alaska => String::from("Alaska")
    }
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        },
        Coin::Nickel => {
            println!("Nickel");
            5
        },
        Coin::Dime => {
            println!("a Dime");
            10
        },
        Coin::Quarter(s) => {
            println!("a Quarter from the state of {}", get_state(s));
            25
        },
    }
}




fn main() {
    let my_game = Media::Game(String::from("game_a"), String::from("developer_a"));
    let my_movie = Media::Movie(String::from("movie_a"), 8);
    let my_song = Media::Song(String::from("song_a"), String::from("singer_a"));

    print_game_song_only_info(&my_game);
    print_game_song_only_info(&my_song);
    print_game_song_only_info(&my_movie);

    let my_media_info = get_media_info(&my_movie);
    println!("movie: {}, rating: {}", my_media_info.0, my_media_info.1);


    let some_number = Some(1);
    let some_string = Some("a string");

    match some_number{
        Some(v) => println!("{}", v),
        None => {},
    }

    match some_string{
        Some(v) => println!("{}", v),
        None => {},
    }
    
    let mut absent_number: Option<i32>; // this line can be omitted, type will be understood from 'Some(5)' as Option<i32>
    absent_number = None;

    match absent_number{
        Some(v) => {
            Some(10);
            println!("{}", v)
        },
        None => println!("None value"),
    }

    absent_number = Some(5);

    match absent_number {
        Some(v) => println!("{}", v),
        None => {},
    }

    let my_coin = Coin::Dime;
    value_in_cents(&my_coin);

    let my_other_coin = Coin::Quarter(State::Alaska);
    value_in_cents(&my_other_coin);

    if let Coin::Quarter(state) = my_coin {
        println!("State quarter from {}!", get_state(&state));
    } else {
        println!("this coin is not a quarter")
    }

    if let Coin::Quarter(state) = my_other_coin {
        println!("State quarter from {}!", get_state(&state));
    } else {
        println!("this coin is not a quarter")
    }
}
