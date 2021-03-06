#[macro_use]
extern crate approx;

use autodiff::F;
use hrbf::kernel::*;

const TEST_RADIUS: f64 = 2.0;

fn test_kernel<C, K: Kernel<F>>(ker: K, x0: f64, compare: C)
where
    C: Fn(f64, f64),
{
    let x = F::var(x0);

    let f = ker.f(x);
    let df = ker.df(x);
    let ddf = ker.ddf(x);
    let dddf = ker.dddf(x);
    let ddddf = ker.ddddf(x);

    compare(f.dx, df.x);
    compare(df.dx, ddf.x);
    compare(ddf.dx, dddf.x);
    compare(dddf.dx, ddddf.x);

    if x0 != 0.0 {
        let df_l = ker.df_l(x);
        let g = ker.g(x);
        let g_l = ker.g_l(x);
        let h3 = ker.h(x, F::cst(3.0));
        let h52 = ker.h(x, F::cst(5.0 / 2.0));
        compare(x0 * df_l.x, df.x);
        compare(x0 * x0 * g.x, ddf.x * x0 - df.x);
        compare(x0 * g_l.x, g.x);
        compare(
            x0 * x0 * x0 * h3.x,
            x0 * x0 * dddf.x - 3.0 * (x0 * ddf.x - df.x),
        );
        compare(
            x0 * x0 * x0 * h52.x,
            x0 * x0 * dddf.x - 0.5 * 5.0 * (x0 * ddf.x - df.x),
        );
    }
}

fn test_kernel_simple<K: Kernel<F> + Copy>(kern: K) {
    for &x in [0.0, 1.0, 0.5, ::std::f64::consts::PI].iter() {
        test_kernel(kern, x, ulp_compare);
    }
}

fn test_kernel_random<K: Kernel<F> + Copy>(kern: K) {
    use rand::distributions::Uniform;
    use rand::prelude::*;

    let seed = [3; 32];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let range = Uniform::new(-1.0, 1.0);
    for _ in 0..999 {
        let x = rng.sample(range);
        test_kernel(kern, x, rel_compare);
    }
}

fn ulp_compare(a: f64, b: f64) {
    assert_ulps_eq!(a, b, max_ulps = 6);
}

fn rel_compare(a: f64, b: f64) {
    assert_relative_eq!(a, b, max_relative = 1e-12, epsilon = 1e-14);
}

#[test]
fn pow2_test() {
    let kern = Pow2::<F>::default();
    test_kernel_simple(kern);
    test_kernel_random(kern);
}

#[test]
fn pow3_test() {
    let kern = Pow3::<F>::default();
    test_kernel_simple(kern);
    test_kernel_random(kern);
}

#[test]
fn pow4_test() {
    let kern = Pow4::<F>::default();
    test_kernel_simple(kern);
    test_kernel_random(kern);
}

#[test]
fn pow5_test() {
    let kern = Pow5::<F>::default();
    test_kernel_simple(kern);
    test_kernel_random(kern);
}

#[test]
fn gauss_test() {
    let kern = Gauss::<F>::new(F::cst(TEST_RADIUS));
    test_kernel_simple(kern);
    test_kernel_random(kern);
}

#[test]
fn csrbf31_test() {
    let kern = Csrbf31::<F>::new(F::cst(TEST_RADIUS));
    test_kernel_simple(kern);
    test_kernel_random(kern);
}

#[test]
fn csrbf42_test() {
    let kern = Csrbf42::<F>::new(F::cst(TEST_RADIUS));
    test_kernel_simple(kern);
    test_kernel_random(kern);
}
