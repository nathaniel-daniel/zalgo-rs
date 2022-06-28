/// zalgo chars
mod chars;

/// [`RandOrStatic`] type
mod rand_or_static;

pub use self::rand_or_static::RandOrStatic;
use rand::{
    seq::SliceRandom,
    Rng,
};

/// A builder for a zalgoifier
#[derive(Debug)]
pub struct ZalgoBuilder {
    /// The up limit
    pub up: RandOrStatic,

    /// The down limit
    pub down: RandOrStatic,

    /// The mid limit
    pub mid: RandOrStatic,
}

impl ZalgoBuilder {
    /// Make a new [`ZalgoBuilder`].
    pub fn new() -> Self {
        Self {
            up: RandOrStatic::Rand { start: 0, end: 8 },
            down: RandOrStatic::Rand { start: 0, end: 2 },
            mid: RandOrStatic::Rand { start: 0, end: 8 },
        }
    }

    /// Set the up limits
    pub fn set_up(&mut self, up: RandOrStatic) -> &mut Self {
        self.up = up;
        self
    }

    /// Set the down limits
    pub fn set_down(&mut self, down: RandOrStatic) -> &mut Self {
        self.down = down;
        self
    }

    /// Set the mid limits
    pub fn set_mid(&mut self, mid: RandOrStatic) -> &mut Self {
        self.mid = mid;
        self
    }

    /// Zalgoify a string
    pub fn zalgoify(&self, input: &str) -> String {
        let mut rng = rand::thread_rng();
        let up_num = self.up.generate_num(&mut rng);
        let mid_num = self.mid.generate_num(&mut rng);
        let down_num = self.down.generate_num(&mut rng);

        // Assuming average char len is 2 bytes (TODO: Check this).
        let input_len = input.len();
        let estimated_len =
            (input_len * up_num * 2) + (input_len * mid_num * 2) + (input_len * down_num * 2);

        let mut ret = String::with_capacity(estimated_len);
        for c in input.chars().filter(|c| !is_zalgo_char(*c)) {
            ret.push(c);
            for _ in 0..up_num {
                ret.push(get_rand_char(&mut rng, ZalgoType::Up));
            }

            for _ in 0..mid_num {
                ret.push(get_rand_char(&mut rng, ZalgoType::Mid));
            }

            for _ in 0..down_num {
                ret.push(get_rand_char(&mut rng, ZalgoType::Down));
            }
        }

        ret
    }
}

impl Default for ZalgoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a given char is a zalgo char.
fn is_zalgo_char(c: char) -> bool {
    crate::chars::ZALGO_UP
        .iter()
        .chain(chars::ZALGO_DOWN.iter())
        .chain(chars::ZALGO_MID.iter())
        .any(|&el| el == c)
}

/// Get a random char of the given type.
fn get_rand_char<R>(rng: &mut R, zalgo_type: ZalgoType) -> char
where
    R: Rng,
{
    *zalgo_type
        .get_char_array()
        .choose(rng)
        .expect("zalgo char array is empty")
}

/// Zalgoify the input using default settings
pub fn zalgoify(input: &str) -> String {
    ZalgoBuilder::new().zalgoify(input)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_zalgoify_works() {
        let ret = zalgoify("Hello World!");
        println!("{}", ret);
        assert!(!ret.is_empty());
    }

    #[test]
    fn zalgoify_builder_works() {
        let mut zalgo_builder = ZalgoBuilder::new();
        zalgo_builder
            .set_up(RandOrStatic::Rand { start: 0, end: 100 })
            .set_down(RandOrStatic::Static { value: 0 })
            .set_mid(RandOrStatic::Static { value: 0 });

        let ret = zalgo_builder.zalgoify("Hello World!");

        println!("{}", ret);
        assert!(!ret.is_empty());
    }

    #[test]
    fn zalgo_noop_works() {
        let mut zalgo_builder = ZalgoBuilder::new();
        zalgo_builder
            .set_up(RandOrStatic::Static { value: 0 })
            .set_down(RandOrStatic::Static { value: 0 })
            .set_mid(RandOrStatic::Static { value: 0 });

        let test = "Hello World!";
        assert_eq!(test, zalgo_builder.zalgoify("Hello World!"));
    }
}
