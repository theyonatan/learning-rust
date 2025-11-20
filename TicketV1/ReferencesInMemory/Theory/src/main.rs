fn main() {
    assert_eq!(std::mem::size_of::<&String>(), 8);
    assert_eq!(std::mem::size_of::<&mut String>(), 8);
}
