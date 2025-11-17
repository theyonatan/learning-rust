fn main() {
    let x = 255u8;
    let y = 1u8;
    let sum = x.wrapping_add(y);
    assert_eq!(sum, 0);
}
