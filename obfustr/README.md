# obfustr

Compile time obfuscation of string literals.

May be useful to slightly hinder people from reverse engineering your program.

The obfuscated strings are encrypted by XORing with a random bit pattern,
and the random bit pattern is stored next to the encrypted data.
This means you can not easily find the string in the binary by inspecting it with conventional tools.

However, the decryption key is stored directly next to the data,
so this does not effectively protect data that needs to be kept secret.

The library supports obfuscating string literals, byte string literals and C string literals.
All of them are processed using the [`obfuscate!()`] macro.

## Example 1: Obfuscate a string literal
```rust
let message = obfuscate!("Hello world!"); // This gives a `&str`.
```

## Example 2: Obfuscate a byte string literal
```rust
let message = obfuscate!(b"Hello world!"); // This gives a `&[u8]`.
```

## Example 3: Obfuscate a C string literal
```rust
let message = obfuscate!(c"Hello world!"); // This gives a `CStr`.
```

[`obfuscate!()`]: https://docs.rs/obfustr/latest/obfustr/macro.obfuscate.html
