# Shared

A few utility types similar to `std::borrow::Cow`, but more niche.

Currently includes the following:
### `Shared<T>`
```rust
/// For potentially static, but otherwise immutable values
/// (the motivating purpose was config values that can be either
/// staticlly specified, or lazily loaded at runtime)
pub enum Shared<T: ?Sized> {
  Static(&'static T),
  Shared(std::sync::Arc<T>),
}
```
### `StaticOrBoxed<T>`
```rust
/// Similar to `Shared<T>`, but for values that don't need to be cloned or shared. 
pub enum StaticOrBoxed<T: ?Sized> {
  Static(&'static T),
  Boxed(Box<T>),
}
```
### `ArcOrRef<'_, T>`
```rust
/// Allows for lazy cloning of `Arc<T>`'s, useful for
/// chaining method that would otherwise have to clone every
/// method call.
pub enum ArcOrRef<'a, T: ?Sized> {
  Ref(&'a std::sync::Arc<T>),
  Arc(std::sync::Arc<T>),
}
```
