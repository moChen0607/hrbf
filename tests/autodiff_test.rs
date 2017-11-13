extern crate num_traits;
extern crate rand;

mod autodiff;

use num_traits::Float;
use autodiff::{cst, diff, grad};

// NOTE: we don't need approximate equality here because we compare with the exact derivative
// expression, which means that the derivative and expected values should be identically computed.
#[test]
fn simple_test() {
    assert_eq!(diff(|_| cst(1.0), 0.0), 0.0);
    assert_eq!(diff(|x| x, 0.0), 1.0);
    assert_eq!(diff(|x| x*x, 0.0), 0.0);
    assert_eq!(diff(|x| x*x, 1.0), 2.0);
    assert_eq!(diff(|x| Float::exp(-x*x/cst(2.0)), 0.0), 0.0);
}

#[test]
fn random_test() {
    use self::rand::{SeedableRng, StdRng};
    use self::rand::distributions::{IndependentSample, Range};

    let seed: &[_] = &[1,2,3,4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let range = Range::new(-1.0, 1.0);
    for _ in 0..99 {
        let t = range.ind_sample(&mut rng);
        assert_eq!(diff(|x| Float::exp(-x*x/cst(2.0)), t), -t*Float::exp(-t*t/2.0));
    }
}

#[test]
fn grad_test() {
    use self::rand::{SeedableRng, StdRng};
    use self::rand::distributions::{IndependentSample, Range};

    let seed: &[_] = &[1,2,3,4];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let range = Range::new(-1.0, 1.0);
    for _ in 0..99 {
        let t = vec![range.ind_sample(&mut rng), range.ind_sample(&mut rng)];
        let expected = vec![-0.5*t[1]*Float::exp(-t[0]*t[1]/2.0), -0.5*t[0]*Float::exp(-t[0]*t[1]/2.0)];
        assert_eq!(grad(|x| Float::exp(-x[0]*x[1]/cst(2.0)), t), expected);
    }
}