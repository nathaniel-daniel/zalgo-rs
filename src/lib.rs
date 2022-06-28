mod chars;

use rand::{
    rngs::ThreadRng,
    seq::SliceRandom,
    Rng,
};

/// Check if a given char is a zalgo char.
fn is_zalgo_char(c: char) -> bool {
    crate::chars::ZALGO_UP
        .iter()
        .chain(chars::ZALGO_DOWN.iter())
        .chain(chars::ZALGO_MID.iter())
        .any(|&el| el == c)
}

/// Zalgoify the input using default settings
pub fn zalgoify(input: &str) -> String {
    Zalgoifier::new().zalgoify(input)
}

pub struct Zalgoifier {
    rng: ThreadRng,
    up: RandOrStatic,
    down: RandOrStatic,
    mid: RandOrStatic,
}

impl Zalgoifier {
    pub fn new() -> Self {
        Zalgoifier {
            rng: rand::thread_rng(),
            up: RandOrStatic::Rand(8),
            down: RandOrStatic::Rand(2),
            mid: RandOrStatic::Rand(8),
        }
    }

    pub fn set_up(&mut self, up: RandOrStatic) {
        self.up = up;
    }

    pub fn set_down(&mut self, down: RandOrStatic) {
        self.down = down;
    }

    pub fn set_mid(&mut self, mid: RandOrStatic) {
        self.mid = mid;
    }

    pub fn get_rand(&mut self, max: usize) -> usize {
        self.rng.gen_range(0..max)
    }

    fn get_rand_char(&mut self, zalgo_type: ZalgoType) -> char {
        *zalgo_type.get_char_array().choose(&mut self.rng).unwrap()
    }

    pub fn get_num(&mut self, val: RandOrStatic) -> usize {
        match val {
            RandOrStatic::Rand(n) => self.get_rand(n),
            RandOrStatic::Static(n) => n,
        }
    }

    pub fn zalgoify(&mut self, input: &str) -> String {
        let up_num = self.get_num(self.up);
        let mid_num = self.get_num(self.mid);
        let down_num = self.get_num(self.down);

        // TODO: This is in bytes. I should probably find the avergae length of a zalgo char and multiply it here.
        let cap = input.len() * up_num + input.len() * mid_num + input.len() * down_num;

        let mut ret = String::with_capacity(cap);
        for c in input.chars().filter(|c| !is_zalgo_char(*c)) {
            ret.push(c);
            for _ in 0..up_num {
                ret.push(self.get_rand_char(ZalgoType::Up));
            }

            for _ in 0..mid_num {
                ret.push(self.get_rand_char(ZalgoType::Mid));
            }

            for _ in 0..down_num {
                ret.push(self.get_rand_char(ZalgoType::Down));
            }
        }

        ret
    }
}

impl Default for Zalgoifier {
    fn default() -> Self {
        Self::new()
    }
}

/// A random value or a static value
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RandOrStatic {
    Rand(usize),
    Static(usize),
}

/// The type of zalgo char
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum ZalgoType {
    Up,
    Down,
    Mid,
}

impl ZalgoType {
    /// Get the char array for the given zalgo type.
    fn get_char_array(self) -> &'static [char] {
        match self {
            ZalgoType::Up => chars::ZALGO_UP,
            ZalgoType::Down => chars::ZALGO_DOWN,
            ZalgoType::Mid => chars::ZALGO_MID,
        }
    }
}
