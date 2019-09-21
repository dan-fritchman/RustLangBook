//! # crates
//!
//! A crate about trying to make crates.
//! From *The Rust Programming Language*, chapter 14.
//! Stuff included:
//!
//! * (This) documentation generation!
//! * Other stuff
//!


/// Add one
///
/// # Panics
///
/// None
///
/// # Errors
///
/// None
///
/// # Examples
///
/// ```
/// assert_eq!(6,crates::add_one(5))
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub mod kinds {
    /// Primary Colors
    pub enum PrimaryColor { Red, Yellow, Blue }

    /// Secondary Colors
    pub enum SecondaryColor { Orange, Green, Purple }
}

pub mod utils {
    use super::kinds::*;

    /// Mix two PrimaryColors into a SecondaryColor
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        return SecondaryColor::Purple;
    }
}

// Promote these to top-level scope
pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

