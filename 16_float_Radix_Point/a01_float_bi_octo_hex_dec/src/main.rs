fn main() {
    // Invalid: binary float literal is not supported, and '2' and '3' are not binary digits.
    // let bi_float = 0b123.123;

    // 0o123 is 83. Rust doesn't support octal floats.
    let oct_float = 83.0;

    let dec_float = 123.123;

    // 0x123 is 291. Rust doesn't support hex floats.
    let hex_float = 291.0;

    // Example of an octal number with a fractional part (0o12.3) converted to decimal and represented in scientific notation.
    // 0o12.3 = (1 * 8^1) + (2 * 8^0) + (3 * 8^-1) = 8 + 2 + 0.375 = 10.375
    let octal_with_fraction = 1.0375e1;
    let octal_with_fraction_e3 = 1.0375e3;
    let octal_with_fraction_e_3 = 1.0375e-3;

    println!("Hello, world!");
    println!("dec_float: {}", dec_float);
    println!("oct_float: {}", oct_float);
    println!("hex_float: {}", hex_float);
    println!("octal_with_fraction: {}", octal_with_fraction);
    println!("octal_with_fraction: {}", octal_with_fraction_e3);
    println!("octal_with_fraction: {}", octal_with_fraction_e_3);
}
