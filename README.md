# Is my type an `Option`?

```rust
use is_option::is_option;

assert!(is_option!(Option<bool>));
assert!(is_option!(Option<String>));
assert!(is_option!(Option<&str>));

assert!(!is_option!(bool));
assert!(!is_option!(&str));
assert!(!is_option!(Vec<usize>));
```