use macroquad::color_u8;
use macroquad::prelude::*;

use super::player::*;
use super::tile::*;

use crate::game::BACKGROUND;

pub const RED: Color = color_u8!(255, 104, 83, 255);
pub const LIGHT_BLUE: Color = color_u8!(183, 208, 204, 255);
pub const BLUE: Color = color_u8!(44, 150, 181, 255);
pub const PINK: Color = color_u8!(237, 177, 167, 255);
pub const TAN: Color = color_u8!(198, 192, 168, 255);

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

pub struct Consts {
    pub step: f32,
    pub letter_size: f32,
    pub letter_space: f32,
    pub selected_tile_glow_thickness: f32,
    pub offset: f32,
    pub board_upper: (f32, f32),
    pub board_lower: (f32, f32),
    pub rack_upper: (f32, f32),
    pub rack_lower: (f32, f32),
}

impl Consts {
    fn new() -> Consts {
        let x = if screen_height() > screen_width() {
            screen_width()
        } else {
            screen_height()
        };
        let offset = x / 6.0;
        let step = x * (9.0 / 200.0);
        let letter_size = step;
        let letter_space = x / 24.0;
        let selected_tile_glow_thickness = x / 60.0;

        let board_lower_x = step + screen_width() / 2.0 - step * 7.0;
        let board_upper_x = 14.0 * step + screen_width() / 2.0 - step * 7.0;
        let board_lower_y = step + screen_height() / 2.0 - step * 7.0;
        let board_upper_y = 14.0 * step + screen_height() / 2.0 - step * 7.0;
        let board_upper = (board_upper_x, board_upper_y);
        let board_lower = (board_lower_x, board_lower_y);

        let len = 15.0;
        let rack_lower_x =
            screen_width() / 2.0 - ((len / 2.0) * step + ((len - 2.0) / 2.0) * letter_space);
        let rack_upper_x = rack_lower_x + len * step + (len - 1.0) * letter_space;
        let rack_lower_y = screen_height() / 2.0 + step * 7.0 + offset / 2.0;
        let rack_upper_y = rack_lower_y + step;
        let rack_upper = (rack_upper_x, rack_upper_y);
        let rack_lower = (rack_lower_x, rack_lower_y);

        Consts {
            offset,
            step,
            letter_size,
            letter_space,
            selected_tile_glow_thickness,
            board_upper,
            board_lower,
            rack_upper,
            rack_lower,
        }
    }

