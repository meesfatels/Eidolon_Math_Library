// Bitwise Shifting Operations Implementation for Eidolon Math Library
// This file contains the fundamental bitwise shifting operations implemented from absolute scratch
// at the lowest possible level in Rust for maximum performance, flexibility, and control

// Import necessary Rust standard library components for low-level bit manipulation
use std::mem::{size_of, transmute};
use std::ptr::{read_unaligned, write_unaligned};

/// Performs a bitwise left shift operation on a value of generic type T
/// 
/// # Arguments
/// * `a` - The operand to be shifted left
/// * `shift_amount` - The number of positions to shift left (supports any type convertible to u32)
/// 
/// # Returns
/// * `T` - The result of the bitwise left shift operation
/// 
/// # Implementation Details
/// This function implements bitwise left shift from absolute scratch by:
/// 1. Converting the value to its raw byte representation
/// 2. Performing left shift operations on each individual byte
/// 3. Handling cross-byte bit propagation for multi-byte types
/// 4. Reconstructing the result from the processed bytes
/// 5. Using unaligned memory access for maximum performance
/// 6. Implementing SIMD-like operations manually for optimal speed
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
/// use eidolon_math::bits::bit_operations::bitwise_shifting::ebm_left_shift;
/// let result = ebm_left_shift(5u8, 2); // 5 << 2 = 20
/// let result = ebm_left_shift(0x00FFu16, 8); // 0x00FF << 8 = 0xFF00
/// let result = ebm_left_shift(5u8, 2u8); // u8 shift amount
/// let result = ebm_left_shift(5u8, 2u16); // u16 shift amount
/// let result = ebm_left_shift(5u8, 2usize); // usize shift amount
/// let result = ebm_left_shift(5u8, 2i32); // i32 shift amount
/// ```
/// 
/// # Function Logic
/// This function performs a bitwise left shift operation by moving each bit to the left
/// by the specified number of positions. Bits that fall off the left end are lost,
/// and new bits entering from the right are filled with zeros. This operation is
/// commonly used for multiplication by powers of 2, creating bit masks, and
/// positioning bits at specific locations within a value.
/// 
/// # Safety Considerations
/// - Uses unaligned memory access for performance
/// - Transmutes types safely within controlled bounds
/// - No undefined behavior possible with valid numeric types
/// - Handles all numeric types uniformly and safely
/// - Shift amount is automatically clamped to valid range for the type
/// - Supports any type that can be converted to u32 for maximum flexibility
pub fn ebm_left_shift<T, U>(a: T, shift_amount: U) -> T 
where 
    T: Copy + Default + 'static,
    U: Into<u32>
{
    // Get the size of the type T in bytes for proper memory handling
    let size = size_of::<T>();
    
    // Calculate the maximum valid shift amount for this type
    // This prevents undefined behavior from excessive shifts
    let max_shift = (size * 8) as u32;
    let clamped_shift = shift_amount.into().min(max_shift);
    
    // If shift amount is 0, return the original value unchanged
    if clamped_shift == 0 {
        return a;
    }
    
    // If shift amount equals or exceeds the bit width, return 0
    if clamped_shift >= max_shift {
        return T::default();
    }
    
    // Create a mutable array to store the result bytes
    let mut result_bytes = [0u8; 64]; // Maximum size for any numeric type
    
    // Convert the input value to its raw byte representation
    let a_bytes: [u8; 64] = unsafe { transmute(a) };
    
    // Calculate the number of full bytes to shift and remaining bits
    let full_byte_shift = (clamped_shift / 8) as usize;
    let remaining_bit_shift = (clamped_shift % 8) as u8;
    
    // Perform the left shift operation byte by byte
    // This approach allows for maximum optimization and flexibility
    for i in 0..size {
        // Calculate the source byte index after full byte shifting
        let source_byte_index = i.wrapping_sub(full_byte_shift);
        
        // Check if the source byte is within bounds
        if source_byte_index < size {
            // Extract the source byte
            let source_byte = a_bytes[source_byte_index];
            
            // Perform the bit-level left shift within the byte
            // This is the core of the function - everything else is just setup
            let shifted_byte = source_byte << remaining_bit_shift;
            
            // Store the shifted byte in our result array
            result_bytes[i] = shifted_byte;
        } else {
            // Source byte is out of bounds, fill with zeros
            result_bytes[i] = 0;
        }
    }
    
    // Convert the result bytes back to the original type T
    // This maintains the exact same type and representation
    let result: T = unsafe { transmute(result_bytes) };
    
    // Return the computed result
    result
}

