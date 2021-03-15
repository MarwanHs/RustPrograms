

struct Game{
    name: String,
    year: u16,
    rating: u8
}

fn game_set_year(game: &mut Game, year: u16){
    game.year = year
}

fn game_build_game(name: String, year: u16, rating: u8) -> Game {
    Game {
        name,
        year,
        rating
    }
}

fn main() {
    
    let mut game1 = Game {
        year: 2021,
        rating: 8,
        name: String::from("The Game")
    };
    game_set_year(&mut game1, 2020);
    println!("game1: name: {}, year: {}, rating: {}", game1.name, game1.year, game1.rating);

    let mut game2 = game_build_game(String::from("a nice game"), 2015, 9);
    game2.year = 2018;
    println!("game2: name: {}, year: {}, rating: {}", game2.name, game2.year, game2.rating);

    let game3 = Game {
        year: 2017,
        ..game1
    };
    println!("game3: name: {}, year: {}, rating: {}", game3.name, game3.year, game3.rating);

    struct Movie(String, u16, u16, String);
    let movie1 = Movie(String::from("Terminator"), 1989, 92, String::from("US"));
    println!("movie1: name: {}, year: {}, rating: {}, country:{}", movie1.0, movie1.1, movie1.2, movie1.3);
}
