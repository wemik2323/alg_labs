use l3::*;

fn main() {
    let fun = |x: f64| (x - 2.).powi(2);

    let min = iter_fib_min(fun, 1., 4.);
    println!("{min}");
    let min = rec_fib_min(fun, 1., 4.);
    println!("{min}");
}