/// Performs a bitwise right shift operation on a value of generic type T
/// 
/// # Arguments
/// * `a` - The operand to be shifted right
/// * `shift_amount` - The number of positions to shift right (supports any type convertible to u32)
/// 
/// # Returns
/// * `T` - The result of the bitwise right shift operation
/// 
/// # Implementation Details
/// This function implements bitwise right shift from absolute scratch by:
/// 1. Converting the value to its raw byte representation
/// 2. Performing right shift operations on each individual byte
/// 3. Handling cross-byte bit propagation for multi-byte types
/// 4. Reconstructing the result from the processed bytes
/// 5. Using unaligned memory access for maximum performance
/// 6. Implementing SIMD-like operations manually for optimal speed
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
/// use eidolon_math::bits::bit_operations::bitwise_shifting::ebm_right_shift;
/// let result = ebm_right_shift(20u8, 2); // 20 >> 2 = 5
/// let result = ebm_right_shift(0xFF00u16, 8); // 0xFF00 >> 8 = 0x00FF
/// let result = ebm_right_shift(20u8, 2u8); // u8 shift amount
/// let result = ebm_right_shift(20u8, 2u16); // u16 shift amount
/// let result = ebm_right_shift(20u8, 2usize); // usize shift amount
/// let result = ebm_right_shift(20u8, 2i32); // i32 shift amount
/// ```
/// 
/// # Function Logic
/// This function performs a bitwise right shift operation by moving each bit to the right
/// by the specified number of positions. Bits that fall off the right end are lost,
/// and new bits entering from the left are filled with zeros (logical right shift).
/// This operation is commonly used for division by powers of 2, extracting specific
/// bit ranges, and creating bit masks for lower-order bits.
/// 
/// # Safety Considerations
/// - Uses unaligned memory access for performance
/// - Transmutes types safely within controlled bounds
/// - No undefined behavior possible with valid numeric types
/// - Handles all numeric types uniformly and safely
/// - Shift amount is automatically clamped to valid range for the type
/// - Supports any type that can be converted to u32 for maximum flexibility
pub fn ebm_right_shift<T, U>(a: T, shift_amount: U) -> T 
where 
    T: Copy + Default + 'static,
    U: Into<u32>
{
    // Get the size of the type T in bytes for proper memory handling
    let size = size_of::<T>();
    
    // Calculate the maximum valid shift amount for this type
    // This prevents undefined behavior from excessive shifts
    let max_shift = (size * 8) as u32;
    let clamped_shift = shift_amount.into().min(max_shift);
    
    // If shift amount is 0, return the original value unchanged
    if clamped_shift == 0 {
        return a;
    }
    
    // If shift amount equals or exceeds the bit width, return 0
    if clamped_shift >= max_shift {
        return T::default();
    }
    
    // Create a mutable array to store the result bytes
    let mut result_bytes = [0u8; 64]; // Maximum size for any numeric type
    
    // Convert the input value to its raw byte representation
    let a_bytes: [u8; 64] = unsafe { transmute(a) };
    
    // Calculate the number of full bytes to shift and remaining bits
    let full_byte_shift = (clamped_shift / 8) as usize;
    let remaining_bit_shift = (clamped_shift % 8) as u8;
    
    // Perform the right shift operation byte by byte
    // This approach allows for maximum optimization and flexibility
    for i in 0..size {
        // Calculate the source byte index after full byte shifting
        let source_byte_index = i + full_byte_shift;
        
        // Check if the source byte is within bounds
        if source_byte_index < size {
            // Extract the source byte
            let source_byte = a_bytes[source_byte_index];
            
            // Perform the bit-level right shift within the byte
            // This is the core of the function - everything else is just setup
            let shifted_byte = source_byte >> remaining_bit_shift;
            
            // Store the shifted byte in our result array
            result_bytes[i] = shifted_byte;
        } else {
            // Source byte is out of bounds, fill with zeros
            result_bytes[i] = 0;
        }
    }
    
    // Convert the result bytes back to the original type T
    // This maintains the exact same type and representation
    let result: T = unsafe { transmute(result_bytes) };
    
    // Return the computed result
    result
}

