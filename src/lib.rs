pub mod crypto;
pub mod wallet;
pub mod didcomm;
pub mod error;

pub use crate::crypto::KeyManager;
pub use crate::wallet::Wallet;
pub use crate::didcomm::Message;
pub use crate::error::Error;

fn main() {
    println!("Hello, world!");
}
