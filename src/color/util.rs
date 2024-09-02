use image::Rgb;

pub(crate) fn map<T: Copy, R>(color: Rgb<T>, mut f: impl FnMut(T) -> R) -> Rgb<R> {
    Rgb([f(red(color)), f(green(color)), f(blue(color))])
}

pub(crate) fn map2<A: Copy, B: Copy, R>(
    a: Rgb<A>,
    b: Rgb<B>,
    mut f: impl FnMut(A, B) -> R,
) -> Rgb<R> {
    Rgb([
        f(red(a), red(b)),
        f(green(a), green(b)),
        f(blue(a), blue(b)),
    ])
}

pub(crate) fn float(color: Rgb<u8>) -> Rgb<f64> {
    map(color, |chanel| chanel as f64 / 255.0)
}

pub(crate) fn red<T: Copy>(color: Rgb<T>) -> T {
    color.0[0]
}

pub(crate) fn green<T: Copy>(color: Rgb<T>) -> T {
    color.0[1]
}

pub(crate) fn blue<T: Copy>(color: Rgb<T>) -> T {
    color.0[2]
}

pub(crate) fn interpolate(from: Rgb<f64>, to: Rgb<f64>, t: f64) -> Rgb<f64> {
    map2(from, to, |from, to| from + t * (to - from))
}

pub(crate) fn square_distance(a: Rgb<f64>, b: Rgb<f64>) -> f64 {
    let square = map2(a, b, |a, b| (a - b) * (a - b));

    red(square) + green(square) + blue(square)
}
