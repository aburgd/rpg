//! A tiny lib crate for tiny dice.
#![crate_name = "tiny_die"]
#![no_std]

#[cfg(test)]
mod tests {
    use crate::tiny_die::Die;
    #[test]
    fn non_zero_roll() {
        // Following can also be written as
        // let dee_six = Die::default();
        let dee_six = Die::new(6);
        assert_ne!(dee_six.roll(), 0);
    }
    #[test]
    fn less_equal_n_sides_roll() {
        let n_sides: u8 = 20;
        let dee_twenty = Die::new(n_sides);
        assert!(dee_twenty.roll() <= n_sides);
    }
    #[test]
    fn default_dee_six() {
        let dee_six = Die::default();
        assert_eq!(dee_six.sides, 6);
    }
}

/// Contains the `Die` struct and its functions.
pub mod tiny_die {
    use core::fmt;
    use rand::Rng;

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
}
