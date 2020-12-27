# category.rs

Category theory traits for Rust, and helper structs for operations on them.

```rust
fn min<T>(values: Vec<T>) -> T {
    let Min(res) = Monoid::concat(values.into_iter().map(Min));
    res
}
```

## Roadmap

- [x] Semigroup
- [x] Monoid
- [ ] Functor (this and following need GATs)
- [ ] Applicative
- [ ] Alternative
- [ ] Monad
- [ ] Traversable