// Bitwise Logic Operations for Eidolon Math Library

// This module contains ultra-low-level implementations of fundamental bitwise logical operations
// All functions are implemented using Rust's highly optimized built-in operators for maximum performance
// Supporting all numeric types: u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize

// Import necessary standard library components for low-level operations
use std::ops::{BitAnd, BitOr, BitXor, Not};

/// Performs a bitwise AND operation between two values of generic type T
/// 
/// # Arguments
/// * `a` - The first operand for the AND operation
/// * `b` - The second operand for the AND operation
/// 
/// # Returns
/// * `T` - The result of the bitwise AND operation
/// 
/// # Implementation Details
/// This function uses Rust's built-in bitwise AND operator (`&`) which is:
/// 1. Highly optimized by the Rust compiler
/// 2. Compiled to the most efficient CPU instructions
/// 3. Automatically optimized for different architectures
/// 4. Handles all numeric types uniformly and safely
/// 5. Provides consistent performance across platforms
/// 
/// # Performance Characteristics
/// - Zero heap allocations
/// - Minimal stack usage
/// - Optimized for modern CPU architectures
/// - Branchless implementation for consistent performance
/// - Cache-friendly memory access patterns
/// 
/// # Examples
/// ```
/// use eidolon_math::bits::bit_operations::bitwise_logic::bitwise_logic::ebm_and;
/// let result = ebm_and(5u8, 3u8); // 5 & 3 = 1
/// let result = ebm_and(0xFFFFu16, 0x00FFu16); // 0x00FF
/// let result = ebm_and(0xFFFFFFFFu32, 0x0000FFFFu32); // 0x0000FFFF
/// ```
/// 
/// # Function Logic
/// This function performs a bitwise AND operation by comparing each corresponding bit
/// of the two operands. If both bits are 1, the result bit is 1; otherwise, it's 0.
/// This operation is commonly used for masking, clearing specific bits, and checking
/// if certain bits are set in a value.
/// 
/// # Safety Considerations
/// - Uses Rust's safe built-in operators
/// - No undefined behavior possible with valid numeric types
/// - Handles all numeric types uniformly and safely
/// - Compiler ensures type safety at compile time
pub fn ebm_and<T>(a: T, b: T) -> T
where
    T: BitAnd<Output = T> + Copy
{
    // Use Rust's built-in bitwise AND operator for optimal performance
    // This is actually more optimized than manual byte-by-byte manipulation
    // The compiler generates the most efficient CPU instructions for the target architecture
    a & b
}

/// Performs a bitwise OR operation between two values of generic type T
/// 
/// # Arguments
/// * `a` - The first operand for the OR operation
/// * `b` - The second operand for the OR operation
/// 
/// # Returns
/// * `T` - The result of the bitwise OR operation
/// 
/// # Implementation Details
/// This function uses Rust's built-in bitwise OR operator (`|`) which is:
/// 1. Highly optimized by the Rust compiler
/// 2. Compiled to the most efficient CPU instructions
/// 3. Automatically optimized for different architectures
/// 4. Handles all numeric types uniformly and safely
/// 5. Provides consistent performance across platforms
/// 
/// # Performance Characteristics
/// - Zero heap allocations
/// - Minimal stack usage
/// - Optimized for modern CPU architectures
/// - Branchless implementation for consistent performance
/// - Cache-friendly memory access patterns
/// 
/// # Examples
/// ```
/// use eidolon_math::bits::bit_operations::bitwise_logic::bitwise_logic::ebmor;
/// let result = ebmor(5u8, 3u8); // 5 | 3 = 7
/// let result = ebmor(0x00FFu16, 0xFF00u16); // 0xFFFF
/// let result = ebmor(0x0Fu8, 0xF0u8); // 0xFF
/// ```
/// 
/// # Function Logic
/// This function performs a bitwise OR operation by comparing each corresponding bit
/// of the two operands. If either bit is 1, the result bit is 1; only if both bits
/// are 0 does the result bit become 0. This operation is commonly used for setting
/// specific bits, combining bit patterns, and creating masks.
/// 
/// # Safety Considerations
/// - Uses Rust's safe built-in operators
/// - No undefined behavior possible with valid numeric types
/// - Handles all numeric types uniformly and safely
/// - Compiler ensures type safety at compile time
pub fn ebmor<T>(a: T, b: T) -> T
where
    T: BitOr<Output = T> + Copy
{
    // Use Rust's built-in bitwise OR operator for optimal performance
    // This is actually more optimized than manual byte-by-byte manipulation
    // The compiler generates the most efficient CPU instructions for the target architecture
    a | b
}

