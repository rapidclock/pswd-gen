use rand::seq::SliceRandom;
use rand::{rngs::StdRng, SeedableRng};

use super::config::PasswordConfig;

pub struct PasswordGenerator<'a> {
    generator: StdRng,
    conf: &'a PasswordConfig,
}

type Category = (Vec<char>, StdRng);
type WeightedCategory = (Category, usize);

fn get_lowercase_category(weight: usize) -> WeightedCategory {
    (
        (
            (b'a'..b'z').map(char::from).collect::<Vec<_>>(),
            StdRng::from_entropy(),
        ),
        weight,
    )
}

fn get_uppercase_category(weight: usize) -> WeightedCategory {
    (
        (
            (b'A'..b'Z').map(char::from).collect::<Vec<_>>(),
            StdRng::from_entropy(),
        ),
        weight,
    )
}

fn get_digit_category(weight: usize) -> WeightedCategory {
    (
        (
            (b'0'..b'9').map(char::from).collect::<Vec<_>>(),
            StdRng::from_entropy(),
        ),
        weight,
    )
}

fn get_symbol_category(symbols: Option<&Vec<char>>, weight: usize) -> WeightedCategory {
    ((symbols.unwrap().clone(), StdRng::from_entropy()), weight)
}

impl<'b> PasswordGenerator<'b> {
    pub fn new(config: &'b PasswordConfig) -> Self {
        PasswordGenerator {
            generator: StdRng::from_entropy(),
            conf: config,
        }
    }

    fn rand_char_from_byte_range(&mut self, category: &Category) -> char {
        category.0.choose(&mut self.generator).copied().unwrap()
    }

    pub fn generate(&mut self) -> String {
        let mut password = String::with_capacity(self.conf.length);
        let mut ranges: Vec<WeightedCategory> = Vec::with_capacity(4);
        if self.conf.caps {
            ranges.push(get_uppercase_category(6));
        }
        if self.conf.letters {
            ranges.push(get_lowercase_category(6));
        }
        if self.conf.numbers {
            ranges.push(get_digit_category(5));
        }
        if self.conf.symbols.is_some() {
            ranges.push(get_symbol_category(self.conf.symbols.as_ref(), 3));
        }
        for _ in 0..self.conf.length {
            let category = &ranges
                .choose_weighted(&mut self.generator, |item| item.1)
                .unwrap()
                .0;
            password.push(self.rand_char_from_byte_range(category));
        }
        password
    }
}
