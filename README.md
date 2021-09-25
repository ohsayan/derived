# `derived`: Macros for the boring stuff

> **Note: This is currently a WIP**

The `derived` crate provides macros that can simplify all the boring stuff, like writing constructors for example.

## Example: Generating constructors

```rust
use derived::Ctor;

#[derive(Ctor)]
pub struct MyStruct {
    a: u8,
    b: i8,
}

let mystruct = Ctor::new(1, -1);
assert_eq!(mystruct.a, 1);
assert_eq!(mystruct,b, -1);
```

## Example: Generating getters

```rust
use derived::{Ctor, Gtor};

// we'll derive `Ctor` to avoid having to rewrite ctors
#[derive(Ctor, Gtor)]
pub struct MyStruct {
    name: String,
    userid: u64,
}

let ms = MyStruct::new("Sayan".to_owned(), 1);
assert_eq!(ms.get_name(), "sayan");
// we need to deref because the generated getters always return references
assert_eq!(*ms.get_userid(), 1);
```

## License

This crate is distributed under the [Apache-2.0 License](./LICENSE).
