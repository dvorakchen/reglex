mod alpha;
mod alpha_lowercase;
mod alpha_uppercase;
mod alphanumeric_underline;
mod digit;
mod single_char;

pub use alpha::AlphaRule;
pub use alpha_lowercase::AlphaLowercaseRule;
pub use alpha_uppercase::AlphaUppercaseRule;
pub use alphanumeric_underline::AlphanumericUnderlineRule;
pub use digit::Digit;
pub use single_char::SingleCharRule;

pub trait StatusTargetRule {
    fn input(&self, text: char) -> bool;
}
