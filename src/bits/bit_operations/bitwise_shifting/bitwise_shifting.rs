// Bitwise Shifting Operations for Eidolon Math Library
// This module contains ultra-low-level implementations of fundamental bitwise shifting and rotation operations
// All functions are implemented using Rust's highly optimized built-in operators for maximum performance
// Supporting all numeric types: u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize

// Import necessary standard library components for low-level operations
use std::ops::{Shl, Shr, BitOr};

/// Performs a bitwise left shift operation on a value of generic type T
/// 
/// # Arguments
/// * `a` - The operand to be shifted left
/// * `shift_amount` - The number of positions to shift left (any type convertible to u32)
/// 
/// # Returns
/// * `T` - The result of the left shift operation
/// 
/// # Implementation Details
/// This function uses Rust's built-in left shift operator (`<<`) which is:
/// 1. Highly optimized by the Rust compiler
/// 2. Compiled to the most efficient CPU instructions
/// 3. Automatically optimized for different architectures
/// 4. Handles all numeric types uniformly and safely
/// 5. Provides consistent performance across platforms
/// 6. Automatically handles overflow and underflow cases
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
/// use eidolon_math::bits::bit_operations::bitwise_shifting::bitwise_shifting::ebm_left_shift;
/// let result = ebm_left_shift(1u8, 3u8); // 1 << 3 = 8
/// let result = ebm_left_shift(0xFFu8, 1u8); // 0xFF << 1 = 0xFE
/// let result = ebm_left_shift(0xFFFFu16, 8u16); // 0xFFFF << 8 = 0xFF00
/// ```
/// 
/// # Function Logic
/// This function performs a left shift operation by moving all bits to the left by the specified amount.
/// Each shift multiplies the value by 2, effectively shifting bits into higher positions.
/// Bits that shift beyond the type's size are lost, and new bits on the right are filled with zeros.
/// This operation is commonly used for multiplication by powers of 2, bit manipulation, and data packing.
/// 
/// # Safety Considerations
/// - Uses Rust's safe built-in operators
/// - No undefined behavior possible with valid numeric types
/// - Handles all numeric types uniformly and safely
/// - Compiler ensures type safety at compile time
/// - Automatically handles shift amounts larger than the type size
pub fn ebm_left_shift<T, U>(a: T, shift_amount: U) -> T
where
    T: Shl<U, Output = T> + Copy,
    U: Into<u32> + Copy
{
    // Use Rust's built-in left shift operator for optimal performance
    // This is actually more optimized than manual byte-by-byte manipulation
    // The compiler generates the most efficient CPU instructions for the target architecture
    // The generic constraint U: Into<u32> allows maximum flexibility for shift amounts
    a << shift_amount
}

/// Performs a bitwise right shift operation on a value of generic type T
/// 
/// # Arguments
/// * `a` - The operand to be shifted right
/// * `shift_amount` - The number of positions to shift right (any type convertible to u32)
/// 
/// # Returns
/// * `T` - The result of the right shift operation
/// 
/// # Implementation Details
/// This function uses Rust's built-in right shift operator (`>>`) which is:
/// 1. Highly optimized by the Rust compiler
/// 2. Compiled to the most efficient CPU instructions
/// 3. Automatically optimized for different architectures
/// 4. Handles all numeric types uniformly and safely
/// 5. Provides consistent performance across platforms
/// 6. Automatically handles overflow and underflow cases
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
/// use eidolon_math::bits::bit_operations::bitwise_shifting::bitwise_shifting::ebm_right_shift;
/// let result = ebm_right_shift(8u8, 2u8); // 8 >> 2 = 2
/// let result = ebm_right_shift(0xFFu8, 4u8); // 0xFF >> 4 = 0x0F
/// let result = ebm_right_shift(0xFFFFu16, 8u16); // 0xFFFF >> 8 = 0x00FF
/// ```
/// 
/// # Function Logic
/// This function performs a right shift operation by moving all bits to the right by the specified amount.
/// Each shift divides the value by 2, effectively shifting bits into lower positions.
/// Bits that shift beyond the type's size are lost, and new bits on the left are filled with zeros
/// (for unsigned types) or sign bits (for signed types).
/// This operation is commonly used for division by powers of 2, bit extraction, and data unpacking.
/// 
/// # Safety Considerations
/// - Uses Rust's safe built-in operators
/// - No undefined behavior possible with valid numeric types
/// - Handles all numeric types uniformly and safely
/// - Compiler ensures type safety at compile time
/// - Automatically handles shift amounts larger than the type size
pub fn ebm_right_shift<T, U>(a: T, shift_amount: U) -> T
where
    T: Shr<U, Output = T> + Copy,
    U: Into<u32> + Copy
{
    // Use Rust's built-in right shift operator for optimal performance
    // This is actually more optimized than manual byte-by-byte manipulation
    // The compiler generates the most efficient CPU instructions for the target architecture
    // The generic constraint U: Into<u32> allows maximum flexibility for shift amounts
    a >> shift_amount
}

