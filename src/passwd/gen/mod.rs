use rand::{rngs::StdRng, Rng, SeedableRng};
use super::config::PasswordConfig;

type Byte = u8;

pub struct PasswordGenerator <'a> {
    generator: StdRng,
    conf: &'a PasswordConfig,
}

struct ByteRange {
    low: Byte,
    high: Byte,
}

//struct GenDist {
//    lower_case_count: usize,
//    upper_case_count: usize,
//    digit_count: usize,
//}

const LOWER_CASE_CHARS: ByteRange = ByteRange{low: b'a', high: b'z'};
const UPPER_CASE_CHARS: ByteRange = ByteRange{low: b'A', high: b'Z'};
const DIGITS: ByteRange = ByteRange{low: b'0', high: b'9'};

impl <'b> PasswordGenerator<'b> {
    pub fn new(config : &'b PasswordConfig) -> Self {
        PasswordGenerator{generator: StdRng::from_entropy(), conf: config}
    }

    fn rand_char_from_byte_range(&mut self, range: &ByteRange) -> char {
        char::from(self.generator.gen_range(range.low, range.high))
    }

    pub fn generate(&mut self) -> String {
        let mut password = String::with_capacity(self.conf.length);
        let mut ranges = Vec::with_capacity(3);
        if self.conf.caps {
            ranges.push(UPPER_CASE_CHARS);
        }
        if self.conf.letters {
            ranges.push(LOWER_CASE_CHARS);
        }
        if self.conf.numbers {
            ranges.push(DIGITS);
        }
        for _ in 0..self.conf.length {
            let selected_range_index : usize = self.generator.gen::<usize>() % ranges.len();
            password.push(self.rand_char_from_byte_range(ranges.get(selected_range_index).unwrap()));
        }
        password
    }
}