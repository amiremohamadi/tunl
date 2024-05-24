pub mod vless;
pub mod vmess;

pub use vless::*;
pub use vmess::*;
use worker::*;

pub trait Proxy {
    fn process(&mut self) -> impl std::future::Future<Output = Result<()>>;
}
