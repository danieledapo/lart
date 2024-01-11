/// Dead simple wrapper over a f64 that can be used as the key to the various
/// sort_by_key functions.
///
/// Note that it behaves slightly differently than f64 with regard to equality
/// checking namely that `-0.0_f64 == 0.0_f64` but `F64Key(-0.0) != F64Key(0.0)`.
#[derive(Debug, Clone, Copy)]
pub struct F64Key(pub f64);

impl Eq for F64Key {}
impl PartialEq for F64Key {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Ord for F64Key {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}
impl PartialOrd for F64Key {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
