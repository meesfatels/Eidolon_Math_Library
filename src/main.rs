// Main entry point for the Eidolon Math Library
// This file serves as the primary interface for the entire math library
// It will import and re-export all the mathematical systems and modules

// Import the bits system module
pub mod bits;

// Re-export commonly used items from the bits system for easy access
pub use bits::*;

// Main function - this will be used when building as a binary
// For library usage, this won't be called
#[cfg(test)]
fn main() {
    println!("Eidolon Math Library - Running in test mode");
}

// When building as a library, this function won't be included
#[cfg(not(test))]
fn main() {
    println!("Eidolon Math Library - Running as binary");
}
