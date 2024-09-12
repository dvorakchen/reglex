use super::StatusTargetRule;

/// match single alphabet, digit, underline,
/// equal to [a-ZA-Z0-9_]
pub struct AlphanumericUnderlineRule;

impl AlphanumericUnderlineRule {
    pub fn boxed() -> Box<Self> {
        Box::new(Self)
    }
}

impl StatusTargetRule for AlphanumericUnderlineRule {
    fn input(&self, text: char) -> bool {
        text.is_ascii_alphanumeric() || text == '_'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha() {
        const EXPECTED: char = 'a';
        let rule = AlphanumericUnderlineRule::boxed();

        assert!(rule.input(EXPECTED));
    }

    #[test]
    fn digit() {
        const EXPECTED: char = '1';
        let rule = AlphanumericUnderlineRule::boxed();

        assert!(rule.input(EXPECTED));
    }

    #[test]
    fn underline() {
        const EXPECTED: char = '_';
        let rule = AlphanumericUnderlineRule::boxed();

        assert!(rule.input(EXPECTED));
    }

    #[test]
    fn punctuation_except_underline() {
        const EXPECTED: [char; 3] = ['(', ')', '!'];
        let rule = AlphanumericUnderlineRule::boxed();

        for item in EXPECTED {
            assert_eq!(rule.input(item), false);
        }
    }
}
