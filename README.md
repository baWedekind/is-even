# is-even
Returns true if the given number is even.

## Install
Specify the dependencty in Cargo.toml:

```yaml
[dependencies]
is-even = "~1.0.0"
```

Fetch it with cargo:
```bash
$ cargo build
```

## Usage

```rust
extern crate is_even;
use is_even::IsEven;

let _i : i32 = 1;
println!("{}", _i.is_even()); // prints false
```

## About
### License
Copyright © 2019, [baWedekind](https://github.com/baWedekind).
Released under the [MIT License](LICENSE).
