# tiny_die

this tiny crate provides the means of incorporating die rolls into your own project.
uses `no_std`.

## example

```rust
use tiny_die::Die;

fn main() {
    let dee_six: Die = Die::new(6);
    // or you can use Die::default(), you'll get the same d6
    println!("rolled a {}", dee_six.roll());
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.