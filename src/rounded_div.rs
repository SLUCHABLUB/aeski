#[inline]
pub(crate) fn rounded_div(numerator: u32, denominator: u32) -> u32 {
    (numerator + (denominator >> 1)) / denominator
}
