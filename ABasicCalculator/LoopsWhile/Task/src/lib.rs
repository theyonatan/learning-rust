// Rewrite the factorial function using a `while` loop.
pub fn factorial(mut n: u32) -> u32 {
    let mut result: u32 = n;

    if n == 0 {
        return 1;
    }

    while n > 1 {
        result *= n - 1;
        n -= 1;
    }

    result
}