    pub fn update(&mut self, player: &Player) {
        let x = if screen_height() > screen_width() {
            screen_width()
        } else {
            screen_height()
        };
        self.offset = x / 6.0;
        self.step = x * (9.0 / 200.0);
        self.letter_size = self.step;
        self.letter_space = x / 24.0;
        self.selected_tile_glow_thickness = x / 60.0;

        let board_lower_x = self.step + screen_width() / 2.0 - self.step * 8.0;
        let board_upper_x = 14.0 * self.step + screen_width() / 2.0 - self.step * 6.0;
        let board_lower_y = self.step + screen_height() / 2.0 - self.step * 8.0;
        let board_upper_y = 14.0 * self.step + screen_height() / 2.0 - self.step * 6.0;
        self.board_upper = (board_upper_x, board_upper_y);
        self.board_lower = (board_lower_x, board_lower_y);

        let len = player.tiles.len() as f32;
        let rack_lower_x = screen_width() / 2.0
            - ((len / 2.0) * self.step + ((len - 2.0) / 2.0) * self.letter_space);
        let rack_upper_x = rack_lower_x + len * self.step + (len - 1.0) * self.letter_space;
        let rack_lower_y = screen_height() / 2.0 + self.step * 7.0 + self.offset / 2.0;
        let rack_upper_y = rack_lower_y + self.step;
        self.rack_upper = (rack_upper_x, rack_upper_y);
        self.rack_lower = (rack_lower_x, rack_lower_y);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectedTile {
    None,
    Board(usize, usize),
    Rack(usize),
}

pub struct Board {
    board: [[Option<Tile>; 15]; 15],
    pub selected_tile: SelectedTile,
    pub consts: Consts,
}

impl Board {
    pub fn new() -> Board {
        Board {
            board: [[None; 15]; 15],
            selected_tile: SelectedTile::None,
            consts: Consts::new(),
        }
    }

    pub fn draw(&self, player: &Player) {
        clear_background(BACKGROUND);
        self.draw_tiles();
        self.draw_rack(player);
    }

    pub fn draw_tiles(&self) {
        for (i, row) in self.board.iter().enumerate() {
            for (o, tile) in row.iter().enumerate() {
                let x = o as f32 * self.consts.step + screen_width() / 2.0 - self.consts.step * 7.0;
                let y =
                    i as f32 * self.consts.step + screen_height() / 2.0 - self.consts.step * 7.0;

                let i = i as u8;
                let o = o as u8;
                let tile = tile.unwrap_or(Tile::Blank);

                if i == 7 && o == 7 {
                    tile.draw(x, y, TAN, &self.consts);
                } else if TRIPLE_WORD.contains(&(i, o)) {
                    tile.draw(x, y, RED, &self.consts);
                } else if DOUBLE_LETTER.contains(&(i, o)) {
                    tile.draw(x, y, LIGHT_BLUE, &self.consts);
                } else if TRIPLE_LETTER.contains(&(i, o)) {
                    tile.draw(x, y, BLUE, &self.consts);
                } else if DOUBLE_WORD.contains(&(i, o)) {
                    tile.draw(x, y, PINK, &self.consts);
                } else if let Some(tile) = self.board[i as usize][o as usize] {
                    tile.draw(x, y, TAN, &self.consts);
                }

                if self.selected_tile == SelectedTile::Board(i as usize, o as usize) {
                    draw_rectangle_lines(
                        x,
                        y,
                        self.consts.step,
                        self.consts.step,
                        self.consts.selected_tile_glow_thickness,
                        GOLD,
                    );
                }

                draw_rectangle_lines(x, y, self.consts.step, self.consts.step, 5., DARKGRAY);
            }
        }
    }

    pub fn draw_rack(&self, player: &Player) {
        for (i, tile) in player.tiles.iter().enumerate() {
            let i = i as f32;
            let len = player.tiles.len() as f32;
            let x = screen_width() / 2.0
                - ((len / 2.0) * self.consts.step + ((len - 2.0) / 2.0) * self.consts.letter_space)
                + i * self.consts.step
                + i * self.consts.letter_space;
            let y = screen_height() / 2.0 + self.consts.step * 7.0 + self.consts.offset / 2.0;
            tile.draw(x, y, TAN, &self.consts);

            if let SelectedTile::Rack(selected_tile) = self.selected_tile {
                if selected_tile == i as usize {
                    let x = x - self.consts.selected_tile_glow_thickness;
                    let y = y - self.consts.selected_tile_glow_thickness;
                    let w = self.consts.step + 2.0 * self.consts.selected_tile_glow_thickness;
                    let h = self.consts.step + 2.0 * self.consts.selected_tile_glow_thickness;
                    draw_rectangle_lines(
                        x,
                        y,
                        w,
                        h,
                        self.consts.selected_tile_glow_thickness,
                        GOLD,
                    );
                }
            }
        }
    }

    pub fn get_board_tile(&self, x: f32, y: f32) -> Option<(usize, usize)> {
        if (self.consts.board_lower.0..=self.consts.board_upper.0).contains(&x)
            && (self.consts.board_lower.1..=self.consts.board_upper.1).contains(&y)
        {
            let row = ((y - self.consts.board_lower.1) / self.consts.step) as usize;
            let col = ((x - self.consts.board_lower.0) / self.consts.step) as usize;

            if let Some(_tile) = self.board[row][col] {
                return Some((row, col));
            }
        }

        None
    }

    pub fn get_rack_tile(&self, x: f32, y: f32, player: &Player) -> Option<usize> {
        if (self.consts.rack_lower.0..=self.consts.rack_upper.0).contains(&x)
            && (self.consts.rack_lower.1..=self.consts.rack_upper.1).contains(&y)
        {
            for (i, _) in player.tiles.iter().enumerate() {
                let i = i as f32;
                let lower_x =
                    self.consts.rack_lower.0 + i * self.consts.step + i * self.consts.letter_space;
                let upper_x = lower_x + self.consts.step;

                if (lower_x..=upper_x).contains(&x)
                    && (self.consts.rack_lower.1..=self.consts.rack_upper.1).contains(&y)
                {
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

        if (self.consts.board_lower.0..=self.consts.board_upper.0).contains(&x)
            && (self.consts.board_lower.1..=self.consts.board_upper.1).contains(&y)
        {
            if let SelectedTile::Rack(selected_tile) = self.selected_tile {
                let tile = player.tiles.remove(selected_tile);
                let row = ((y - self.consts.board_lower.1) / self.consts.step as f32) as usize;
                let col = ((x - self.consts.board_lower.0) / self.consts.step as f32) as usize;

                self.board[row][col] = Some(tile);
                self.selected_tile = SelectedTile::None;
            }
        }
    }

    pub fn remove_tile_from_board(&mut self, x: f32, y: f32, player: &mut Player) {
        if let SelectedTile::None = self.selected_tile {
            return;
        }

        if (self.consts.board_lower.0..=self.consts.board_upper.0).contains(&x)
            && (self.consts.board_lower.1..=self.consts.board_upper.1).contains(&y)
        {
            let row = ((y - self.consts.board_lower.1) / self.consts.step as f32) as usize;
            let col = ((x - self.consts.board_lower.0) / self.consts.step as f32) as usize;
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

        if (self.consts.board_lower.0..=self.consts.board_upper.0).contains(&x)
            && (self.consts.board_lower.1..=self.consts.board_upper.1).contains(&y)
        {
            let row = ((y - self.consts.board_lower.1) / self.consts.step as f32) as usize;
            let col = ((x - self.consts.board_lower.0) / self.consts.step as f32) as usize;
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

        if (self.consts.rack_lower.0..=self.consts.rack_upper.0).contains(&x)
            && (self.consts.rack_lower.1..=self.consts.rack_upper.1).contains(&y)
        {
            for (i, _) in player.tiles.iter().enumerate() {
                let i = i as f32;
                let len = player.tiles.len() as f32;
                let lower_x = screen_width() / 2.0
                    - ((len / 2.0) * self.consts.step
                        + ((len - 1.0) / 2.0) * self.consts.letter_space)
                    + i * self.consts.step
                    + i * self.consts.letter_space;
                let upper_x = lower_x + self.consts.step;

                if (lower_x..=upper_x).contains(&x)
                    && (self.consts.rack_lower.1..=self.consts.rack_upper.1).contains(&y)
                {
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
