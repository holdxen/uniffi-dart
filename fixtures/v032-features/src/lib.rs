//! Regression fixture for the breaking changes introduced by UniFFI 0.32.
//!
//! Covers the bindgen-relevant changes that required adaptations in uniffi-dart:
//!   * `Type::Set` — proc-macro `HashSet<T>` support (new wire type, Dart `Set`).
//!   * `Type::Box` — `Box<T>` now implements the FFI traits and is used directly
//!     in enum variants / function parameters (bindings treat it as its inner type).
//!   * `ObjectImpl::Trait(TraitKind)` — the new `#[uniffi::export(foreign)]` and
//!     `#[uniffi::export(rust, foreign)]` trait kinds replace `with_foreign`.
//!   * `Type::Custom { builtin }` / recursive support remain exercised by other fixtures.

use std::collections::HashSet;
use std::sync::Arc;

/// A set crossing the FFI: exercises `Type::Set` and the Dart `Set` converter.
#[uniffi::export]
pub fn sum_set(values: HashSet<u32>) -> u64 {
    values.into_iter().map(u64::from).sum()
}

/// Round-trips a set through a record field.
#[derive(uniffi::Record)]
pub struct SetBag {
    pub values: HashSet<String>,
}

#[uniffi::export]
pub fn echo_set_bag(bag: SetBag) -> SetBag {
    bag
}

/// A set nested inside an optional / sequence exercises recursive converter rendering.
#[uniffi::export]
pub fn sets_len(bags: Vec<SetBag>) -> u32 {
    bags.iter().map(|bag| bag.values.len() as u32).sum()
}

/// `Box<T>` in an enum variant: bindings render the inner type, but the metadata
/// now carries `Type::Box`.
#[derive(uniffi::Enum)]
pub enum MaybeBoxed {
    Empty,
    Value(Box<u32>),
}

#[uniffi::export]
pub fn unwrap_boxed(value: MaybeBoxed) -> Option<u32> {
    match value {
        MaybeBoxed::Empty => None,
        MaybeBoxed::Value(v) => Some(*v),
    }
}

/// `Box<T>` as a function parameter (0.32 auto-implements the FFI traits).
#[uniffi::export]
pub fn double_boxed(value: Box<i64>) -> i64 {
    *value * 2
}

/// Foreign-only trait (`TraitKind::ForeignOnly`) — the 0.32 replacement for the
/// legacy `#[uniffi::export(callback_interface)]`.
#[uniffi::export(foreign)]
pub trait Greeter: Send + Sync {
    fn greet(&self, name: String) -> String;
}

#[uniffi::export]
pub fn greet_with(greeter: Arc<dyn Greeter>, name: String) -> String {
    greeter.greet(name)
}

/// Rust + foreign trait (`TraitKind::Both`) — the new spelling of `with_foreign`.
#[uniffi::export(rust, foreign)]
pub trait Named: Send + Sync {
    fn name(&self) -> String;
}

struct RustNamed;

impl Named for RustNamed {
    fn name(&self) -> String {
        "RustNamed".to_string()
    }
}

#[uniffi::export]
pub fn rust_named() -> Arc<dyn Named> {
    Arc::new(RustNamed)
}

#[uniffi::export]
pub fn read_name(named: Arc<dyn Named>) -> String {
    named.name()
}

uniffi::include_scaffolding!("api");
