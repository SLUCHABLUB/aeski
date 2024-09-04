use image::Rgb;
use num_rational::Ratio;

#[inline]
pub fn map2<A: Copy, B: Copy, R>(a: Rgb<A>, b: Rgb<B>, mut f: impl FnMut(A, B) -> R) -> Rgb<R> {
    Rgb([f(a.0[0], b.0[0]), f(a.0[1], b.0[1]), f(a.0[2], b.0[2])])
}

#[inline]
pub(crate) fn interpolate(from: Rgb<u8>, to: Rgb<u8>, t: Ratio<usize>) -> Rgb<u8> {
    map2(from, to, |from, to| {
        let from = Ratio::from(from as usize);
        let to = Ratio::from(to as usize);
        (from + t * to - t * from)
            .round()
            .to_integer()
            .try_into()
            .unwrap_or(u8::MAX)
    })
}

#[inline]
pub(crate) fn square_distance(a: Rgb<u8>, b: Rgb<u8>) -> u32 {
    let r = u8::abs_diff(a.0[0], b.0[0]) as u32;
    let g = u8::abs_diff(a.0[1], b.0[1]) as u32;
    let b = u8::abs_diff(a.0[2], b.0[2]) as u32;

    r * r + g * g + b * b
}
