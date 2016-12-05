extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn main() {
  let begin = "uqwqemis";

  let mut password = String::new();
  for i in 0u64.. {
    let current = format!("{}{}", begin, i);
    let mut md5 = Md5::new();
    md5.input_str(&current);
    let md5 = md5.result_str();
    if md5.starts_with("00000") {
      println!("hi");
      password.push_str(&md5[5..6]);
    } else {
      // no match
    }
    if password.len() == 8 {
      break;
    }
  }

  println!("{}", password);
}
