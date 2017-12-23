#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }

  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.width
  }

  fn square(size: u32) -> Rectangle {
    Rectangle {
      height: size,
      width: size,
    }
  }
}

fn main() {
  let rectange = Rectangle {
    width: 30,
    height: 30,
  };
  println!("The area of the rectangle is {}", rectange.area());
  let other_rectangle = Rectangle {
    width: 20,
    height: 100,
  };
  println!("Can hold? {}", rectange.can_hold(&other_rectangle));
  println!("I made a square with area {}", Rectangle::square(40).area());
}
