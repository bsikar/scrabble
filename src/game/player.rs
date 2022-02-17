use super::tile::*;
use ::rand::{rngs::ThreadRng, Rng};
use std::default::Default;

pub struct Player {
    pub tiles: Vec<Tile>,
    pub score: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Player {
        Player {
            tiles: vec![],
            score: 0,
        }
    }

    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }

    pub fn remove_tile(&mut self, tile: Tile) {
        self.tiles.retain(|t| *t != tile);
    }

    pub fn fill_tiles(&mut self, bag: &mut Vec<Tile>, rng: &mut ThreadRng) {
        while self.tiles.len() < 7 {
            let i = rng.gen_range(0..bag.len());
            let tile = bag.remove(i);
            self.add_tile(tile);
        }
    }
}
