mod snake;
mod game;
mod canvas;
mod food;
mod error;

pub mod snake_game {
    use std::error::Error;
    use crate::game::GameInfo;

    pub fn run() {
        let mut game_info = initial_game();
        game_info.format_output();
        start_game(game_info).expect("start game error!");
    }

    pub fn initial_game() -> GameInfo { GameInfo::default() }

    pub fn start_game(mut game_info: GameInfo) -> Result<bool, Box<dyn Error>> {
        game_info.start()
    }
}
