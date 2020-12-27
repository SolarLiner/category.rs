use std::cmp::Ordering;

pub trait Semigroup: Sized {
    fn op(self, other: Self) -> Self;
    fn concat<I: Iterator<Item = Self>>(
        this: impl IntoIterator<Item = Self, IntoIter = I>,
    ) -> Option<Self> {
        this.into_iter().fold(None, |acc, x| match acc {
            None => Some(x),
            Some(y) => Some(y.op(x)),
        })
    }

    fn repeat(self, n: usize) -> Self
    where
        Self: Copy,
    {
        let mut res = self;
        for _ in 0..n {
            res = res.op(res);
        }
        return res;
    }
}

impl Semigroup for Ordering {
    fn op(self, other: Self) -> Self {
        match (self, other) {
            (Self::Less, _) => Self::Less,
            (Self::Equal, y) => y,
            (Self::Greater, _) => Self::Greater,
        }
    }
}

impl Semigroup for () {
    fn op(self, _other: Self) -> Self {
        ()
    }

    fn concat<I: Iterator<Item = Self>>(
        _: impl IntoIterator<Item = Self, IntoIter = I>,
    ) -> Option<Self> {
        Some(())
    }

    fn repeat(self, _: usize) -> Self
    where
        Self: Copy,
    {
        ()
    }
}

impl<T> Semigroup for Vec<T> {
    fn op(self, other: Self) -> Self {
        self.into_iter().chain(other.into_iter()).collect()
    }
}

impl Semigroup for String {
    fn op(mut self, other: Self) -> Self {
        self.push_str(&other);
        self
    }
}

impl<T: Semigroup> Semigroup for Option<T> {
    fn op(self, other: Self) -> Self {
        match (self, other) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(a.op(b)),
        }
    }
}

impl<T: Semigroup, U: Semigroup> Semigroup for (T, U) {
    fn op(self, other: Self) -> Self {
        (self.0.op(other.0), self.1.op(other.1))
    }
}

pub trait Monoid: Semigroup {
    fn empty() -> Self;
    fn concat(this: impl IntoIterator<Item = Self>) -> Self {
        this.into_iter().fold(Self::empty(), Semigroup::op)
    }
}

pub trait DefaultMonoid: Default + Semigroup {}

impl<T: DefaultMonoid> Monoid for T {
    fn empty() -> Self {
        T::default()
    }
}

impl Monoid for Ordering {
    fn empty() -> Self {
        Self::Equal
    }
}

impl<T> Monoid for Vec<T> {
    fn empty() -> Self {
        Vec::with_capacity(0) // Preventing allocation
    }
}

impl Monoid for String {
    fn empty() -> Self {
        String::new()
    }
}

impl<T: Monoid, U: Monoid> Monoid for (T, U) {
    fn empty() -> Self {
        (T::empty(), U::empty())
    }
}
