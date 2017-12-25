fn main() {
  let v_1 = vec![1, 2, 3];
  // need to make the vector mutable to add and remove elements
  let mut v_2 = Vec::new();
  v_2.push(1);
  v_2.push(2);
  v_2.push(3);
  // pop removes last
  v_2.pop();
  // remove works like it does in java
  v_2.remove(0);
  // the below line does not run, new() cannot have arguments
  // let v_3 = Vec::new(1, 2, 3);

  println!("v_1: {:?}", v_1);
  println!("v_2: {:?}", v_2);

  let v_3 = vec![1, 2, 3, 4, 5];
  for i in v_3 {
    println!("loop: {}", i);
  }

  // you can store different types in the vector by wrapping the type in an enum
  // the vector is then only storing a single type which is the enum which wraps the inner type
  let v_4 = vec![
    StoreMyEnums::Int(1),
    StoreMyEnums::Float(2.0),
    StoreMyEnums::Text(String::from("text")),
  ];
  println!("v_4: {:?}", v_4);

  let v_5 = vec![1, 2, 3, 4];
  let first = &v_5[0];
  println!("first: {}", first);
  let last: Option<&i32> = v_5.get(3);
  println!("last: {:?}", last);
  let last_2 = v_5.get(100);
  println!("out of bounds: {:?}", last_2);

  match last {
    Some(x) => println!("inside match: {}", x),
    None => println!("inside match not found"),
  };
}

#[derive(Debug)]
enum StoreMyEnums {
  Int(i32),
  Float(f64),
  Text(String),
}
