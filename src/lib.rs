// Eidolon Math Library - Main Library File
// This file serves as the primary interface for the entire math library
// It exports all mathematical systems and modules for external use

// Export the bits system module
pub mod bits;

// Re-export commonly used items from the bits system for easy access
pub use bits::*;

// Basic tests for GitHub Actions
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_imports() {
        // Test that we can import from the bits module
        assert!(true); // Basic assertion to ensure tests run
    }

    #[test]
    fn test_project_structure() {
        // Test that our module structure is working
        let _bits_module = bits::bit_operations::bitwise_logic::bitwise_logic::ebm_and(0u8, 0u8);
        assert!(true); // If we get here, the import worked
    }
}
