pub fn bin_u64(bin: impl AsRef<[u8]>) -> u64 {
  let bin = bin.as_ref();
  let mut b = [0u8; 8];
  b[..bin.len()].copy_from_slice(bin);
  u64::from_le_bytes(b)
}

pub fn u64_bin(n: u64) -> Box<[u8]> {
  let n = n.to_le_bytes();
  let mut i = 8;
  while i > 0 {
    let p = i - 1;
    if n[p] != 0 {
      break;
    }
    i = p;
  }
  Box::from(&n[..i])
}

pub fn u8_bin(n: u8) -> Box<[u8]> {
  if n == 0 {
    return [].into();
  }
  [n].into()
}

pub fn bin_u8(bin: impl AsRef<[u8]>) -> u8 {
  let bin = bin.as_ref();
  if bin.is_empty() {
    return 0;
  };
  bin[1]
}
