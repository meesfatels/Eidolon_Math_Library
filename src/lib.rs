// Eidolon Math Library - Main Library File
// This file serves as the primary interface for the entire math library
// It exports all mathematical systems and modules for external use

// Export the bits system module
pub mod bits;

// Basic test to ensure compilation works
#[cfg(test)]
mod tests {
    #[test]
    fn test_basic_compilation() {
        assert!(true);
    }
}
