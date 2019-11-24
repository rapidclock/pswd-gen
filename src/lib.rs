pub mod passwd;
pub use passwd::config as conf;
use passwd::gen;

pub fn make_new_password(config: conf::PasswordConfig) -> String {
    let mut generator = gen::PasswordGenerator::new(&config);
    generator.generate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conf;

    #[test]
    fn test_fixed_len_only_lowercase() {
        let mut config = conf::PasswordConfig::new();
        let pass_len: usize = 5;
        config.length = pass_len;
        config.letters = true;
        config.numbers = false;
        config.caps = false;
        config.symbols = None;
        assert_eq!(pass_len, make_new_password(config).len());
    }

    #[test]
    fn test_only_lowercase() {
        let req_len = 8;
        let config = conf::PasswordConfig::with_params(req_len, true, false, false, None);
        let generated_password = make_new_password(config);
        assert_eq!(req_len, generated_password.len());
        generated_password
            .chars()
            .for_each(|c| assert!(c.is_alphabetic() && c.is_lowercase()));
    }

    #[test]
    fn test_only_uppercase() {
        let req_len = 8;
        let config = conf::PasswordConfig::with_params(req_len, false, true, false, None);
        let generated_password = make_new_password(config);
        assert_eq!(req_len, generated_password.len());
        generated_password
            .chars()
            .for_each(|c| assert!(c.is_alphabetic() && c.is_uppercase()));
    }

    #[test]
    fn test_only_digits() {
        let req_len = 8;
        let config = conf::PasswordConfig::with_params(req_len, false, false, true, None);
        let generated_password = make_new_password(config);
        assert_eq!(req_len, generated_password.len());
        generated_password
            .chars()
            .for_each(|c| assert!(c.is_ascii_digit()));
    }

    #[test]
    fn test_only_lowercase_and_uppercase() {
        let req_len = 8;
        let config = conf::PasswordConfig::with_params(req_len, true, true, false, None);
        let generated_password = make_new_password(config);
        assert_eq!(req_len, generated_password.len());
        generated_password
            .chars()
            .for_each(|c| assert!(c.is_alphabetic() && (c.is_lowercase() || c.is_uppercase())));
    }

    #[test]
    fn test_any_but_no_caps() {
        let req_len = 12;
        let config = conf::PasswordConfig::with_params(req_len, true, false, true, None);
        let generated_password = make_new_password(config);
        assert_eq!(req_len, generated_password.len());
        generated_password
            .chars()
            .for_each(|c| assert!(c.is_alphanumeric() && !c.is_uppercase()));
    }

    #[test]
    fn test_alphanumeric() {
        let req_len = 8;
        let config = conf::PasswordConfig::with_params(req_len, true, true, true, None);
        let generated_password = make_new_password(config);
        assert_eq!(req_len, generated_password.len());
        generated_password
            .chars()
            .for_each(|c| assert!(c.is_alphanumeric()));
    }

    #[test]
    #[should_panic]
    fn test_no_length_should_panic() {
        let req_len = 0;
        conf::PasswordConfig::with_params(req_len, true, true, true, None);
    }

    #[test]
    #[should_panic]
    fn test_valid_length_no_char_type_selected_should_panic() {
        let req_len = 8;
        conf::PasswordConfig::with_params(req_len, false, false, false, None);
    }

    #[test]
    #[ignore]
    fn test_very_long_alphanumeric() {
        let req_len = 1_000_000_000;
        let config = conf::PasswordConfig::with_params(req_len, true, true, true, None);
        let generated_password = make_new_password(config);
        assert_eq!(req_len, generated_password.len());
        generated_password
            .chars()
            .for_each(|c| assert!(c.is_alphanumeric()));
    }
}
