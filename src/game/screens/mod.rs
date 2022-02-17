use crate::Game;
use macroquad::prelude::*;

pub const BACKGROUND: Color = color_u8!(43, 42, 51, 255);

#[derive(Copy, Clone, Debug)]
pub enum Screen {
    Main,
    Start,
}

impl Screen {
    pub fn draw(&self, game: &Game) -> Screen {
        use Screen::*;

        match *self {
            Main => game.board.draw(&game.players[0]),
            Start => {
                if Screen::draw_start() {
                    return Main;
                }
            }
        }

        *self
    }

    pub fn draw_start() -> bool {
        clear_background(BACKGROUND);
        true
    }
}
