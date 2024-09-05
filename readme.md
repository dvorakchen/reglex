# Reglex

I try to make a 'lex' for Rust
Not completed yet

# Features

Now it only supporting the text literally, like 'abcd123+\*', +\*? is unavailable

## Usage

```rust

const RULE: &str = "abcdef";
const TEST_SUCCESS: &str = "abcdef";

let nfa = NFA::from_str(RULE).unwrap();

let ne = NFAExecutor::new(nfa, TEST_SUCCESS);

let is_matched = ne.test();
assert!(is_matched);
```
