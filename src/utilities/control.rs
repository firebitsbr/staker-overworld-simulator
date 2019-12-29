/// Used for numeric types that can be clamped to a minimum
/// and maximum range.
pub trait Clampable {
    /// Returns the min_bound if self is less than min_bound,
    /// returns the max_bound if self is greater than max_bound,
    /// otherwise, returns self.
    fn clamp_range(self, min_bound: Self, max_bound:Self)-> Self;
}


impl Clampable for f32 {
    fn clamp_range(self, min_bound: Self, max_bound:Self)-> Self {
        self.max(min_bound).min(max_bound)
    }
}