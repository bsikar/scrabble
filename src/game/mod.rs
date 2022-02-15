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

pub struct Game {
    pub tile_bag: Vec<Tile>,
    pub score: u32,
    pub words: HashSet<String>,
    pub board: Board,
}

impl Game {
    pub fn new() -> Game {
        let mut tile_bag = vec![];

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

        Game {
            tile_bag,
            score: 0,
            words,
            board: Board::new(),
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
}
