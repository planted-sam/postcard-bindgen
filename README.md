# Postcard Bindgen

[![Build status](https://github.com/teamplayer3/postcard-bindgen/workflows/Rust/badge.svg)](https://github.com/teamplayer3/postcard-bindgen/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](https://github.com/teamplayer3/postcard-bindgen)
[![Crates.io](https://img.shields.io/crates/v/postcard-bindgen.svg)](https://crates.io/crates/postcard-bindgen)
[![Documentation](https://docs.rs/postcard-bindgen/badge.svg)](https://docs.rs/postcard-bindgen)

The [postcard crate](https://github.com/jamesmunns/postcard) serializes and deserializes rust structs by using the [serde crate](https://github.com/serde-rs/serde) to a byte format. The resulting byte size is minimal. This is very useful if serialization and deserialization is done in rust and share the same structures.

This `crate` can generate bindings from the rust structures for other languages than rust. This allows to use the `postcard crate` from other languages.

> `Crate` is work in progress. By now it can't be used for productions.

## Supported languages

- [x] JavaScript
- [ ] Python

## Usage

The structs for which bindings should be generated must be annotated with the `PostcardBindings` macro. This macro understands `serde` annotation. This means renaming fields and other functionality by `serde` is supported.

## Example

```rust
#[derive(Serialize, PostcardBindings)]
struct Test {
    name: u8,
    other: u16,
}

fn main() {
    export_bindings(
        Path::new("./js_export.js"),
        generate_bindings!(Test), // register container for generating bindings
    )
    .unwrap();
}
```

## JavaScript Type mapping

<table>
<tr><td> Type Name </td> <td> Rust </td> <td> Js </td></tr>
<tr><td>Unit Type</td><td>

```rust
struct UnitStruct;
```
</td><td>

```javascript
{}
```
</td><tr>
<tr><td>New Type</td><td>

```rust
struct NewType(u8);
```
</td><td>

```javascript
[123]
```
</td><tr>
<tr><td>Tuple Struct</td><td>

```rust
struct TupleStruct(u8, u16, u32);
```
</td><td>

```javascript
[123, 1234, 12345]
```
</td><tr>
<tr><td>Struct</td><td>

```rust
struct Struct {
    a: u8,
    b: u16
};
```
</td><td>

```javascript
{
    a: 123,
    b: 1234
}
```
</td><tr>
<tr><td>Enum</td><td>

```rust
enum Enum {
    A,
    B(u8),
    C {
        a: u8
    }
};
```
</td><td>

```javascript
{
    key: "A",
},
{
    key: "B",
    value: 123
},
{
    key: "C",
    value: {
        a: 123
    }
}
```
</td><tr>
<tr><td>Option</td><td>

```rust
struct OptionTuple(Option<u8>);

struct OptionStruct {
    a: Option<u8>
}
```
</td><td>

```javascript
// OptionTuple(Some(123))
[123]
// OptionTuple(None)
[undefined]

// OptionStruct { a: Some(123) }
{
    a: 123
}
// OptionStruct { a: None }
{}
// or
{
    a: undefined
}
```
</td><tr>
<tr><td>Map</td><td>

```rust
let map_string_key = HashMap::<string, u8>::new();

let map_any_key = HashMap::<u16, u8>::new();
```
</td><td>

```javascript
// map_string_key
{
    key: value
}

// map_any_key
new Map()
```
</td><tr>
</table>


