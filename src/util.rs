
pub fn parse_byte(val: &str) -> Option<u8> {
    val.parse().ok()
}

/// Returns true if two enums have the same variant.
pub fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
