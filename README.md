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
   Compiling intbin v0.1.1 (/Users/z/art/intbin)
    Finished test [unoptimized + debuginfo] target(s) in 0.12s
     Running unittests src/lib.rs (target/debug/deps/intbin-ffe375fcab040bbd)
     Running tests/main.rs (target/debug/deps/main-5e93df032d50e825)
[tests/main.rs:6] &bin = [
    177,
    104,
    222,
    58,
]
[tests/main.rs:7] bin_u64(bin) = 987654321
   Doc-tests intbin
```

