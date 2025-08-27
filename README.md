# Eidolon Math Library

A low-level mathematics library implemented in Rust with no external dependencies.

## Structure

### Core Organization
- `src/` - Main source code
  - `bits/` - Bit manipulation system
    - `bit_operations/` - Bit operation categories
      - `bitwise_logic/` - AND, OR, XOR, NOT operations
      - `bitwise_shifting/` - Shift and rotate operations
      - `bitwise_counting/ - Population count, leading/trailing zeros/ones
      - `bitwise_arithmetic/` - Add, subtract, multiply, divide, modulo

### File Organization
Each function group follows this structure:
- `[group_name]/[group_name].rs` - Basic implementations
- `[group_name]_advanced/` - Advanced implementations folder
  - `[group_name]_[function_category].rs` - Specific advanced functions
  - `other_related.rs` - Related functions not based on core implementations

### Advanced Functions
Advanced functions are grouped by similarity:
- `bitwise_counting_leading.rs` - Leading zeros and ones functions
- `bitwise_counting_trailing.rs` - Trailing zeros and ones functions
- `bitwise_counting_population.rs` - Population count functions

## Naming Conventions

### Function Names
Functions use the prefix `E` + `B` + `M`:
- `E` - Eidolon
- `B` - Category (B for Bit operations)
- `M` - Math

Examples:
- `ebm_and` - Bitwise AND operation
- `ebm_left_shift` - Left shift operation
- `ebm_population_count` - Population count operation

### File Names
- Basic files: `[function_group].rs`
- Advanced files: `[function_group]_[category].rs`
- Module files: `mod.rs`

## Implementation Details

### Low-Level Approach
- Uses `std::mem::transmute` for direct memory manipulation
- Byte-by-byte processing for maximum control
- Generic type support for all numeric types (u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize)

### Function Parameters
- Generic type `T` for numeric values
- Generic type `U: Into<u32>` for shift/rotate amounts
- Maximum flexibility for different use cases

### Code Consistency
- Identical structure across all function groups
- Consistent commenting style
- Uniform error handling
- Standardized parameter validation

## Build Configuration

### Cargo.toml
- Edition: 2021
- License: Apache-2.0
- Optimized release profile (opt-level = 3, LTO enabled)
- No external dependencies

### Build Commands
```bash
cargo build          # Debug build
cargo build --release # Optimized release build
cargo test           # Run tests
```

## License

Apache License 2.0 - See LICENSE file for details.

## Development

All code was written by AI carefully directed by human.