/// Performs a bitwise XOR (exclusive OR) operation between two values of generic type T
/// 
/// # Arguments
/// * `a` - The first operand for the XOR operation
/// * `b` - The second operand for the XOR operation
/// 
/// # Returns
/// * `T` - The result of the bitwise XOR operation
/// 
/// # Implementation Details
/// This function uses Rust's built-in bitwise XOR operator (`^`) which is:
/// 1. Highly optimized by the Rust compiler
/// 2. Compiled to the most efficient CPU instructions
/// 3. Automatically optimized for different architectures
/// 4. Handles all numeric types uniformly and safely
/// 5. Provides consistent performance across platforms
/// 
/// # Performance Characteristics
/// - Zero heap allocations
/// - Minimal stack usage
/// - Optimized for modern CPU architectures
/// - Branchless implementation for consistent performance
/// - Cache-friendly memory access patterns
/// 
/// # Examples
/// ```
/// use eidolon_math::bits::bit_operations::bitwise_logic::bitwise_logic::ebmxor;
/// let result = ebmxor(5u8, 3u8); // 5 ^ 3 = 6
/// let result = ebmxor(0xFFFFu16, 0x00FFu16); // 0xFF00
/// let result = ebmxor(0xFFu8, 0xFFu8); // 0 (same value XOR = 0)
/// ```
/// 
/// # Function Logic
/// This function performs a bitwise XOR operation by comparing each corresponding bit
/// of the two operands. If the bits are different (one is 1, the other is 0), the
/// result bit is 1; if the bits are the same (both 1 or both 0), the result bit is 0.
/// This operation is commonly used for toggling bits, detecting differences between
/// values, and simple encryption algorithms.
/// 
/// # Safety Considerations
/// - Uses Rust's safe built-in operators
/// - No undefined behavior possible with valid numeric types
/// - Handles all numeric types uniformly and safely
/// - Compiler ensures type safety at compile time
pub fn ebmxor<T>(a: T, b: T) -> T
where
    T: BitXor<Output = T> + Copy
{
    // Use Rust's built-in bitwise XOR operator for optimal performance
    // This is actually more optimized than manual byte-by-byte manipulation
    // The compiler generates the most efficient CPU instructions for the target architecture
    a ^ b
}

/// Performs a bitwise NOT (complement) operation on a value of generic type T
/// 
/// # Arguments
/// * `a` - The operand for the NOT operation
/// 
/// # Returns
/// * `T` - The result of the bitwise NOT operation
/// 
/// # Implementation Details
/// This function uses Rust's built-in bitwise NOT operator (`!`) which is:
/// 1. Highly optimized by the Rust compiler
/// 2. Compiled to the most efficient CPU instructions
/// 3. Automatically optimized for different architectures
/// 4. Handles all numeric types uniformly and safely
/// 5. Provides consistent performance across platforms
/// 
/// # Performance Characteristics
/// - Zero heap allocations
/// - Minimal stack usage
/// - Optimized for modern CPU architectures
/// - Branchless implementation for consistent performance
/// - Cache-friendly memory access patterns
/// 
/// # Examples
/// ```
/// use eidolon_math::bits::bit_operations::bitwise_logic::bitwise_logic::ebmnot;
/// let result = ebmnot(5u8); // ~5 = 250 (for u8)
/// let result = ebmnot(0x00FFu16); // ~0x00FF = 0xFF00
/// let result = ebmnot(0u8); // ~0 = 0xFF
/// ```
/// 
/// # Function Logic
/// This function performs a bitwise NOT operation by flipping each bit of the operand.
/// Every 1 becomes 0, and every 0 becomes 1. This operation is commonly used for
/// inverting all bits in a value, creating complementary masks, and implementing
/// certain mathematical operations like two's complement arithmetic.
/// 
/// # Safety Considerations
/// - Uses Rust's safe built-in operators
/// - No undefined behavior possible with valid numeric types
/// - Handles all numeric types uniformly and safely
/// - Compiler ensures type safety at compile time
pub fn ebmnot<T>(a: T) -> T
where
    T: Not<Output = T> + Copy
{
    // Use Rust's built-in bitwise NOT operator for optimal performance
    // This is actually more optimized than manual byte-by-byte manipulation
    // The compiler generates the most efficient CPU instructions for the target architecture
    !a
}
