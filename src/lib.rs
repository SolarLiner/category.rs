use crate::traits::*;
use alga::general::*;
use num_traits::{One, Zero};
use std::cmp::Ordering;
use num_traits::real::Real;

pub mod traits;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Any(pub bool);

impl Default for Any {
    fn default() -> Self {
        Self(false)
    }
}

impl Semigroup for Any {
    fn op(self, other: Self) -> Self {
        Self(self.0 || other.0)
    }
}
impl DefaultMonoid for Any {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct All(pub bool);

impl Default for All {
    fn default() -> Self {
        Self(true)
    }
}

impl Semigroup for All {
    fn op(self, other: Self) -> Self {
        All(self.0 && other.0)
    }
}
impl DefaultMonoid for All {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Min<T>(pub T);

impl<T: Ord> Semigroup for Min<T> {
    fn op(self, other: Self) -> Self {
        self.min(other)
    }
}

impl<T: Default> Default for Min<T> {
    fn default() -> Self {
        Self(T::default())
    }
}

impl<T: Ord + Default> DefaultMonoid for Min<T> {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Max<T>(pub T);

impl<T: Ord> Semigroup for Max<T> {
    fn op(self, other: Self) -> Self {
        self.max(other)
    }
}

impl<T: Default> Default for Max<T> {
    fn default() -> Self {
        Self(T::default())
    }
}

impl<T: Ord + Default> DefaultMonoid for Max<T> {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Sum<T>(pub T);

impl<T: Zero> Default for Sum<T> {
    fn default() -> Self {
        Sum(T::zero())
    }
}

impl<T: ClosedAdd> Semigroup for Sum<T> {
    fn op(self, other: Self) -> Self {
        Sum(self.0 + other.0)
    }
}
impl<T: ClosedAdd + Zero> DefaultMonoid for Sum<T> {}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Product<T>(pub T);

impl<T: One> Default for Product<T> {
    fn default() -> Self {
        Product(T::one())
    }
}

impl<T: ClosedMul> Semigroup for Product<T> {
    fn op(self, other: Self) -> Self {
        Product(self.0 * other.0)
    }
}

impl<T: ClosedMul + One> DefaultMonoid for Product<T> {}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct First<T>(T);

impl<T> Semigroup for First<T> {
    fn op(self, other: Self) -> Self {
        self
    }
}

impl<T: Monoid> Monoid for First<T> {
    fn empty() -> Self {
        First(T::empty())
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Last<T>(T);

impl<T> Semigroup for Last<T> {
    fn op(self, other: Self) -> Self {
        other
    }
}

impl<T: Monoid> Monoid for Last<T> {
    fn empty() -> Self {
        Last(T::empty())
    }
}

pub struct Predicate<T: ?Sized>(pub Box<dyn FnOnce(&T) -> bool>);

impl<T: 'static + ?Sized> Semigroup for Predicate<T> {
    fn op(self, other: Self) -> Self {
        match (self, other) {
            (Self(p), Self(q)) => Self::new(|a| p(a) && q(a)),
        }
    }
}

impl<T: 'static> Monoid for Predicate<T> {
    fn empty() -> Self {
        Self::new(|_| true)
    }
}

impl<T: ?Sized> Predicate<T> {
    pub fn new<F: 'static + FnOnce(&T) -> bool>(pred: F) -> Self {
        Self(Box::new(pred))
    }

    pub fn call(self, x: &T) -> bool {
        self.0(x)
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::*;
    use crate::*;

    #[test]
    fn any_semigroup() {
        let v = vec![0, 1, 2, 3, 4].into_iter().map(|x| x > 2).map(Any);
        let r = Semigroup::concat(v);
        assert_eq!(Some(Any(true)), r);
    }

    #[test]
    fn all_semigroup() {
        let v = vec![0, 1, 2, 3, 4].into_iter().map(|x| x > 2).map(All);
        let r = Semigroup::concat(v);
        assert_eq!(Some(All(false)), r);
    }

    #[test]
    fn predicate_semigroup() {
        let v = vec![
            Predicate::new(|x: &str| x.starts_with("a")),
            Predicate::new(|x: &str| x.ends_with("z")),
        ];
        let r: Option<Predicate<_>> = Semigroup::concat(v);
        assert!(r.is_some());
        let r = r.unwrap();
        assert!(r.call("a to z"));
    }

    #[test]
    fn sum_semigroup() {
        let s = Sum(0.0).op(Sum(0.1)).op(Sum(1.0)).0;
        assert_eq!(1.1, s);
    }

    #[test]
    fn product_semigroup() {
        let v = vec![1u8, 2, 3, 4, 0, 5].into_iter().map(Product);
        let v = Semigroup::concat(v);
        assert!(v.is_some());
        assert_eq!(v.unwrap(), Product(0));
    }

    #[test]
    fn tuple_semigroup() {
        let a = (Sum(1.0), Product(10));
        let b = a.op((Sum(1.0), Product(2)));
        assert_eq!(b, (Sum(2.0), Product(20)));
    }

    #[test]
    fn float_monoid() {
        let v = (0u8..10).map(|x| Sum(x as f32 / 10.0));
        let Sum(r) = Monoid::concat(v);
        assert_eq!(r, 4.5);
    }

    #[test]
    fn int_min_monoid() {
        let v = vec![1i32,-1,15,-42,74,42].into_iter().map(Min);
        let Min(r) = Monoid::concat(v);

        assert_eq!(r, -42);
    }
}
