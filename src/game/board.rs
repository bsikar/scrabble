use macroquad::color_u8;
use macroquad::prelude::*;

use super::player::*;
use super::tile::*;

pub const RED: Color = color_u8!(255, 104, 83, 255);
pub const LIGHT_BLUE: Color = color_u8!(183, 208, 204, 255);
pub const BLUE: Color = color_u8!(44, 150, 181, 255);
pub const PINK: Color = color_u8!(237, 177, 167, 255);
pub const TAN: Color = color_u8!(198, 192, 168, 255);
pub const BACKGROUND: Color = color_u8!(43, 42, 51, 255);

pub const STEP: f32 = 810.0 / 15.0;
pub const LETTER_SIZE: f32 = STEP;
pub const LETTER_SPACE: f32 = 50.0;
pub const SELECTED_TILE_GLOW_THICKNESS: f32 = 20.0;

const TRIPLE_WORD: &[(u8, u8)] = &[
    (0, 0),
    (0, 7),
    (0, 14),
    (7, 0),
    (7, 14),
    (14, 0),
    (14, 7),
    (14, 14),
];

const DOUBLE_LETTER: &[(u8, u8)] = &[
    (0, 3),
    (0, 11),
    (3, 0),
    (3, 14),
    (11, 0),
    (11, 14),
    (14, 3),
    (14, 11),
    (2, 6),
    (3, 7),
    (2, 8),
    (12, 6),
    (11, 7),
    (12, 8),
    (6, 2),
    (7, 3),
    (8, 2),
    (6, 12),
    (7, 11),
    (8, 12),
    (8, 8),
    (6, 6),
    (8, 6),
    (6, 8),
];

const TRIPLE_LETTER: &[(u8, u8)] = &[
    (1, 5),
    (1, 9),
    (5, 5),
    (5, 9),
    (9, 5),
    (9, 9),
    (13, 5),
    (13, 9),
    (5, 1),
    (9, 1),
    (5, 13),
    (9, 13),
];

