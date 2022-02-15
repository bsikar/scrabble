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

const TRIPLE_WORD: &[(usize, usize)] = &[
    (0, 0),
    (0, 7 * STEP as usize),
    (0, 14 * STEP as usize),
    (7 * STEP as usize, 0),
    (7 * STEP as usize, 14 * STEP as usize),
    (14 * STEP as usize, 0),
    (14 * STEP as usize, 7 * STEP as usize),
    (14 * STEP as usize, 14 * STEP as usize),
];

const DOUBLE_LETTER: &[(usize, usize)] = &[
    (0, 3 * STEP as usize),
    (0, 11 * STEP as usize),
    (3 * STEP as usize, 0),
    (3 * STEP as usize, 14 * STEP as usize),
    (11 * STEP as usize, 0),
    (11 * STEP as usize, 14 * STEP as usize),
    (14 * STEP as usize, 3 * STEP as usize),
    (14 * STEP as usize, 11 * STEP as usize),
    (2 * STEP as usize, 6 * STEP as usize),
    (3 * STEP as usize, 7 * STEP as usize),
    (2 * STEP as usize, 8 * STEP as usize),
    (12 * STEP as usize, 6 * STEP as usize),
    (11 * STEP as usize, 7 * STEP as usize),
    (12 * STEP as usize, 8 * STEP as usize),
    (6 * STEP as usize, 2 * STEP as usize),
    (7 * STEP as usize, 3 * STEP as usize),
    (8 * STEP as usize, 2 * STEP as usize),
    (6 * STEP as usize, 12 * STEP as usize),
    (7 * STEP as usize, 11 * STEP as usize),
    (8 * STEP as usize, 12 * STEP as usize),
    (8 * STEP as usize, 8 * STEP as usize),
    (6 * STEP as usize, 6 * STEP as usize),
    (8 * STEP as usize, 6 * STEP as usize),
    (6 * STEP as usize, 8 * STEP as usize),
];

const TRIPLE_LETTER: &[(usize, usize)] = &[
    (STEP as usize, 5 * STEP as usize),
    (STEP as usize, 9 * STEP as usize),
    (5 * STEP as usize, 5 * STEP as usize),
    (5 * STEP as usize, 9 * STEP as usize),
    (9 * STEP as usize, 5 * STEP as usize),
    (9 * STEP as usize, 9 * STEP as usize),
    (13 * STEP as usize, 5 * STEP as usize),
    (13 * STEP as usize, 9 * STEP as usize),
    (5 * STEP as usize, STEP as usize),
    (9 * STEP as usize, STEP as usize),
    (5 * STEP as usize, 13 * STEP as usize),
    (9 * STEP as usize, 13 * STEP as usize),
];

const DOUBLE_WORD: &[(usize, usize)] = &[
    (STEP as usize, STEP as usize),
    (STEP as usize, 13 * STEP as usize),
    (13 * STEP as usize, STEP as usize),
    (13 * STEP as usize, 13 * STEP as usize),
    (2 * STEP as usize, 2 * STEP as usize),
    (2 * STEP as usize, 12 * STEP as usize),
    (12 * STEP as usize, 2 * STEP as usize),
    (12 * STEP as usize, 12 * STEP as usize),
    (11 * STEP as usize, 3 * STEP as usize),
    (11 * STEP as usize, 11 * STEP as usize),
    (3 * STEP as usize, 3 * STEP as usize),
    (3 * STEP as usize, 11 * STEP as usize),
    (10 * STEP as usize, 4 * STEP as usize),
    (10 * STEP as usize, 10 * STEP as usize),
    (4 * STEP as usize, 4 * STEP as usize),
    (4 * STEP as usize, 10 * STEP as usize),
];

pub struct Board {
    board: [[Option<Tile>; 15]; 15],
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[None; 15]; 15],
        }
    }

    pub fn draw(&self, player: &Player) {
        self.draw_board();
        self.draw_rack(player);
        self.draw_tiles();
    }

    pub fn draw_tiles(&self) {
        for (i, row) in self.board.iter().enumerate() {
            for (_, tile) in row.iter().enumerate() {
                let i = i as f32;
                let x = screen_width() / 2.0 + i * STEP + i * LETTER_SPACE;
                let y = screen_height() - 2.0 * STEP;
                if let Some(tile) = tile {
                    tile.draw(x, y);
                }
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
            tile.draw(x, y);

            if let Some(selected_tile) = player.selected_tile {
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

    pub fn get_rack_tile(&self, x: f32, y: f32, player: &Player) -> Option<usize> {
        let len = player.tiles.len() as f32;
        let x_lower =
            screen_width() / 2.0 - ((len / 2.0) * STEP + ((len - 1.0) / 2.0) * LETTER_SPACE);
        let x_upper = x_lower + len * STEP + (len - 1.0) * LETTER_SPACE;

        let y_lower = screen_height() - 2.0 * STEP;
        let y_upper = y_lower + STEP;

        if (x_lower..=x_upper).contains(&x) && (y_lower..=y_upper).contains(&y) {
            for (i, tile) in player.tiles.iter().enumerate() {
                let i = i as f32;
                let len = player.tiles.len() as f32;
                let x_lower = screen_width() / 2.0
                    - ((len / 2.0) * STEP + ((len - 1.0) / 2.0) * LETTER_SPACE)
                    + i * STEP
                    + i * LETTER_SPACE;
                let y_lower = screen_height() - 2.0 * STEP;
                let x_upper = x_lower + STEP;
                let y_upper = y_lower + STEP;

                if (x_lower..=x_upper).contains(&x) && (y_lower..=y_upper).contains(&y) {
                    return Some(i as usize);
                }
            }
        }

        None
    }

    pub fn draw_board(&self) {
        clear_background(BACKGROUND);

        for i in (200..=screen_width() as usize - 200).step_by(STEP as usize) {
            let i = i as f32;
            for o in (200..=screen_height() as usize - 200).step_by(STEP as usize) {
                let o = o as f32;

                if i == 7.0 * STEP + 200.0 && o == 7.0 * STEP + 200.0 {
                    draw_rectangle(i, o, STEP, STEP, TAN);
                    continue;
                } else if TRIPLE_WORD.contains(&(i as usize - 200, o as usize - 200)) {
                    draw_rectangle(i, o, STEP, STEP, RED);
                    continue;
                } else if DOUBLE_LETTER.contains(&(i as usize - 200, o as usize - 200)) {
                    draw_rectangle(i, o, STEP, STEP, LIGHT_BLUE);
                    continue;
                } else if TRIPLE_LETTER.contains(&(i as usize - 200, o as usize - 200)) {
                    draw_rectangle(i, o, STEP, STEP, BLUE);
                    continue;
                } else if DOUBLE_WORD.contains(&(i as usize - 200, o as usize - 200)) {
                    draw_rectangle(i, o, STEP, STEP, PINK);
                    continue;
                }

                draw_rectangle_lines(i, o, STEP, STEP, 5., DARKGRAY);
            }
        }
    }
}
