use macroquad::color::colors::*;
use macroquad::color::Color;
use macroquad::input::{is_key_pressed, KeyCode};
use macroquad::shapes::{draw_line, draw_rectangle, draw_rectangle_lines};
use macroquad::window::clear_background;
use macroquad::window::{next_frame, Conf};
use std::collections::HashSet;
use std::default::Default;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug)]
enum Tile {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Blank,
}

impl Tile {
    fn get_value(&self) -> u8 {
        use Tile::*;

        match self {
            Blank => 0,
            A | E | I | L | N | O | R | S | T | U => 1,
            D | G => 2,
            B | C | M | P => 3,
            F | H | V | W | Y => 4,
            K => 5,
            J | X => 8,
            Q | Z => 10,
        }
    }

    fn get_quantity(&self) -> u8 {
        use Tile::*;

        match self {
            Z | X | Q | K | J => 1,
            Y | W | V | P | M | H | F | C | B | Blank => 2,
            G => 3,
            U | S | L | D => 4,
            T | R | N => 6,
            O => 8,
            I | A => 9,
            E => 12,
        }
    }

    fn iter() -> std::slice::Iter<'static, Tile> {
        use Tile::*;

        [
            A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Blank,
        ]
        .iter()
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Tile {
        use Tile::*;

        match c.to_ascii_lowercase() {
            'a' => A,
            'b' => B,
            'c' => C,
            'd' => D,
            'e' => E,
            'f' => F,
            'g' => G,
            'h' => H,
            'i' => I,
            'j' => J,
            'k' => K,
            'l' => L,
            'm' => M,
            'n' => N,
            'o' => O,
            'p' => P,
            'q' => Q,
            'r' => R,
            's' => S,
            't' => T,
            'u' => U,
            'v' => V,
            'w' => W,
            'x' => X,
            'y' => Y,
            'z' => Z,
            _ => Blank,
        }
    }
}

struct Game {
    tile_bag: Vec<Tile>,
    score: u32,
    words: HashSet<String>,
}

impl Default for Game {
    fn default() -> Self {
        let mut tile_bag = vec![];

        for i in Tile::iter() {
            let quantity = i.get_quantity();
            for _ in 0..quantity {
                tile_bag.push(*i);
            }
        }

        let word_file =
            BufReader::new(File::open("sowpods.txt").expect("failed to open sowpods.txt"));
        let mut words = HashSet::new();
        for word in word_file.lines() {
            words.insert(word.expect("failed to read line"));
        }

        Self {
            tile_bag,
            score: 0,
            words,
        }
    }
}

impl Game {
    fn new() -> Self {
        Default::default()
    }

    fn calculate_score(&self, word: &str) -> Result<u16, ()> {
        if self.words.contains(word) {
            Ok(word
                .chars()
                .map(|x| From::from(x))
                .fold(0, |i, x: Tile| i + x.get_value() as u16))
        } else {
            Err(())
        }
    }

    fn draw_grid(&self) {
        clear_background(Color::from_rgba(43, 42, 51, 255));

        for i in (0..810).step_by(810 / 15) {
            let i = i as f32;
            for o in (0..810).step_by(810 / 15) {
                let o = o as f32;
                draw_rectangle_lines(i, o, 810. / 15., 810. / 15., 5., DARKGRAY);
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let game = Game::new();

    while !is_key_pressed(KeyCode::Escape) {
        game.draw_grid();

        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Scrabble".to_owned(),
        window_width: 810,
        window_height: 810,
        window_resizable: false,
        ..Default::default()
    }
}
