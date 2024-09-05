use image::{GenericImageView, Pixel, Rgb, Rgba};
use num_rational::Ratio;
use num_traits::ToPrimitive;
use rounded_div::RoundedDiv;

#[inline]
pub(super) fn map2<A: Copy, B: Copy, R>(a: Rgb<A>, b: Rgb<B>, mut f: impl FnMut(A, B) -> R) -> Rgb<R> {
    Rgb([f(a.0[0], b.0[0]), f(a.0[1], b.0[1]), f(a.0[2], b.0[2])])
}

#[inline]
pub(super) fn interpolate(from: Rgb<u8>, to: Rgb<u8>, t: Ratio<u32>) -> Rgb<u8> {
    map2(from, to, |from, to| {
        let (numerator, denominator) = t.into_raw();

        let from = from as u32;
        let to = to as u32;

        ((denominator - numerator) * from + numerator * to)
            .rounded_div(denominator)
            .try_into()
            .unwrap_or(u8::MAX)
    })
}

#[inline]
pub(super) fn square_distance(a: Rgb<u8>, b: Rgb<u8>) -> u32 {
    let r = u8::abs_diff(a.0[0], b.0[0]) as u32;
    let g = u8::abs_diff(a.0[1], b.0[1]) as u32;
    let b = u8::abs_diff(a.0[2], b.0[2]) as u32;

    r * r + g * g + b * b
}

pub(super) fn average_color(image: impl GenericImageView) -> Rgba<u8> {
    let (width, height) = image.dimensions();
    let area = (width * height) as u64;

    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let mut a = 0;

    for x in 0..width {
        for y in 0..height {
            let color = image.get_pixel(x, y).to_rgba();
            r += color.0[0].to_u64().unwrap_or_default();
            g += color.0[1].to_u64().unwrap_or_default();
            b += color.0[2].to_u64().unwrap_or_default();
            a += color.0[3].to_u64().unwrap_or_default();
        }
    }

    r /= area;
    g /= area;
    b /= area;
    a /= area;

    Rgba([r as u8, g as u8, b as u8, a as u8])
}