use std::fs::File;
use std::io::Read;

const INDEX: [[u8; 5]; 5] =
[
  [   0,    0, b'1',    0,    0],
  [   0, b'2', b'3', b'4',    0],
  [b'5', b'6', b'7', b'8', b'9'],
  [   0, b'A', b'B', b'C',    0],
  [   0,    0, b'D',    0,    0],
];

struct Keypad {
  x: usize,
  y: usize,
}

impl Keypad {
  fn new() -> Self {
    Keypad {
      x: 0,
      y: 2,
    }
  }

  fn get_self(&self) -> char {
    INDEX[self.y][self.x] as char
  }

  fn left(&mut self) {
    if self.x != 0 && INDEX[self.y][self.x - 1] != 0 {
      self.x -= 1;
    }
  }
  fn right(&mut self) {
    if self.x != 4 && INDEX[self.y][self.x + 1] != 0 {
      self.x += 1;
    }
  }
  fn up(&mut self) {
    if self.y != 0 && INDEX[self.y - 1][self.x] != 0 {
      self.y -= 1;
    }
  }
  fn down(&mut self) {
    if self.y != 4 && INDEX[self.y + 1][self.x] != 0 {
      self.y += 1;
    }
  }
}

fn main() {
  let mut inp = Vec::new();
  File::open("day2.txt").unwrap().read_to_end(&mut inp).unwrap();
  let inp = String::from_utf8(inp).unwrap();

  let mut keypad = Keypad::new();

  let mut code = Vec::new();
  for ch in inp.bytes() {
    match ch {
      b'L' => keypad.left(),
      b'R' => keypad.right(),
      b'U' => keypad.up(),
      b'D' => keypad.down(),
      b'\n' => code.push(keypad.get_self()),
      c => panic!("{}", c),
    }
  }

  println!("{:?}", code);
}
