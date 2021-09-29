# Changelog

All changes to this project will be noted in this file

## Unreleased

### Additions

- Added support for full paths to primitives (`std::primitives::u8` or `core::primitives::u8`,
  for example)

### 0.4.1

### Additions

- Added the `Constdef` (`#[derive(Constdef)]`) derive macro for generating compile-time, constant
  default implementations
- Added full support for nesting:
  - Nested arrays
  - Nested tuples
  - Nesting arrays in tuples
  - Nesting tuples in arrays

## 0.4.0

### Additions

- Added the `#[gtor_const]` and `#[ctor_const]` attributes for providing compile-time ctors and getters
- Added the `#[gtor_copy]` attribute for compound `Copy` types (UDFs)
- Added the `#[gtor_skip]` attribute for skipping gtors for specific fields
- Added the `#[stor_skip]` attribute for skipping stors for specific fields
- Added the `#[phantom]` attribute for skipping ctors, gtors and stors for `PhantomData` fields
- Added the `gtor` attribute for passing `#[gtor(get, get_mut)]` to make only mutable getters, only
  immutable getters, or both

## 0.3.1

- Fixed incorrect compiler error messages and inconsistencies in the documentation

## 0.3.0

### Additions

- Added the `Stor` macro for generating setters
- Added support for generic paramters (and lifetimes) in all the macros

## 0.2.0

Added the `Gtor` macro for generating getters. `Gtor` will automatically generate getters, automatically
copy some primitive types and add documentation comments.

## 0.1.1

Fixed field re-ordering in the `Ctor` macro

## 0.1.0

Added the `Ctor` macro
