extern crate modules;

/*
The mod declaration should not be included like a class name in java. This is taken from the file name.
For inner modules you can use mod to defien a new one, but if you extra this into another file, remember that
you create a new folder for the parent module, create a mod.rs file for the parent module within the folder and 
new files for each inner module. These modules do not need a mod declaration and will only contain their methods within 
their file.
*/

fn main() {
  modules::client::connect();
  modules::network::connect();
  modules::network::client::connect();
  modules::network::server::connect();
}
