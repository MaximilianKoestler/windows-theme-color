# Retrieve Windows Theme Colors in Rust

Simple wrapper around the winrt-API to get access to the active Windows theme colors.

## Usage

```toml
[dependencies]
windows-theme-color = { git = "https://github.com/MaximilianKoestler/windows-theme-color" }
```

## Example

A very basic example that prints the current theme colors using `dbg!(...)`

```rust
use windows_theme_color::{accent_color, background_color, foreground_color};

fn main() {
    dbg!(accent_color());
    dbg!(background_color());
    dbg!(background_color());
}
```

This readme was auto-generated using `cargo readme > README.md`.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