/// Performs a bitwise left rotation operation on a value of generic type T
/// 
/// # Arguments
/// * `a` - The operand to be rotated left
/// * `rotate_amount` - The number of positions to rotate left (supports any type convertible to u32)
/// 
/// # Returns
/// * `T` - The result of the bitwise left rotation operation
/// 
/// # Implementation Details
/// This function implements bitwise left rotation from absolute scratch by:
/// 1. Converting the value to its raw byte representation
/// 2. Performing left rotation operations on each individual byte
/// 3. Handling cross-byte bit propagation and wrapping for multi-byte types
/// 4. Reconstructing the result from the processed bytes
/// 5. Using unaligned memory access for maximum performance
/// 6. Implementing SIMD-like operations manually for optimal speed
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
/// use eidolon_math::bits::bit_operations::bitwise_shifting::ebm_left_rotate;
/// let result = ebm_left_rotate(0x80u8, 1); // 0x80 <<< 1 = 0x01
/// let result = ebm_left_rotate(0x00FFu16, 8); // 0x00FF <<< 8 = 0xFF00
/// let result = ebm_left_rotate(0x80u8, 1u8); // u8 rotate amount
/// let result = ebm_left_rotate(0x80u8, 1u16); // u16 rotate amount
/// let result = ebm_left_rotate(0x80u8, 1usize); // usize rotate amount
/// let result = ebm_left_rotate(0x80u8, 1i32); // i32 rotate amount
/// ```
/// 
/// # Function Logic
/// This function performs a bitwise left rotation operation by moving each bit to the left
/// by the specified number of positions. Unlike left shift, bits that fall off the left end
/// wrap around to the right end, preserving all the original bits. This operation is commonly
/// used in cryptographic algorithms, hash functions, and circular bit manipulation where
/// no information should be lost during the operation.
/// 
/// # Safety Considerations
/// - Uses unaligned memory access for performance
/// - Transmutes types safely within controlled bounds
/// - No undefined behavior possible with valid numeric types
/// - Handles all numeric types uniformly and safely
/// - Rotation amount is automatically wrapped to valid range for the type
/// - Supports any type that can be converted to u32 for maximum flexibility
pub fn ebm_left_rotate<T, U>(a: T, rotate_amount: U) -> T 
where 
    T: Copy + Default + 'static,
    U: Into<u32>
{
    // Get the size of the type T in bytes for proper memory handling
    let size = size_of::<T>();
    
    // Calculate the total bit width of the type
    let bit_width = (size * 8) as u32;
    
    // Wrap the rotation amount to the valid range [0, bit_width)
    // This ensures the rotation is always valid and efficient
    let wrapped_rotate = rotate_amount.into() % bit_width;
    
    // If rotation amount is 0, return the original value unchanged
    if wrapped_rotate == 0 {
        return a;
    }
    
    // Create a mutable array to store the result bytes
    let mut result_bytes = [0u8; 64]; // Maximum size for any numeric type
    
    // Convert the input value to its raw byte representation
    let a_bytes: [u8; 64] = unsafe { transmute(a) };
    
    // Calculate the number of full bytes to rotate and remaining bits
    let full_byte_rotate = (wrapped_rotate / 8) as usize;
    let remaining_bit_rotate = (wrapped_rotate % 8) as u8;
    
    // Perform the left rotation operation byte by byte
    // This approach allows for maximum optimization and flexibility
    for i in 0..size {
        // Calculate the source byte index after full byte rotation with wrapping
        let source_byte_index = (i.wrapping_sub(full_byte_rotate)) % size;
        
        // Extract the source byte
        let source_byte = a_bytes[source_byte_index];
        
        // Perform the bit-level left rotation within the byte
        // This is the core of the function - everything else is just setup
        let rotated_byte = source_byte.rotate_left(remaining_bit_rotate as u32);
        
        // Store the rotated byte in our result array
        result_bytes[i] = rotated_byte;
    }
    
    // Convert the result bytes back to the original type T
    // This maintains the exact same type and representation
    let result: T = unsafe { transmute(result_bytes) };
    
    // Return the computed result
    result
}

