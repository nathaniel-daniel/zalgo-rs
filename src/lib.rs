#![cfg_attr(feature = "no-unsafe", forbid(unsafe_code))]

/// zalgo chars
mod chars;

/// [`RandOrStatic`] type
mod rand_or_static;

use self::chars::is_zalgo_char;
pub use self::rand_or_static::RandOrStatic;
use rand::{
    seq::SliceRandom,
    SeedableRng,
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
    #[inline]
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
        let mut push_buf = [0; 4];
        let mut rng = rand::rngs::SmallRng::from_entropy();
        let up_num = self.up.generate_num(&mut rng);
        let mid_num = self.mid.generate_num(&mut rng);
        let down_num = self.down.generate_num(&mut rng);

        // Assuming average char len is 2 bytes (TODO: Check this).
        // input_len  * (1 + up + down + mid)
        let input_len = input.len();
        let bytes_per_char = 1 + ((up_num + down_num + mid_num) * 2);
        let estimated_len = input_len * bytes_per_char;

        // let mut ret = String::with_capacity(estimated_len);
        let mut ret = Vec::with_capacity(estimated_len);
        for c in input.chars().filter(|c| !is_zalgo_char(*c)) {
            if c.len_utf8() == 1 {
                ret.push(c as u8);
            } else {
                for b in c.encode_utf8(&mut push_buf).as_bytes() {
                    ret.push(*b);
                }
            }

            for _ in 0..up_num {
                /*
                let c = *self::chars::ZALGO_UP
                    .choose(&mut rng)
                    .expect("`ZALGO_UP` is empty");
                string_push_buf(&mut ret, &mut push_buf, c);
                */
                let bytes = *self::chars::ZALGO_UP_ENCODED
                    .choose(&mut rng)
                    .expect("`ZALGO_UP_ENCODED` is empty");
                for b in bytes {
                    ret.push(b);
                }
            }

            for _ in 0..mid_num {
                /*
                let c = *self::chars::ZALGO_MID
                    .choose(&mut rng)
                    .expect("`ZALGO_MID` is empty");
                string_push_buf(&mut ret, &mut push_buf, c);
                */
                let bytes = *self::chars::ZALGO_MID_ENCODED
                    .choose(&mut rng)
                    .expect("`ZALGO_MID_ENCODED` is empty");
                for b in bytes {
                    ret.push(b);
                }
            }

            for _ in 0..down_num {
                let bytes = *self::chars::ZALGO_DOWN_ENCODED
                    .choose(&mut rng)
                    .expect("`ZALGO_DOWN_ENCODED` is empty"); 
                for b in bytes {
                    ret.push(b);
                }
                /*
                let c = *self::chars::ZALGO_DOWN
                    .choose(&mut rng)
                    .expect("`ZALGO_DOWN` is empty");
                string_push_buf(&mut ret, &mut push_buf, c);
                */
            }
        }

        #[cfg(not(feature = "no-unsafe"))]
        unsafe {
            String::from_utf8_unchecked(ret)
        }

        #[cfg(feature = "no-unsafe")]
        String::from_utf8(ret).expect("vec should be utf8")
    }
}

impl Default for ZalgoBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Zalgoify the input using default settings.
#[inline]
pub fn zalgoify(input: &str) -> String {
    ZalgoBuilder::new().zalgoify(input)
}

/*
/// Push a char to a String using a buffer to encode utf8.
///
/// This is much faster for multi-byte chars (like zalgo chars) than `push`.
/// This is because Rust will emit a `memcpy` for multi-byte chars when pushing them to the `String`,
/// which is incredibly inefficient.
///
/// Instead, we manually push the UTF8 bytes 1 by 1 to the inner `Vec`.
/// It is impossible to implement this in safe code.
#[cfg(not(feature = "no-unsafe"))]
#[inline]
fn string_push_buf(string: &mut String, buf: &mut [u8], c: char) {
    // Safety:
    // Only UTF8 bytes are appended to the Vec.
    unsafe {
        let vec = string.as_mut_vec();
        for b in c.encode_utf8(buf).as_bytes() {
            vec.push(*b);
        }
    }
}

/// Push a char to a String using a buffer to encode utf8.
///
/// This is much faster for multi-byte chars (like zalgo chars) than `push`.
/// This is because Rust will emit a `memcpy` for multi-byte chars when pushing them to the `String`,
/// which is incredibly inefficient.
///
/// We cannot safely avoid this `memcpy` in safe code,
/// so we mitigate it by assuming all chars passed to this function are multi-byte.
#[cfg(feature = "no-unsafe")]
#[inline]
fn string_push_buf(string: &mut String, buf: &mut [u8], c: char) {
    string.push_str(c.encode_utf8(buf));
}
*/

#[cfg(test)]
mod test {
    use super::*;
    use std::time::Instant;

    /// Check if a given char is a zalgo char.
    ///
    /// This is an old impl
    fn is_zalgo_char_version_2(c: char) -> bool {
        if crate::chars::ZALGO_UP.binary_search(&c).is_ok() {
            return true;
        }

        if crate::chars::ZALGO_DOWN.binary_search(&c).is_ok() {
            return true;
        }

        if crate::chars::ZALGO_MID.binary_search(&c).is_ok() {
            return true;
        }

        false
    }

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

    #[test]
    fn zalgo_bench() {
        let data = "Hello World!".repeat(12);

        let start = Instant::now();
        zalgoify(&data);
        let elapsed = start.elapsed();
        println!("Time: {:?}", elapsed);
    }

    #[test]
    fn test_is_zalgo_char() {
        for i in 0..u32::MAX {
            if let Ok(c) = char::try_from(i) {
                let is_zalgo_char_version_2_result = is_zalgo_char_version_2(c);
                let is_zalgo_char_result = is_zalgo_char(c);
                assert!(
                    is_zalgo_char_version_2_result == is_zalgo_char_result,
                    "failed on {:x?} ({:b}), expected {}, got {}",
                    c,
                    u32::from(c),
                    is_zalgo_char_version_2_result,
                    is_zalgo_char_result,
                );
            }
        }
    }
}
