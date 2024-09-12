use super::StatusTargetRule;

/// match single alphabet, digit, underline,
/// equal to [a-ZA-Z0-9_]
pub struct AlphaRule;

impl AlphaRule {
    pub fn boxed() -> Box<Self> {
        Box::new(Self)
    }
}

impl StatusTargetRule for AlphaRule {
    fn input(&self, text: char) -> bool {
        text.is_ascii_alphabetic()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha() {
        const EXPECTED: char = 'a';
        let rule = AlphaRule::boxed();

        assert!(rule.input(EXPECTED));
    }

    #[test]
    fn digit() {
        const EXPECTED: char = '0';
        let rule = AlphaRule::boxed();

        assert_eq!(rule.input(EXPECTED), false);
    }

    #[test]
    fn punctuation() {
        const EXPECTED: char = '_';
        let rule = AlphaRule::boxed();

        assert_eq!(rule.input(EXPECTED), false);
    }
}
