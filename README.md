# intbin : convert int to bin / bin to int

[→ tests/main.rs](tests/main.rs)

```rust
use intbin::{bin_u64, u64_bin};

#[test]
fn main() {
  let bin = u64_bin(987654321);
  dbg!(&bin);
  dbg!(bin_u64(bin));
}
```


run

[→ out.txt](out.txt)

```txt
+ cargo test -- --nocapture
    Finished test [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/intbin-255e90d6507d21ad)
     Running tests/main.rs (target/debug/deps/main-0f27b02ead65ab8e)
[tests/main.rs:6] &bin = [
    177,
    104,
    222,
    58,
]
[tests/main.rs:7] bin_u64(bin) = 987654321
   Doc-tests intbin
```