const DOUBLE_WORD: &[(u8, u8)] = &[
    (1, 1),
    (1, 13),
    (13, 1),
    (13, 13),
    (2, 2),
    (2, 12),
    (12, 2),
    (12, 12),
    (11, 3),
    (11, 11),
    (3, 3),
    (3, 11),
    (10, 4),
    (10, 10),
    (4, 4),
    (4, 10),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedTile {
    None,
    Board(usize, usize),
    Rack(usize),
}

pub struct Board {
    board: [[Option<Tile>; 15]; 15],
    pub selected_tile: SelectedTile,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[None; 15]; 15],
            selected_tile: SelectedTile::None,
        }
    }

    pub fn draw(&self, player: &Player) {
        self.draw_tiles();
        self.draw_rack(player);
    }

    pub fn draw_tiles(&self) {
        clear_background(BACKGROUND);

        for (i, row) in self.board.iter().enumerate() {
            for (o, tile) in row.iter().enumerate() {
                let x = o as f32 * STEP + 200.0;
                let y = i as f32 * STEP + 200.0;
                let i = i as u8;
                let o = o as u8;
                let tile = tile.unwrap_or(Tile::Blank);

                if i == 7 && o == 7 {
                    tile.draw(x, y, TAN);
                } else if TRIPLE_WORD.contains(&(i, o)) {
                    tile.draw(x, y, RED);
                } else if DOUBLE_LETTER.contains(&(i, o)) {
                    tile.draw(x, y, LIGHT_BLUE);
                } else if TRIPLE_LETTER.contains(&(i, o)) {
                    tile.draw(x, y, BLUE);
                } else if DOUBLE_WORD.contains(&(i, o)) {
                    tile.draw(x, y, PINK);
                } else if let Some(tile) = self.board[i as usize][o as usize] {
                    tile.draw(x, y, TAN);
                }

                if self.selected_tile == SelectedTile::Board(i as usize, o as usize) {
                    draw_rectangle_lines(x, y, STEP, STEP, SELECTED_TILE_GLOW_THICKNESS, GOLD);
                }

                draw_rectangle_lines(x, y, STEP, STEP, 5., DARKGRAY);
            }
        }
    }

    pub fn draw_rack(&self, player: &Player) {
        for (i, tile) in player.tiles.iter().enumerate() {
            let i = i as f32;
            let len = player.tiles.len() as f32;
            let x = screen_width() / 2.0
                - ((len / 2.0) * STEP + ((len - 1.0) / 2.0) * LETTER_SPACE)
                + i * STEP
                + i * LETTER_SPACE;
            let y = screen_height() - 2.0 * STEP;
            tile.draw(x, y, TAN);

            if let SelectedTile::Rack(selected_tile) = self.selected_tile {
                if selected_tile == i as usize {
                    let x = x - SELECTED_TILE_GLOW_THICKNESS;
                    let y = y - SELECTED_TILE_GLOW_THICKNESS;
                    let w = STEP + 2.0 * SELECTED_TILE_GLOW_THICKNESS;
                    let h = STEP + 2.0 * SELECTED_TILE_GLOW_THICKNESS;
                    draw_rectangle_lines(x, y, w, h, SELECTED_TILE_GLOW_THICKNESS, GOLD);
                }
            }
        }
    }

    pub fn get_board_tile(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        let lower_x = 200.0;
        let lower_y = 200.0;
        let upper_x = screen_width() - 200.0 + 10.0;
        let upper_y = screen_height() - 200.0 + 10.0;

        if (lower_x..=upper_x).contains(&x) && (lower_y..=upper_y).contains(&y) {
            let row = ((y - lower_y) / STEP) as usize;
            let col = ((x - lower_x) / STEP) as usize;

            if let Some(_tile) = self.board[row][col] {
                return Some((row, col));
            }
        }

        None
    }

    pub fn get_rack_tile(&self, x: f32, y: f32, player: &Player) -> Option<usize> {
        let len = player.tiles.len() as f32;
        let lower_x =
            screen_width() / 2.0 - ((len / 2.0) * STEP + ((len - 1.0) / 2.0) * LETTER_SPACE);
        let upper_x = lower_x + len * STEP + (len - 1.0) * LETTER_SPACE;

        let lower_y = screen_height() - 2.0 * STEP;
        let upper_y = lower_y + STEP;

        if (lower_x..=upper_x).contains(&x) && (lower_y..=upper_y).contains(&y) {
            for (i, _) in player.tiles.iter().enumerate() {
                let i = i as f32;
                let len = player.tiles.len() as f32;
                let lower_x = screen_width() / 2.0
                    - ((len / 2.0) * STEP + ((len - 1.0) / 2.0) * LETTER_SPACE)
                    + i * STEP
                    + i * LETTER_SPACE;
                let lower_y = screen_height() - 2.0 * STEP;
                let upper_x = lower_x + STEP;
                let upper_y = lower_y + STEP;

                if (lower_x..=upper_x).contains(&x) && (lower_y..=upper_y).contains(&y) {
                    return Some(i as usize);
                }
            }
        }

        None
    }

    pub fn place_tile(&mut self, x: f32, y: f32, player: &mut Player) {
        if let SelectedTile::None = self.selected_tile {
            return;
        }

        let lower_x = 200.0;
        let lower_y = 200.0;
        let upper_x = screen_width() - 200.0 + 10.0;
        let upper_y = screen_height() - 200.0 + 10.0;

        if (lower_x..=upper_x).contains(&x) && (lower_y..=upper_y).contains(&y) {
            if let SelectedTile::Rack(selected_tile) = self.selected_tile {
                let tile = player.tiles.remove(selected_tile);
                let row = ((y - lower_y) / STEP as f32) as usize;
                let col = ((x - lower_x) / STEP as f32) as usize;

                self.board[row][col] = Some(tile);
                self.selected_tile = SelectedTile::None;
            }
        }
    }

    pub fn remove_tile_from_board(&mut self, x: f32, y: f32, player: &mut Player) {
        if let SelectedTile::None = self.selected_tile {
            return;
        }

        let lower_x = 200.0;
        let lower_y = 200.0;
        let upper_x = screen_width() - 200.0 + 10.0;
        let upper_y = screen_height() - 200.0 + 10.0;

        if (lower_x..=upper_x).contains(&x) && (lower_y..=upper_y).contains(&y) {
            let row = ((y - lower_y) / STEP as f32) as usize;
            let col = ((x - lower_x) / STEP as f32) as usize;
            if self.board[row][col].is_some() {
                player.tiles.push(self.board[row][col].unwrap());
                self.board[row][col] = None;

                if SelectedTile::Board(row, col) == self.selected_tile {
                    self.selected_tile = SelectedTile::None;
                }
            }
        }
    }

    pub fn swap_tile_on_board(&mut self, x: f32, y: f32) {
        if let SelectedTile::None = self.selected_tile {
            return;
        }

        let lower_x = 200.0;
        let lower_y = 200.0;
        let upper_x = screen_width() - 200.0 + 10.0;
        let upper_y = screen_height() - 200.0 + 10.0;

        if (lower_x..=upper_x).contains(&x) && (lower_y..=upper_y).contains(&y) {
            let row = ((y - lower_y) / STEP as f32) as usize;
            let col = ((x - lower_x) / STEP as f32) as usize;
            if let Some(tile) = self.board[row][col] {
                if let SelectedTile::Board(selected_row, selected_col) = self.selected_tile {
                    let tmp = self.board[selected_row][selected_col];
                    self.board[selected_row][selected_col] = Some(tile);
                    self.board[row][col] = tmp;
                    self.selected_tile = SelectedTile::Board(row, col);
                }
            }
        }
    }

    pub fn swap_tile_on_rack(&mut self, x: f32, y: f32, player: &mut Player) {
        if let SelectedTile::None = self.selected_tile {
            return;
        }

        let len = player.tiles.len() as f32;
        let lower_x =
            screen_width() / 2.0 - ((len / 2.0) * STEP + ((len - 1.0) / 2.0) * LETTER_SPACE);
        let upper_x = lower_x + len * STEP + (len - 1.0) * LETTER_SPACE;

        let lower_y = screen_height() - 2.0 * STEP;
        let upper_y = lower_y + STEP;

        if (lower_x..=upper_x).contains(&x) && (lower_y..=upper_y).contains(&y) {
            for (i, _) in player.tiles.iter().enumerate() {
                let i = i as f32;
                let len = player.tiles.len() as f32;
                let lower_x = screen_width() / 2.0
                    - ((len / 2.0) * STEP + ((len - 1.0) / 2.0) * LETTER_SPACE)
                    + i * STEP
                    + i * LETTER_SPACE;
                let lower_y = screen_height() - 2.0 * STEP;
                let upper_x = lower_x + STEP;
                let upper_y = lower_y + STEP;

                if (lower_x..=upper_x).contains(&x) && (lower_y..=upper_y).contains(&y) {
                    if let SelectedTile::Rack(tile) = self.selected_tile {
                        player.tiles.swap(i as usize, tile);
                        self.selected_tile = SelectedTile::Rack(i as usize);
                        return;
                    }
                }
            }
        }
    }
}
