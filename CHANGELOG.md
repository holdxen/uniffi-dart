# Changelog

All notable changes to uniffi-dart will be documented in this file.

## v0.3.0+v0.32.0

### Breaking changes

- **BREAKING**: Upgrade uniffi-rs from v0.31.2 to v0.32.0. Downstream crates
  targeting 0.31.x must move to 0.32.x as well.

### Changes

- Generate Dart `Set<T>` bindings for the new `HashSet` / `Type::Set` metadata,
  using the same i32-length-prefixed wire format as sequences.
- Treat `Box<T>` as its inner type in generated bindings, matching uniffi-rs
  0.32 where `Box<T>` implements the FFI traits directly.
- Support the new trait export kinds: `ObjectImpl::Trait` now carries a
  `TraitKind` (`RustOnly`, `Both`, `ForeignOnly`), replacing the old
  `Trait` / `CallbackTrait` variants. `#[uniffi::export(foreign)]` and
  `#[uniffi::export(rust, foreign)]` generate the appropriate callback
  interfaces, and the legacy `with_foreign` spelling keeps working.
- Handle the new `EmptySet` default literal, rendering a typed const empty set.
- Adapt to the pipeline `nodes` types no longer being publicly re-exported.

### Fixes

- Render `Set`, `Box`, and their nested forms consistently across all type,
  FFI, and default-value code paths.

### Testing

- Add the `v032-features` fixture covering HashSet round-trips (including in
  records and sequences), `Box<T>` in enum variants and parameters, and the
  new foreign-only / rust+foreign trait kinds.

## v0.2.1+v0.31.2

### Changes

- Upgrade uniffi-rs from v0.31.1 to v0.31.2 (#146)

### Fixes

- Fix name mangling for all-caps acronym identifiers (#145)
- Register recursive type helpers so recursive types generate correctly (#138)
- Generate Rust-backed proxies for callback trait interfaces and handle
  tagged callback interface pointers (#135)
- Make the ordering of generated Dart type helpers deterministic (#134)

## v0.2.0+v0.31.1

### Breaking changes

- **BREAKING**: Upgrade uniffi-rs from v0.30.0 to v0.31.1 (#125)

### Fixes

- Generate FfiConverter for Sequence/Optional of records in callback interfaces (#117)
- Fix Dart error object renaming (#120)
- Use Optional FfiConverter's `lower()` for inner types to avoid double-wrapping nullable records in callback return values (#121)
- Make nullable record fields optional in Dart constructors (#122)

## v0.1.0+v0.30.0

Initial release of uniffi-dart targeting uniffi-rs v0.30.0.

### Dart binding generation

- All primitive types with bounds checking
- Strings, bytes, optionals, sequences, and maps
- Records with default values and named constructors
- Enums (flat and complex) with variant support
- Objects with constructors, methods, and disposable pattern
- Error types and exception handling
- Custom types
- Durations and timestamps

### Async support

- Async/Future support for functions, methods, and constructors
- Callback interfaces (UDL and proc-macro)
- Trait interfaces
- Stream support via extension macros

### Code generation

- Named parameters for generated functions and objects
- Multiple namespace support
- Dart Native Assets with `@Native` annotations
- Configurable library loading strategy
- Formatted generated Dart code

### Testing

- Comprehensive test suite
- CI with downstream testing (rust-payjoin and bdk-dart)
