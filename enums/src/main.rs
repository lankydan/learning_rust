#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  // can take in &String or string literal ".."
  fn some_method(&self, string: &str) {
    // do something
  }
}

// all possible values of the enum must be included in the match statement
// _ can be used to represent all values not desfined in the match statement
// the contents of the enum must be included in the match options
fn do_something_with_enum(message: Message) {
  match message {
    Message::Quit => println!("gone into quit"),
    Message::Move { x, y } => println!("gone into move"),
    Message::Write(x) => println!("gone into write"),
    Message::ChangeColor(x, y, z) => println!("gone into change colour"),
  }
}

fn do_something_with_enum_with_placeholder(message: Message) {
  match message {
    Message::Quit => println!("gone into quit"),
    _ => println!("I have gone into the placeholder"),
  }
}

fn do_something_with_option(option: Option<String>) {
  match option {
    None => println!("did nothing"),
    Some(x) => println!("did something"),
  }
}

fn main() {
  let q = Message::Quit;
  let m = Message::Move { x: 1, y: 2 };
  let w = Message::Write(String::from("string"));
  let c = Message::ChangeColor(1, 2, 3);
  w.some_method("string");
  println!("{:?}", q);
  println!("{:?}", m);
  println!("{:?}", w);
  println!("{:?}", c);

  let absent_number: Option<i32> = None;

  do_something_with_enum(q);
  do_something_with_enum_with_placeholder(m);
  do_something_with_option(Some(String::from("String")));

  let q2 = Message::Quit;
  if let Message::Quit = q2 {
    println!("q2 input");
  };
}
