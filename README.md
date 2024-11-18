# Shared

A few utility types similar to `std::borrow::Cow`, but more niche.

Currently includes the following:
```rust
pub enum Shared<T: ?Sized> {
  Static(&'static T),
  Shared(std::sync::Arc<T>),
}

pub enum StaticOrBoxed<T: ?Sized> {
  Static(&'static T),
  Boxed(Box<T>),
}
 
pub enum ArcOrRef<'a, T: ?Sized> {
  Ref(&'a std::sync::Arc<T>),
  Arc(std::sync::Arc<T>),
}
```
