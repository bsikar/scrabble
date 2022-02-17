use crate::game::TAN;
use crate::Game;
use macroquad::prelude::*;
use std::process::exit;

pub const BACKGROUND: Color = color_u8!(43, 42, 51, 255);

#[derive(Copy, Clone, Debug)]
pub enum Screen {
    Main,
    Exit,
    Start,
}

impl Screen {
    pub fn draw(&self, game: &Game) -> Screen {
        use Screen::*;

        match *self {
            Main => game.board.draw(&game.players[0]),
            Exit => {
                if Screen::confirm_exit() {
                    return Main;
                }
            }
            Start => {
                if Screen::draw_start() {
                    return Main;
                }
            }
        }

        *self
    }

    pub fn confirm_exit() -> bool {
        clear_background(BACKGROUND);
        let x = if screen_height() > screen_width() {
            screen_width()
        } else {
            screen_height()
        };

        let x = x / 20.0;

        let top_text = "Are you sure you want to exit?";
        let top_text_size = measure_text(top_text, None, x as u16, 1.0);
        let middle_text = "Press Escape if you want to exit.";
        let middle_text_size = measure_text(middle_text, None, x as u16, 1.0);
        let bottom_text = "Press Any Other Key to Continue.";
        let bottom_text_size = measure_text(bottom_text, None, x as u16, 1.0);

        draw_text(
            top_text,
            screen_width() / 2.0 - top_text_size.width / 2.0,
            screen_height() / 2.0 - top_text_size.height / 2.0,
            x,
            TAN,
        );

        draw_text(
            middle_text,
            (screen_width() - middle_text_size.width) / 2.0,
            ((screen_height() - middle_text_size.height) / 2.0) + x,
            x,
            TAN,
        );

        draw_text(
            bottom_text,
            (screen_width() - bottom_text_size.width) / 2.0,
            ((screen_height() - bottom_text_size.height) / 2.0) + 2.0 * x,
            x,
            TAN,
        );

        if is_key_pressed(KeyCode::Escape) {
            exit(0);
        }

        matches!(get_last_key_pressed(), Some(_))
    }

    pub fn draw_start() -> bool {
        clear_background(BACKGROUND);
        let x = if screen_height() > screen_width() {
            screen_width()
        } else {
            screen_height()
        };

        let top_text = "Welcome to Scrabble!";
        let top_text_size = measure_text(top_text, None, x as u16 / 10, 1.0);
        let bottom_text = "Press any key to start.";
        let bottom_text_size = measure_text(bottom_text, None, x as u16 / 20, 1.0);

        draw_text(
            top_text,
            screen_width() / 2.0 - top_text_size.width / 2.0,
            screen_height() / 2.0 - top_text_size.height / 2.0,
            x / 10.0,
            TAN,
        );

        draw_text(
            bottom_text,
            (screen_width() - bottom_text_size.width) / 2.0,
            ((screen_height() - bottom_text_size.height) / 2.0) + x / 20.0,
            x / 20.0,
            TAN,
        );

        get_last_key_pressed().is_some()
    }
}
