# Changelog

All changes to this project will be noted in this file

## Unreleased

### Additions

- Added the `#[const_gtor]` and `#[const_ctor]` attributes for providing compile-time ctors and getters
- Added the `#[gtor_copy]` attribute for compound `Copy` types (UDFs)

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
