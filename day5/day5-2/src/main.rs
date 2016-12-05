extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn isdigit(c: u8) -> bool {
  c >= b'0' && c <= b'9'
}

fn main() {
  let begin = "uqwqemis";

  let mut password = vec![0; 8];
  let mut found = 0;
  for i in 0u64.. {
    let current = format!("{}{}", begin, i);
    let mut md5 = Md5::new();
    md5.input_str(&current);
    let md5 = md5.result_str();
    if md5.starts_with("00000") {
      println!("hi");
      let md5 = md5.into_bytes();
      let n = md5[5];
      if isdigit(n) && n < b'8' && password[(n - b'0') as usize] == 0 {
        password[(n - b'0') as usize] = md5[6];
        found += 1;
        if found == 8 {
          break;
        }
      }
    } else {
      // no match
    }
  }

  println!("{}", String::from_utf8(password).unwrap());
}
