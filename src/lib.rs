pub mod dfa;
pub mod nfa;
pub mod nfa_executor;
mod status_rules;

static mut CURRENT_ID: usize = 0;

pub(crate) fn new_id() -> usize {
    unsafe {
        CURRENT_ID += 1;

        CURRENT_ID
    }
}
