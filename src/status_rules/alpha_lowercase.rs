use super::StatusTargetRule;

/// match single alphabet, digit, underline,
/// equal to [a-ZA-Z0-9]
pub struct AlphaLowercaseRule;

impl AlphaLowercaseRule {
    pub fn boxed() -> Box<Self> {
        Box::new(Self)
    }
}

impl StatusTargetRule for AlphaLowercaseRule {
    fn input(&self, text: char) -> bool {
        text.is_ascii_lowercase()
    }
    
    fn get_id(&self) -> usize {
        1_005
    }
    fn clone(&self) -> Box<dyn StatusTargetRule> {
        Box::new(Self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha() {
        const EXPECTED: char = 'a';
        let rule = AlphaLowercaseRule::boxed();

        assert!(rule.input(EXPECTED));
    }

    #[test]
    fn alpha_uppercase() {
        const EXPECTED: char = 'A';
        let rule = AlphaLowercaseRule::boxed();

        assert_eq!(rule.input(EXPECTED), false);
    }

    #[test]
    fn digit() {
        const EXPECTED: char = '0';
        let rule = AlphaLowercaseRule::boxed();

        assert_eq!(rule.input(EXPECTED), false);
    }

    #[test]
    fn punctuation() {
        const EXPECTED: char = '_';
        let rule = AlphaLowercaseRule::boxed();

        assert_eq!(rule.input(EXPECTED), false);
    }
}
