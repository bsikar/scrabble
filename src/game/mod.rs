use ::rand::{self, rngs::ThreadRng};
use hashbrown::HashSet;
use macroquad::prelude::*;
use rust_embed::RustEmbed;
use std::io::{BufRead, BufReader};

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;

mod tile;
use tile::*;

mod board;
use board::*;

mod player;
use player::*;

pub struct Game {
    pub tile_bag: Vec<Tile>,
    pub score: u32,
    pub words: HashSet<String>,
    pub board: Board,
    pub players: Vec<Player>,
    rng: ThreadRng,
}

impl Game {
    pub fn new(num_players: u8) -> Game {
        let mut tile_bag = vec![];
        let mut rng = rand::thread_rng();

        for i in Tile::iter() {
            let quantity = i.get_quantity();
            for _ in 0..quantity {
                tile_bag.push(*i);
            }
        }

        let content: Vec<_> = Asset::get("sowpods.txt").unwrap().data.into();
        let content = content.as_slice();

        let word_file = BufReader::new(content);
        let mut words = HashSet::new();
        for word in word_file.lines() {
            words.insert(word.unwrap());
        }

        let mut players = vec![];
        for _ in 0..num_players {
            let mut player = Player::new();
            player.fill_tiles(&mut tile_bag, &mut rng);
            players.push(player);
        }

        Game {
            tile_bag,
            score: 0,
            words,
            board: Board::new(),
            players,
            rng,
        }
    }

    pub fn calculate_score(&self, word: &str) -> Result<u16, ()> {
        if self.words.contains(word) {
            Ok(word
                .chars()
                .map(|x| From::from(x))
                .fold(0, |i, x: Tile| i + x.get_value() as u16))
        } else {
            Err(())
        }
    }

    pub fn play(&mut self) {
        self.board.draw(&self.players[0]);

        self.handle_movement();
    }

    fn handle_movement(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.select_tile();
            self.place_tile();
        }
    }

    fn place_tile(&mut self) {
        let mouse_pos = mouse_position();
        self.board
            .place_tile(mouse_pos.0, mouse_pos.1, &mut self.players[0]);
    }

    fn select_tile(&mut self) {
        let mouse_pos = mouse_position();
        let selected = self
            .board
            .get_rack_tile(mouse_pos.0, mouse_pos.1, &self.players[0]);

        if selected == self.players[0].selected_tile {
            self.players[0].selected_tile = None;
        } else if selected != None {
            self.players[0].selected_tile = selected;
        }
    }
}
