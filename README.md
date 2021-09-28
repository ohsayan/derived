# `derived`: Macros for the boring stuff

[![Crates.io](https://img.shields.io/crates/v/derived?style=flat-square)](https://crates.io/crates/derived) [![docs.rs](https://img.shields.io/docsrs/derived?style=flat-square)](https://docs.rs/derived) [![GitHub](https://img.shields.io/github/license/ohsayan/derived?style=flat-square)](./LICENSE)

The `derived` crate provides macros that can simplify all the boring stuff, like writing constructors for example.

## Features

- `Ctor`: To generate constructors
- `Gtor`: To generate getters
- `Stor`: To generate setters
- Full lifetimes, generics and `where` clause support
- Skip generation of setters or getters with the `#[stor_skip]` or `#[gtor_skip]` attributes for
  specific fields
- Make ctors and gtors `const` with the `#[ctor_const]` and `#[gtor_const]` attributes

## Example: Generating constructors

```rust
use derived::Ctor;

#[derive(Ctor)]
pub struct MyStruct {
    a: u8,
    b: i8,
}

let mystruct = MyStruct::new(1, -1);
assert_eq!(mystruct.a, 1);
assert_eq!(mystruct,b, -1);
```

## Example: Generating getters

```rust
use derived::{Ctor, Gtor};

// we'll derive `Ctor` to avoid having to write ctors
#[derive(Ctor, Gtor)]
pub struct MyStruct {
    name: String,
    userid: u64,
}

let ms = MyStruct::new("Sayan".to_owned(), 1);
assert_eq!(ms.get_name(), "sayan");
// we don't need to deref because u64 is a copy type
assert_eq!(ms.get_userid(), 1);
```

## Example: Generating setters

```rust
use derived::{Ctor, Stor};

// we'll derive `Ctor` to avoid having to write ctors
#[derive(Ctor, Stor)]
pub struct MyStruct {
    name: String,
    userid: u64,
}

let mut ms = MyStruct::new("Sayan".to_owned(), 1);
assert_eq!(ms.get_name(), "sayan");
// we don't need to deref because u64 is a copy type
assert_eq!(ms.get_userid(), 1);
ms.set_userid(0);
assert_eq!(ms.get_userid(), 0);
```

## Example: Adanced generics and lifetimes in structs

```rust
use derived::{Ctor, Gtor};

#[derive(Ctor, Gtor)]
struct MyTag<'a, T: ToString> {
    val: &'a T,
    tag: u8,
}

let mut x = MyTag::new(&10i32, 20); // this will have type MyTag<i32>
// you can now use getters and setters as you please!
assert_eq!(x.get_val().to_string(), "10");
x.set_val(11);
assert_eq!(x.get_val().to_string(), "11");
```

## License

This crate is distributed under the [Apache-2.0 License](./LICENSE).
