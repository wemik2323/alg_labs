const PHI: f64 = 1.618033988749894;

#[inline]
pub fn iter_gldn_ratio_min(
    func: fn(f64) -> f64,
    mut from: f64,
    mut to: f64,
) -> f64 {
    let mut near = to - (to - from) / PHI;
    let mut far = from + (to - from) / PHI;

    while (to - from).abs() > 1e-5 {
        if func(near) < func(far) {
            to = far;
        } else {
            from = near;
        }

        near = to - (to - from) / PHI;
        far = from + (to - from) / PHI;
    }

    (to + from) / 2.
}

#[inline]
pub fn rec_gldn_ratio_min(func: fn(f64) -> f64, from: f64, to: f64) -> f64 {
    rec_fib_min_impl(
        func,
        from,
        to,
        to - (to - from) / PHI,
        from + (to - from) / PHI,
    )
}

#[inline]
fn rec_fib_min_impl(
    func: fn(f64) -> f64,
    mut from: f64,
    mut to: f64,
    mut near: f64,
    mut far: f64,
) -> f64
{
    if (to - from).abs() <= 1e-5 {
        return (to + from) / 2.;
    }

    if func(near) < func(far) {
        to = far;
    } else {
        from = near;
    }

    near = to - (to - from) / PHI;
    far = from + (to - from) / PHI;

    rec_fib_min_impl(func, from, to, near, far)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_fib_min_works() {
        let linear = |x: f64| -0.5 * x - 3.;
        assert!((iter_gldn_ratio_min(linear, -2., 2.) - 2.).abs() <= 1e-5);

        let quadratic = |x: f64| (2. - x).powi(2);
        assert!((iter_gldn_ratio_min(quadratic, 1., 4.) - 2.).abs() <= 1e-5);

        let cubic = |x: f64| (x + 2.).powi(3) + 1.;
        assert!((iter_gldn_ratio_min(cubic, -3., -1.) + 3.).abs() <= 1e-5);
    }

    #[test]
    fn rec_fib_min_works() {
        let linear = |x: f64| -0.5 * x - 3.;
        assert!((rec_gldn_ratio_min(linear, -2., 2.) - 2.).abs() <= 1e-5);

        let quadratic = |x: f64| (2. - x).powi(2);
        assert!((rec_gldn_ratio_min(quadratic, 1., 4.) - 2.).abs() <= 1e-5);

        let cubic = |x: f64| (x + 2.).powi(3) + 1.;
        assert!((rec_gldn_ratio_min(cubic, -3., -1.) + 3.).abs() <= 1e-5);
    }
}
