# zalgo-rs
A Zalgoifier for Rust.

## Documentation
<https://nathaniel-daniel.github.io/zalgo-rs/zalgo>

## Example
```rust
fn main() {
    let ret = zalgo::zalgoify("Hello World!");
    println!("{}", ret);
}
```

## Features
| Name        | Description                                                                      | Default feature? |
| ----------- | -------------------------------------------------------------------------------- | ---------------- |
| `nightly`   | Enable optimizations for random number generation. It may do more in the future. | No               |
| `no-unsafe` | Don't use `unsafe` code directly in this library.                                | Yes              |

## License
This crate is dual-licensed under [Apache](./LICENSE-APACHE) and [MIT](LICENSE-MIT).