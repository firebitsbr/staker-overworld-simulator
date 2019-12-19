#![deny(missing_docs)]

//! Library for simulation of the Zone

/// Module for simulation functions of zone NPCs
pub mod entity;

/// Sample functions
pub mod sample {
    /// Test function for understand module import in rust
    pub fn test_func() {
        println!("Test function prints");
    }
}

#[cfg(test)]
mod tests {
    //! Unit testing module
    /// sample unit test
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
