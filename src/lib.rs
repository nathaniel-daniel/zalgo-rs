extern crate rand;

pub mod chars;

use rand::{
    rngs::ThreadRng,
    seq::SliceRandom,
    Rng,
};

pub fn is_zalgo_char(c: char) -> bool {
    chars::ZALGO_UP
        .iter()
        .chain(chars::ZALGO_DOWN.iter())
        .chain(chars::ZALGO_MID.iter())
        .any(|&el| el == c)
}

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
        self.rng.gen_range(0, max)
    }

    pub fn get_rand_char(&mut self, zalgo_type: ZalgoType) -> char {
        *zalgo_type.get_char_array().choose(&mut self.rng).unwrap()
    }

    pub fn get_num(&mut self, val: RandOrStatic) -> usize {
        match val {
            RandOrStatic::Rand(n) => self.get_rand(n),
            RandOrStatic::Static(n) => n,
        }
    }

    pub fn zalgoify(&mut self, input: &str) -> String {
        // TODO: Cow
        let mut ret = String::new();
        for c in input.chars() {
            if is_zalgo_char(c) {
                continue;
            }

            ret.push(c);
            for _ in 0..self.get_num(self.up.clone()) {
                ret.push(self.get_rand_char(ZalgoType::Up));
            }

            for _ in 0..self.get_num(self.mid.clone()) {
                ret.push(self.get_rand_char(ZalgoType::Mid));
            }

            for _ in 0..self.get_num(self.down.clone()) {
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

#[derive(Debug, Clone)]
pub enum RandOrStatic {
    Rand(usize),
    Static(usize),
}

#[derive(Debug)]
pub enum ZalgoType {
    Up,
    Down,
    Mid,
}

impl ZalgoType {
    pub fn get_char_array(&self) -> &'static [char] {
        match self {
            ZalgoType::Up => chars::ZALGO_UP,
            ZalgoType::Down => chars::ZALGO_DOWN,
            ZalgoType::Mid => chars::ZALGO_MID,
        }
    }
}
