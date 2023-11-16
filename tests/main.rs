use intbin::{bin_u64, u64_bin};

#[test]
fn main() {
  let bin = u64_bin(987654321);
  dbg!(&bin);
  dbg!(bin_u64(bin));
}
