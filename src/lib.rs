//! A tiny lib crate for tiny dice.
#![crate_name = "tiny_die"]
#![no_std]
use core::fmt;
use rand::Rng;

/// Contains the `Die` struct and its functions.
pub struct Die {
    /// A die has sides, that's what makes it a die.
    pub sides: u8,
}

impl Die {
    /// Returns a die with `sides` number of sides.
    /// # Arguments
    ///
    /// `sides` - optional; a `u8` representing the number of sides on the die
    ///
    /// # Example
    ///
    /// ```rust
    /// use tiny_die::Die;
    /// let dee_ten = Die::new(10);
    /// ```
    pub fn new(sides: u8) -> Self {
        Self { sides: sides }
    }

    /// Returns a die with six sides.
    /// If you just need a standard d6, use Die::default().
    ///
    /// # Example
    ///
    /// ```rust
    /// use tiny_die::Die;
    /// let dee_six = Die::default();
    /// ```
    pub fn default() -> Self {
        // There are two ways this can be executed,
        // but I haven't timed how long they take either way.
        // Self { sides: 6 }
        Self::new(6)
    }

    /// Return a number between 1 and number of sides plus 1.
    /// Plus 1 because that's how `gen_range()` works baybee.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tiny_die::Die;
    /// let dee_ten = Die::new(10);
    /// let rolled = dee_ten.roll();
    /// ```
    pub fn roll(&self) -> u8 {
        rand::thread_rng().gen_range(1, self.sides + 1)
    }
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.roll())
    }
}