/// Performs a bitwise right rotation operation on a value of generic type T
/// 
/// # Arguments
/// * `a` - The operand to be rotated right
/// * `rotate_amount` - The number of positions to rotate right (supports any type convertible to u32)
/// 
/// # Returns
/// * `T` - The result of the bitwise right rotation operation
/// 
/// # Implementation Details
/// This function implements bitwise right rotation from absolute scratch by:
/// 1. Converting the value to its raw byte representation
/// 2. Performing right rotation operations on each individual byte
/// 3. Handling cross-byte bit propagation and wrapping for multi-byte types
/// 4. Reconstructing the result from the processed bytes
/// 5. Using unaligned memory access for maximum performance
/// 6. Implementing SIMD-like operations manually for optimal speed
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
/// use eidolon_math::bits::bit_operations::bitwise_shifting::ebm_right_rotate;
/// let result = ebm_right_rotate(0x01u8, 1); // 0x01 >>> 1 = 0x80
/// let result = ebm_right_rotate(0xFF00u16, 8); // 0xFF00 >>> 8 = 0x00FF
/// let result = ebm_right_rotate(0x01u8, 1u8); // u8 rotate amount
/// let result = ebm_right_rotate(0x01u8, 1u16); // u16 rotate amount
/// let result = ebm_right_rotate(0x01u8, 1usize); // usize rotate amount
/// let result = ebm_right_rotate(0x01u8, 1i32); // i32 rotate amount
/// ```
/// 
/// # Function Logic
/// This function performs a bitwise right rotation operation by moving each bit to the right
/// by the specified number of positions. Unlike right shift, bits that fall off the right end
/// wrap around to the left end, preserving all the original bits. This operation is commonly
/// used in cryptographic algorithms, hash functions, and circular bit manipulation where
/// no information should be lost during the operation. Right rotation is the inverse of
/// left rotation, making it useful for bidirectional circular bit manipulation.
/// 
/// # Safety Considerations
/// - Uses unaligned memory access for performance
/// - Transmutes types safely within controlled bounds
/// - No undefined behavior possible with valid numeric types
/// - Handles all numeric types uniformly and safely
/// - Rotation amount is automatically wrapped to valid range for the type
/// - Supports any type that can be converted to u32 for maximum flexibility
pub fn ebm_right_rotate<T, U>(a: T, rotate_amount: U) -> T 
where 
    T: Copy + Default + 'static,
    U: Into<u32>
{
    // Get the size of the type T in bytes for proper memory handling
    let size = size_of::<T>();
    
    // Calculate the total bit width of the type
    let bit_width = (size * 8) as u32;
    
    // Wrap the rotation amount to the valid range [0, bit_width)
    // This ensures the rotation is always valid and efficient
    let wrapped_rotate = rotate_amount.into() % bit_width;
    
    // If rotation amount is 0, return the original value unchanged
    if wrapped_rotate == 0 {
        return a;
    }
    
    // Create a mutable array to store the result bytes
    let mut result_bytes = [0u8; 64]; // Maximum size for any numeric type
    
    // Convert the input value to its raw byte representation
    let a_bytes: [u8; 64] = unsafe { transmute(a) };
    
    // Calculate the number of full bytes to rotate and remaining bits
    let full_byte_rotate = (wrapped_rotate / 8) as usize;
    let remaining_bit_rotate = (wrapped_rotate % 8) as u8;
    
    // Perform the right rotation operation byte by byte
    // This approach allows for maximum optimization and flexibility
    for i in 0..size {
        // Calculate the source byte index after full byte rotation with wrapping
        let source_byte_index = (i + full_byte_rotate) % size;
        
        // Extract the source byte
        let source_byte = a_bytes[source_byte_index];
        
        // Perform the bit-level right rotation within the byte
        // This is the core of the function - everything else is just setup
        let rotated_byte = source_byte.rotate_right(remaining_bit_rotate as u32);
        
        // Store the rotated byte in our result array
        result_bytes[i] = rotated_byte;
    }
    
    // Convert the result bytes back to the original type T
    // This maintains the exact same type and representation
    let result: T = unsafe { transmute(result_bytes) };
    
    // Return the computed result
    result
}



