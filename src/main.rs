use macroquad::prelude::*;

mod game;
use game::*;

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new(2);

    // TODO make a menu for leaving when Escape is pressed
    while !is_key_pressed(KeyCode::Escape) {
        game.play();

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Scrabble".to_owned(),
        window_width: 1200,
        window_height: 1200,
        window_resizable: false,
        ..Default::default()
    }
}
