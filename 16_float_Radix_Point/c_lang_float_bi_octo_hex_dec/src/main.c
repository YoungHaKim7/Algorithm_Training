#include <stdio.h>

int main(void) {
    // C does not support binary (0b) or octal (0o) floating-point literals.
    // The original code used these non-standard formats.
    // For demonstration, we are using decimal equivalents or valid C syntax.
    // The literal '0b123.123' is invalid as '2' and '3' are not binary digits.
    // We will use a decimal value instead.
    float bi_float = 123.123;

    // For the octal value, we convert the integer part. 0o123 = 83.
    float oc_float = 83.123;

    // For the hexadecimal value, we convert the integer part. 0x123 = 291.
    // The original 0x123.123 is not a valid C hex float literal (it needs a 'p' exponent).
    float hex_float = 291.123;

    // The printf format specifier for floats must be %f, not %d.
    printf("binary_float = %f\n", bi_float);
    printf("octo_float = %f\n", oc_float);
    printf("hex_float = %f\n", hex_float);
    return 0;
}
