use super::StatusTargetRule;

pub struct SingleCharRule {
    text: char,
}

impl SingleCharRule {
    pub fn new(text: char) -> Self {
        Self { text }
    }

    pub fn boxed(text: char) -> Box<Self> {
        Box::new(Self::new(text))
    }
}

impl StatusTargetRule for SingleCharRule {
    fn input(&self, text: char) -> bool {
        self.text == text
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace() {
        let rule = SingleCharRule::new(' ');

        assert!(rule.input(' '))
    }

    #[test]
    fn letter() {
        const EXPECTED_RULE: char = 'a';
        let rule = SingleCharRule::new(EXPECTED_RULE);

        assert!(rule.input(EXPECTED_RULE));
    }

    #[test]
    fn unmatch_diff_letter() {
        const EXPECTED_RULE: char = 'a';
        const UNEXPECTED_TEXT: char = 'b';
        let rule = SingleCharRule::new(EXPECTED_RULE);

        assert_ne!(rule.input(UNEXPECTED_TEXT), true);
    }

    #[test]
    fn unmatch_diff_empyt() {
        const EXPECTED_RULE: char = 'a';
        const UNEXPECTED_TEXT: char = ' ';
        let rule = SingleCharRule::new(EXPECTED_RULE);

        assert_ne!(rule.input(UNEXPECTED_TEXT), true);
    }
}
