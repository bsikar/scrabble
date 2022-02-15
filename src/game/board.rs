use macroquad::color_u8;
use macroquad::prelude::*;

const RED: Color = color_u8!(255, 104, 83, 255);
const LIGHT_BLUE: Color = color_u8!(183, 208, 204, 255);
const BLUE: Color = color_u8!(44, 150, 181, 255);
const PINK: Color = color_u8!(237, 177, 167, 255);
const TAN: Color = color_u8!(198, 192, 168, 255);
const BACKGROUND: Color = color_u8!(43, 42, 51, 255);

const STEP: f32 = 810.0 / 15.0;

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

pub struct Board;

impl Board {
    pub fn new() -> Board {
        Board
    }

    pub fn draw(&self) {
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
