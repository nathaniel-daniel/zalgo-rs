/// zalgo chars
mod chars;

/// [`RandOrStatic`] type
mod rand_or_static;

pub use self::rand_or_static::RandOrStatic;
use rand::seq::SliceRandom;

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
    #[inline]
    pub fn set_up(&mut self, up: impl Into<RandOrStatic>) -> &mut Self {
        self.up = up.into();
        self
    }

    /// Set the down limits
    #[inline]
    pub fn set_down(&mut self, down: impl Into<RandOrStatic>) -> &mut Self {
        self.down = down.into();
        self
    }

    /// Set the mid limits
    #[inline]
    pub fn set_mid(&mut self, mid: impl Into<RandOrStatic>) -> &mut Self {
        self.mid = mid.into();
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
                let c = *self::chars::ZALGO_UP
                    .choose(&mut rng)
                    .expect("`ZALGO_UP` is empty");
                ret.push(c);
            }

            for _ in 0..mid_num {
                let c = *self::chars::ZALGO_MID
                    .choose(&mut rng)
                    .expect("`ZALGO_MID` is empty");
                ret.push(c);
            }

            for _ in 0..down_num {
                let c = *self::chars::ZALGO_DOWN
                    .choose(&mut rng)
                    .expect("`ZALGO_DOWN` is empty");
                ret.push(c);
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

/// Zalgoify the input using default settings.
pub fn zalgoify(input: &str) -> String {
    ZalgoBuilder::new().zalgoify(input)
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
        zalgo_builder.set_up(0..100).set_down(0).set_mid(0);

        let ret = zalgo_builder.zalgoify("Hello World!");

        println!("{}", ret);
        assert!(!ret.is_empty());
    }

    #[test]
    fn zalgo_noop_works() {
        let mut zalgo_builder = ZalgoBuilder::new();
        zalgo_builder.set_up(0).set_down(0).set_mid(0);

        let test = "Hello World!";
        assert_eq!(test, zalgo_builder.zalgoify("Hello World!"));
    }
}
