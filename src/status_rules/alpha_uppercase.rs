use super::StatusTargetRule;

/// match single alphabet, digit, underline,
/// equal to [a-ZA-Z0-9]
pub struct AlphaUppercaseRule;

impl AlphaUppercaseRule {
    pub fn boxed() -> Box<Self> {
        Box::new(Self)
    }
}

impl StatusTargetRule for AlphaUppercaseRule {
    fn input(&self, text: char) -> bool {
        text.is_ascii_uppercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_lowercase() {
        const EXPECTED: char = 'a';
        let rule = AlphaUppercaseRule::boxed();

        assert_eq!(rule.input(EXPECTED), false);
    }

    #[test]
    fn alpha_uppercase() {
        const EXPECTED: char = 'A';
        let rule = AlphaUppercaseRule::boxed();

        assert_eq!(rule.input(EXPECTED), true);
    }

    #[test]
    fn digit() {
        const EXPECTED: char = '0';
        let rule = AlphaUppercaseRule::boxed();

        assert_eq!(rule.input(EXPECTED), false);
    }

    #[test]
    fn punctuation() {
        const EXPECTED: char = '_';
        let rule = AlphaUppercaseRule::boxed();

        assert_eq!(rule.input(EXPECTED), false);
    }
}
