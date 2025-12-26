//! # docs Package
//! `docs` is a sample package showcasing rust sourcecode documentation, doc testing, and workspaces
//! This comment is for the contained item and will appear at the top of the page.


/// Adds two numbers together
/// # Examples
/// This will be tested when running `cargo test`
/// ```
/// let x = 5;
/// let y = 5;
/// let answer = docs::add(x,y);
/// assert_eq!(10,answer);
/// ```
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


/// This module is nested in our lib.rs file
/// We will Export it to be available at the crate level for ease of use. 
/// 
pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple
    }

}

pub mod utils {
    use crate::kinds::*;

    use crate::kinds::{PrimaryColor, SecondaryColor};
    /// Combines two primary colors in equal amounts to create a secondary color
    pub fn mix(
        c1: PrimaryColor,
        c2: PrimaryColor
    ) -> SecondaryColor {
        SecondaryColor::Green
    }
}

// By using pub use, we can allow users to access the mix() function and enums more readily via
// docs::mix() and docs::PrimaryColor

pub use self::utils::mix;
pub use crate::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;

// pub use is distinct from use for a crate, as it allows an alias to be made publically available
// this way, we can create public apis more readily for others to use



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
