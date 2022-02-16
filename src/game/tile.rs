use macroquad::prelude::*;

use crate::game::board::{LETTER_SIZE, STEP};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Tile {
    A, B, C, D, E, F, G,
    H, I, J, K, L, M, N,
    O, P, Q, R, S, T, U,
    V, W, X, Y, Z, Blank,
}

impl Tile {
    pub fn get_value(&self) -> u8 {
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

    pub fn get_quantity(&self) -> u8 {
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

    pub fn iter() -> std::slice::Iter<'static, Tile> {
        use Tile::*;

        [
            A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z, Blank,
        ]
        .iter()
    }

    pub fn draw(&self, x: f32, y: f32, background: Color) {
        let text = self.into();
        let text_size = measure_text(text, None, LETTER_SIZE as u16, 1.0);
        draw_rectangle(x, y, STEP, STEP, background);
        draw_text(
            text,
            x + STEP / 2.0 - text_size.width / 2.0,
            y + STEP / 2.0 + text_size.height / 2.0,
            LETTER_SIZE,
            DARKGRAY,
        );
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

impl From<&Tile> for &str {
    fn from(t: &Tile) -> &'static str {
        use Tile::*;

        match t {
            A => "A",
            B => "B",
            C => "C",
            D => "D",
            E => "E",
            F => "F",
            G => "G",
            H => "H",
            I => "I",
            J => "J",
            K => "K",
            L => "L",
            M => "M",
            N => "N",
            O => "O",
            P => "P",
            Q => "Q",
            R => "R",
            S => "S",
            T => "T",
            U => "U",
            V => "V",
            W => "W",
            X => "X",
            Y => "Y",
            Z => "Z",
            _ => "",
        }
    }
}
