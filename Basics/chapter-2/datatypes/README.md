# `DataTypes`

* The names of Rust’s numeric types follow a regular pattern, spelling out their width in bits, and the representation they use

| Size (bits)   | Unsigned Integer | Signed Integer | Floating-point |
|---------------|------------------|----------------|----------------|
| 8             | u8               | i8             |                |
| 16            | u16              | i16            |                |
| 32            | u32              | i32            | f32            |
| 64            | u64              | i64            | f64            |
| 128           | u128             | i128           |                |
| Machine word  | usize            | isize          |                |

* Imp
```bash
Rust uses the u8 type for byte values
```

* Operations

| Operation           | Name Suffix | Example                                        |
|---------------------|-------------|------------------------------------------------|
| Addition            | add         | 100_i8.checked_add(27) == Some(127)            |
| Subtraction         | sub         | 10_u8.checked_sub(11) == None                  |
| Multiplication      | mul         | 128_u8.saturating_mul(3) == 255                |
| Division            | div         | 64_u16.wrapping_div(8) == 8                    |
| Remainder           | rem         | (-32768_i16).wrapping_rem(-1) == 0             |
| Negation            | neg         | (-128_i8).checked_neg() == None                |
| Absolute value      | abs         | (-32768_i16).wrapping_abs() == -32768          |
| Exponentiation      | pow         | 3_u8.checked_pow(4) == Some(81)                |
| Bitwise left shift  | shl         | 10_u32.wrapping_shl(34) == 40                  |
| Bitwise right shift | shr         | 40_u64.wrapping_shr(66) == 10                  |

* `Float point types`

| Type | Precision                                          | Range                                                      |
|------|----------------------------------------------------|------------------------------------------------------------|
| f32  | IEEE single precision (at least 6 decimal digits)  | Roughly –3.4 × 10<sup>38</sup> to +3.4 × 10<sup>38</sup>   |
| f64  | IEEE double precision (at least 15 decimal digits) | Roughly –1.8 × 10<sup>308</sup> to +1.8 × 10<sup>308</sup> |
