

struct Game{
    name: String,
    year: u16,
    rating: u8
}

impl Game {

    fn build(name: String, year: u16, rating: u8) -> Game {
        Game {
            name,
            year,
            rating
        }
    }

    fn print(&self) {
        println!("the game is called {}, and it was created in {}, with rating of {}",
        self.name, self.year, self.rating);
    }

    fn set_year(&mut self, year: u16){
        self.year = year
    }

    fn set_rating(&mut self, rating: u8){
        self.rating = rating
    }

    fn newer_and_better(&self, other: &Game) -> bool {
        self.year > other.year && self.rating > other.rating
    }
}

fn main() {
    
    let game1 = Game {
        year: 2021,
        rating: 8,
        name: String::from("game1")
    };
    println!("game1: name: {}, year: {}, rating: {}", game1.name, game1.year, game1.rating);


    let mut game2 = Game::build(String::from("game2"), 2015, 7);
    game2.year = 2018;
    println!("game2: name: {}, year: {}, rating: {}", game2.name, game2.year, game2.rating);


    let mut game3 = Game {
        year: 2017,
        ..game1
    };
    game3.set_year(2019);
    game3.set_rating(7);
    game3.print();

    let game4 = Game::build(String::from("newer game"), 2020, 8);
    println!("is game4 newer and better than game3? {}", game4.newer_and_better(&game3));

    // Tuple Struct
    struct Movie(String, u16, u16, String);
    let mut movie1 = Movie(String::from("Terminator"), 1989, 92, String::from("US"));
    movie1.2 = 98;
    println!("movie1: name: {}, year: {}, rating: {}, country:{}", movie1.0, movie1.1, movie1.2, movie1.3);
}
