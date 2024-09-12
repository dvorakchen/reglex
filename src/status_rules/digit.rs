use super::StatusTargetRule;

/// match single alphabet, digit, underline,
/// equal to [a-ZA-Z0-9]
pub struct Digit;

impl Digit {
    pub fn boxed() -> Box<Self> {
        Box::new(Self)
    }
}

impl StatusTargetRule for Digit {
    fn input(&self, text: char) -> bool {
        text.is_ascii_digit()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha() {
        const EXPECTED: char = 'a';
        let rule = Digit::boxed();

        assert_eq!(rule.input(EXPECTED), false);
    }

    #[test]
    fn alpha_uppercase() {
        const EXPECTED: char = 'A';
        let rule = Digit::boxed();

        assert_eq!(rule.input(EXPECTED), false);
    }

    #[test]
    fn digit() {
        const EXPECTED: char = '0';
        let rule = Digit::boxed();

        assert_eq!(rule.input(EXPECTED), true);
    }

    #[test]
    fn punctuation() {
        const EXPECTED: char = '_';
        let rule = Digit::boxed();

        assert_eq!(rule.input(EXPECTED), false);
    }
}
