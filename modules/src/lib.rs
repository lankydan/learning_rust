pub mod network;
pub mod client;

mod tests {
  // super goes up one module level from where you are
  use super::client;
  #[test]
  fn it_works() {
    client::connect();
  }
}
