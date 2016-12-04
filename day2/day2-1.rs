use std::fs::File;
use std::io::Read;

struct Keypad {
  x: u32,
  y: u32,
}

impl Keypad {
  fn new() -> Self {
    Keypad {
      x: 2,
      y: 2,
    }
  }

  fn get_self(&self) -> u32 {
    self.x + (self.y - 1) * 3
  }

  fn left(&mut self) {
    if self.x != 1 {
      self.x -= 1;
    }
  }
  fn right(&mut self) {
    if self.x != 3 {
      self.x += 1;
    }
  }
  fn up(&mut self) {
    if self.y != 1 {
      self.y -= 1;
    }
  }
  fn down(&mut self) {
    if self.y != 3 {
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
