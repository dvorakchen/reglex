use std::{cell::RefCell, rc::Rc};

use crate::nfa::{StatusType, NFA};

pub struct NFAExecutor<T>
where
    T: AsRef<str>,
{
    nfa: NFA,
    text: T,
}

impl<T> NFAExecutor<T>
where
    T: AsRef<str>,
{
    pub fn new(nfa: NFA, text: T) -> Self {
        Self { nfa, text }
    }

    pub fn test(&self) -> bool {
        let mut status = self.nfa.get_start();

        let text = self.text.as_ref().trim();
        for t in text.chars() {
            let tem = Rc::clone(&status);
            let status_ref = RefCell::borrow(&tem);

            if let Some(next) = status_ref.next_skip_empty(t) {
                status = Rc::clone(&next);
            } else {
                break;
            }
        }

        let status = RefCell::borrow(&status);
        status.get_type() == &StatusType::End
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::nfa::NFA;

    use super::NFAExecutor;

    #[test]
    fn nfa_executor_single_success() {
        const RULE: char = 'a';
        const TEST_SUCCESS: &str = "a";

        let nfa = NFA::with(RULE);

        let ne = NFAExecutor::new(nfa, TEST_SUCCESS);

        let is_matched = ne.test();
        assert!(is_matched);
    }

    #[test]
    fn nfa_executor_single_fail() {
        const RULE: char = 'a';
        const TEST_FAIL: &str = "b";

        let nfa = NFA::with(RULE);

        let ne = NFAExecutor::new(nfa, TEST_FAIL);

        let is_matched = ne.test();
        assert!(!is_matched);
    }

    #[test]
    fn nfa_executor_string_success() {
        const RULE: &str = "abcdef";
        const TEST_SUCCESS: &str = "abcdef";

        let nfa = NFA::from_str(RULE).unwrap();

        let ne = NFAExecutor::new(nfa, TEST_SUCCESS);

        let is_matched = ne.test();
        assert!(is_matched);
    }

    #[test]
    fn nfa_executor_string_fail() {
        const RULE: &str = "abcdef";
        const TEST_FAIL: &str = "abcd";

        let nfa = NFA::from_str(RULE).unwrap();

        let ne = NFAExecutor::new(nfa, TEST_FAIL);

        let is_matched = ne.test();
        assert!(!is_matched);
    }

    #[test]
    fn nfa_executor_string_fail_2() {
        const RULE: &str = "abc";
        const TEST_FAIL: &str = "abcd";

        let nfa = NFA::from_str(RULE).unwrap();

        let ne = NFAExecutor::new(nfa, TEST_FAIL);

        let is_matched = ne.test();
        assert!(is_matched);
    }

    #[test]
    fn nfa_executor_string_bracket_w_success() {
        const RULE: &str = r"a[\w]b";
        const TEST_SUCC_1: [&str; 3] = ["aab", "a0b", "a_b"];

        for i in TEST_SUCC_1 {
            let nfa = NFA::from_str(RULE).unwrap();
            let ne = NFAExecutor::new(nfa, i);
            assert!(ne.test());
        }
    }

    #[test]
    fn nfa_executor_string_bracket_w_fail() {
        const RULE: &str = r"a[\w]b";
        const TEST_SUCC: &str = "ab";

        let nfa = NFA::from_str(RULE).unwrap();

        let ne = NFAExecutor::new(nfa, TEST_SUCC);

        assert_eq!(ne.test(), false);
    }

    #[test]
    fn nfa_executor_string_bracket_az_success() {
        const RULE: &str = r"a[a-z]b";
        const TEST_SUCC_1: [&str; 3] = ["aab", "abb", "azb"];

        for i in TEST_SUCC_1 {
            let nfa = NFA::from_str(RULE).unwrap();
            let ne = NFAExecutor::new(nfa, i);
            assert!(ne.test());
        }
    }

    #[test]
    fn nfa_executor_string_bracket_az_fail() {
        const RULE: &str = r"a[a-z]b";
        const TEST_SUCC_1: [&str; 3] = ["aAb", "a0b", "a_b"];

        for i in TEST_SUCC_1 {
            let nfa = NFA::from_str(RULE).unwrap();
            let ne = NFAExecutor::new(nfa, i);
            assert_eq!(ne.test(), false);
        }
    }

    #[test]
    fn nfa_executor_string_bracket_alphanumeric_success() {
        const RULE: &str = r"a[a-zA-Z0-9]b";
        const TEST_SUCC_1: [&str; 3] = ["aab", "aAb", "a0b"];

        for i in TEST_SUCC_1 {
            let nfa = NFA::from_str(RULE).unwrap();
            let ne = NFAExecutor::new(nfa, i);
            assert!(ne.test());
        }
    }
}
