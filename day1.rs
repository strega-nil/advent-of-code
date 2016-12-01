static STR: &'static str = "R4, R4, L1, R3, L5, R2, R5, R1, L4, R3, L5, R2, L3, L4, L3, R1, R5, R1, L3, L1, R3, L1, R2, R2, L2, R5, L3, L4, R4, R4, R2, L4, L1, R5, L1, L4, R4, L1, R1, L2, R5, L2, L3, R2, R1, L194, R2, L4, R49, R1, R3, L5, L4, L1, R4, R2, R1, L5, R3, L5, L4, R4, R4, L2, L3, R78, L5, R4, R191, R4, R3, R1, L2, R1, R3, L1, R3, R4, R2, L2, R1, R4, L5, R2, L2, L4, L2, R1, R2, L3, R5, R2, L3, L3, R3, L1, L1, R5, L4, L4, L2, R5, R1, R4, L3, L5, L4, R5, L4, R5, R4, L3, L2, L5, R4, R3, L3, R1, L5, R5, R1, L3, R2, L5, R5, L3, R1, R4, L5, R4, R2, R3, L4, L5, R3, R4, L5, L5, R4, L4, L4, R1, R5, R3, L1, L4, L3, L4, R1, L5, L1, R2, R2, R4, R4, L5, R4, R1, L1, L1, L3, L5, L2, R4, L3, L5, L4, L1, R3, ";

#[derive(Copy, Clone, Debug)]
enum Direction {
  North,
  South,
  East,
  West,
}
use self::Direction::*;

impl Direction {
  fn left(&self) -> Self {
    match *self {
      North => West,
      South => East,
      East => North,
      West => South,
    }
  }
  fn right(&self) -> Self {
    match *self {
      North => East,
      South => West,
      East => South,
      West => North,
    }
  }

  fn x(&self) -> i32 {
    match *self {
      North => 0,
      South => 0,
      East => 1,
      West => -1,
    }
  }
  fn y(&self) -> i32 {
    match *self {
      North => 1,
      South => -1,
      East => 0,
      West => 0,
    }
  }
}

fn isdigit(c: u8) -> bool {
  c >= b'0' && c <= b'9'
}

fn main() {
  let s = STR.as_bytes();
  let mut i = 0;

  let mut locs = Vec::new();
  let mut x = 0;
  let mut y = 0;

  let mut current_direction = North;
  while i < s.len() {
    current_direction = match s[i] {
      b'L' => current_direction.left(),
      b'R' => current_direction.right(),
      c => panic!("fuck: {}", c as char),
    };
    i += 1;

    let mut parse = String::new();
    while isdigit(s[i]) {
      parse.push(s[i] as char);
      i += 1;
    }
    let mut num = parse.parse::<i32>().unwrap();

    if current_direction.x() != 0 {
      while num != 0 {
        x += current_direction.x();
        num -= 1;
        for &(x_, y_) in &locs {
          if (x, y) == (x_, y_) {
            println!("duplicate destination: {}, {}", x, y);
            std::process::exit(0);
          }
        }
        locs.push((x, y));
      }
    } else {
      while num != 0 {
        y += current_direction.y();
        num -= 1;
        for &(x_, y_) in &locs {
          if (x, y) == (x_, y_) {
            println!("duplicate destination: {}, {}", x, y);
            std::process::exit(0);
          }
        }
        locs.push((x, y));
      }
    }

    i += 2;

  }

  println!("{}, {}", x, y);
}
