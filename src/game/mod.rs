use ::rand;
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

mod screens;
use screens::*;

pub struct Game {
    pub tile_bag: Vec<Tile>,
    pub score: u32,
    pub words: HashSet<String>,
    pub board: Board,
    pub players: [Player; 2],
    pub screen: Screen,
}

impl Game {
    pub fn new() -> Game {
        let screen = Screen::Start;
        Screen::draw_start();

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

        let mut players = [Player::new(), Player::new()];

        for player in players.iter_mut() {
            player.fill_tiles(&mut tile_bag, &mut rng);
        }

        Game {
            tile_bag,
            score: 0,
            words,
            board: Board::new(),
            players,
            screen,
        }
    }

    pub fn play(&mut self) {
        self.screen = self.screen.draw(self);
        self.board.consts.update(&self.players[0]);

        self.handle_movement();
    }

    fn handle_movement(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            self.select_tile_from_rack();
            self.select_tile_from_board();
            self.place_tile();
            self.swap_tile_on_board();
            self.swap_tile_on_rack();
        } else if is_mouse_button_pressed(MouseButton::Right) {
            self.select_tile_from_board();
            self.remove_tile_from_board();
        }
    }

    fn remove_tile_from_board(&mut self) {
        let mouse_pos = mouse_position();

        self.board
            .remove_tile_from_board(mouse_pos.0, mouse_pos.1, &mut self.players[0]);
    }

    fn swap_tile_on_board(&mut self) {
        let mouse_pos = mouse_position();

        self.board.swap_tile_on_board(mouse_pos.0, mouse_pos.1);
    }

    fn swap_tile_on_rack(&mut self) {
        let mouse_pos = mouse_position();

        self.board
            .swap_tile_on_rack(mouse_pos.0, mouse_pos.1, &mut self.players[0]);
    }

    fn place_tile(&mut self) {
        let mouse_pos = mouse_position();
        self.board
            .place_tile(mouse_pos.0, mouse_pos.1, &mut self.players[0]);
    }

    fn select_tile_from_board(&mut self) {
        let mouse_pos = mouse_position();
        let selected = self.board.get_board_tile(mouse_pos.0, mouse_pos.1);

        if selected.is_none() {
            return;
        }
        let selected = selected.unwrap();

        if let SelectedTile::Board(x, y) = self.board.selected_tile {
            if selected == (x, y) {
                self.board.selected_tile = SelectedTile::None;
            }
        } else {
            self.board.selected_tile = SelectedTile::Board(selected.0, selected.1);
        }
    }

    fn select_tile_from_rack(&mut self) {
        let mouse_pos = mouse_position();
        let selected = self
            .board
            .get_rack_tile(mouse_pos.0, mouse_pos.1, &self.players[0]);

        if selected.is_none() {
            return;
        }
        let selected = selected.unwrap();

        if let SelectedTile::Rack(tile) = self.board.selected_tile {
            if selected == tile {
                self.board.selected_tile = SelectedTile::None;
            }
        } else {
            self.board.selected_tile = SelectedTile::Rack(selected);
        }
    }
}
