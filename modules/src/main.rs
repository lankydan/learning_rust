extern crate modules;

use modules::client;
use modules::network;
use modules::network::server;

// the below can be used like in java to import multiple modules like you can with packages
// use modules::* 

// you cannot define different modules with the same name in the file. Like you cant statically import two classes of the same name in java
// use modules::network::client;

enum Colour {
  Red,
  Yellow,
  Blue
}
// using a enum, the below line allows Red or Yellow to be used rather than Colour::Red
use Colour::{ Red, Yellow };

/*
The mod declaration should not be included like a class name in java. This is taken from the file name.
For inner modules you can use mod to defien a new one, but if you extra this into another file, remember that
you create a new folder for the parent module, create a mod.rs file for the parent module within the folder and 
new files for each inner module. These modules do not need a mod declaration and will only contain their methods within 
their file.

For top level modules the mod declaration can be used, e.g. if defined in the lib.rs or main.rs file.
*/

fn main() {
  modules::client::connect();
  modules::network::connect();
  modules::network::client::connect();
  modules::network::server::connect();

  client::connect();
  network::connect();
  network::client::connect();
  server::connect();

  let red = Red;
  let yellow = Yellow;
  let blue = Colour::Blue;
}