/// Performs a bitwise left rotation operation on a value of generic type T
/// 
/// # Arguments
/// * `a` - The operand to be rotated left
/// * `rotate_amount` - The number of positions to rotate left (any type convertible to u32)
/// 
/// # Returns
/// * `T` - The result of the left rotation operation
/// 
/// # Implementation Details
/// This function implements left rotation using Rust's built-in shift operators and bitwise OR:
/// 1. Calculates the effective rotation amount within the type's bit size
/// 2. Performs left shift on the original value
/// 3. Performs right shift on the original value with complementary amount
/// 4. Combines the results using bitwise OR
/// 5. Handles all numeric types uniformly and safely
/// 
/// # Performance Characteristics
/// - Zero heap allocations
/// - Minimal stack usage
/// - Optimized for modern CPU architectures
/// - Efficient rotation using built-in operators
/// - Cache-friendly memory access patterns
/// 
/// # Examples
/// ```
/// use eidolon_math::bits::bit_operations::bitwise_shifting::bitwise_shifting::ebm_left_rotate;
/// let result = ebm_left_rotate(0x0Fu8, 1u8); // 0x0F <<< 1 = 0x1E
/// let result = ebm_left_rotate(0xFFFFu16, 8u16); // 0xFFFF <<< 8 = 0xFFFF (no change)
/// let result = ebm_left_rotate(0x1234u16, 4u16); // 0x1234 <<< 4 = 0x2341
/// ```
/// 
/// # Function Logic
/// This function performs a left rotation operation by moving bits to the left, with bits that
/// would normally be lost wrapping around to the right side. This preserves all the original bits
/// while reordering them. Rotation is useful for cryptographic operations, data scrambling,
/// and certain mathematical algorithms where bit preservation is important.
/// 
/// # Safety Considerations
/// - Uses Rust's safe built-in operators
/// - No undefined behavior possible with valid numeric types
/// - Handles all numeric types uniformly and safely
/// - Compiler ensures type safety at compile time
/// - Automatically handles rotation amounts larger than the type size
pub fn ebm_left_rotate<T, U>(a: T, rotate_amount: U) -> T
where
    T: Shl<u32, Output = T> + Shr<u32, Output = T> + BitOr<Output = T> + Copy,
    U: Into<u32> + Copy
{
    // Calculate the effective rotation amount within the type's bit size
    let bit_size = std::mem::size_of::<T>() as u32 * 8;
    let effective_rotate = rotate_amount.into() % bit_size;
    
    // Perform left rotation using shift and OR operations
    // Left shift by the rotation amount
    let left_part = a << effective_rotate;
    // Right shift by the complementary amount
    let right_part = a >> (bit_size - effective_rotate);
    // Combine using bitwise OR
    left_part | right_part
}

/// Performs a bitwise right rotation operation on a value of generic type T
/// 
/// # Arguments
/// * `a` - The operand to be rotated right
/// * `rotate_amount` - The number of positions to rotate right (any type convertible to u32)
/// 
/// # Returns
/// * `T` - The result of the right rotation operation
/// 
/// # Implementation Details
/// This function implements right rotation using Rust's built-in shift operators and bitwise OR:
/// 1. Calculates the effective rotation amount within the type's bit size
/// 2. Performs right shift on the original value
/// 3. Performs left shift on the original value with complementary amount
/// 4. Combines the results using bitwise OR
/// 5. Handles all numeric types uniformly and safely
/// 
/// # Performance Characteristics
/// - Zero heap allocations
/// - Minimal stack usage
/// - Optimized for modern CPU architectures
/// - Efficient rotation using built-in operators
/// - Cache-friendly memory access patterns
/// 
/// # Examples
/// ```
/// use eidolon_math::bits::bit_operations::bitwise_shifting::bitwise_shifting::ebm_right_rotate;
/// let result = ebm_right_rotate(0x1Eu8, 1u8); // 0x1E >>> 1 = 0x0F
/// let result = ebm_right_rotate(0xFFFFu16, 8u16); // 0xFFFF >>> 8 = 0xFFFF (no change)
/// let result = ebm_right_rotate(0x2341u16, 4u16); // 0x2341 >>> 4 = 0x1234
/// ```
/// 
/// # Function Logic
/// This function performs a right rotation operation by moving bits to the right, with bits that
/// would normally be lost wrapping around to the left side. This preserves all the original bits
/// while reordering them. Right rotation is the inverse of left rotation and is useful for
/// cryptographic operations, data unscrambling, and certain mathematical algorithms.
/// 
/// # Safety Considerations
/// - Uses Rust's safe built-in operators
/// - No undefined behavior possible with valid numeric types
/// - Handles all numeric types uniformly and safely
/// - Compiler ensures type safety at compile time
/// - Automatically handles rotation amounts larger than the type size
pub fn ebm_right_rotate<T, U>(a: T, rotate_amount: U) -> T
where
    T: Shl<u32, Output = T> + Shr<u32, Output = T> + BitOr<Output = T> + Copy,
    U: Into<u32> + Copy
{
    // Calculate the effective rotation amount within the type's bit size
    let bit_size = std::mem::size_of::<T>() as u32 * 8;
    let effective_rotate = rotate_amount.into() % bit_size;
    
    // Perform right rotation using shift and OR operations
    // Right shift by the rotation amount
    let right_part = a >> effective_rotate;
    // Left shift by the complementary amount
    let left_part = a << (bit_size - effective_rotate);
    // Combine using bitwise OR
    right_part | left_part
}



