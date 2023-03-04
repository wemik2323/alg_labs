const PHI: f64 = 1.618033988749894;

#[inline]
pub fn iter_fib_min<F>(func: F, mut from: f64, mut to: f64) -> f64
where
    F: Fn(f64) -> f64,
{
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
pub fn rec_fib_min<F>(func: F, from: f64, to: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    rec_fib_min_impl(
        func,
        from,
        to,
        to - (to - from) / PHI,
        from + (to - from) / PHI,
    )
}

fn rec_fib_min_impl<F>(
    func: F,
    mut from: f64,
    mut to: f64,
    mut near: f64,
    mut far: f64,
) -> f64
where
    F: Fn(f64) -> f64,
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
