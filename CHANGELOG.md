Change Log
==========

## Version 0.3.0

### Features
* support u128 and i128 (where they can safely convert to f64)
* 32-bit support

## Version 0.2.0

### Features

* Update to neon 0.3.1
* Support for nodeJS 12.*

### BREAKING

* Removed support for nodeJS 6.* (end of life)

## Version 0.1.1

### Features

* Document how to use `serde_bytes::ByteBuf` with node `Buffer`s
* Allow docs and other attributes on functions in export macro
* add `#[allow(snake_case)]` added by default in export macro
* Better behaviour for `Option<T>` in export macro

```rust,no-run
...
export! {
    /// Adding docs and attributes here now works
    fn get_length(name: Option<String>) -> Option<u8> {
        name.map(|n| n.len())
    }

    /// Makes a `Buffer` node side
    fn get_big_data() -> serde_bytes::ByteBuf {
        let data: Vec<u8> = ...;
        serde_bytes::ByteBuf::from(data)
    }
}
```
```javascript
nativeModule.get_length("1")       // Always worked
nativeModule.get_length(null)      // Always worked
nativeModule.get_length(undefined) // Always worked
nativeModule.get_length()          // Works as of v0.1.1

nativeModule.get_big_data()        // returns Buffer
```


## Version 0.1.0

### Breaking

* Requires neon 0.2

### Other Changes

* Add Testing on node 10
* Add License file MIT
