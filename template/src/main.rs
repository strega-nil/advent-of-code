use std::fs::File;
use std::io::Read;

fn isalpha(c: u8) -> bool {
  c >= b'a' && c <= b'z' || c >= b'A' && c <= b'Z'
}

fn isdigit(c: u8) -> bool {
  c >= b'0' && c <= b'9'
}

fn isspace(c: u8) -> bool {
  c == b' ' || c == b'\n' || c == b'\t'
}

fn main() {
  let mut inp = Vec::new();
  File::open("day3.txt").unwrap().read_to_end(&mut inp).unwrap();
  let inp = String::from_utf8(inp).unwrap();
}
