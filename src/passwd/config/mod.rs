#[derive(Debug)]
pub struct PasswordConfig {
    pub letters: bool,
    pub numbers: bool,
    pub caps: bool,
    pub length: usize,
    pub symbols: Option<Vec<char>>
}

impl PasswordConfig {
    pub fn new() -> Self {
        PasswordConfig::default()
    }

    pub fn with_params(required_length: usize, lower_case: bool,
                       upper_case: bool, numbers: bool) -> PasswordConfig {
        if !(required_length > 0 && (lower_case || upper_case || numbers)) {
            panic!("The required length has to be greater than 0 and atlease one of \
            lower/upper/digits flags must be specified");
        }
        PasswordConfig {
            letters: lower_case,
            numbers,
            caps: upper_case,
            symbols: None,
            length: required_length,
        }
    }
}

impl Default for PasswordConfig {
    fn default() -> Self {
        PasswordConfig {
            letters: true,
            numbers: false,
            caps: false,
            symbols: None,
            length: 5,
        }
    }
}
