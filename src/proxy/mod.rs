pub mod vless;

pub use vless::*;
use worker::*;

pub trait Proxy {
    fn process(&mut self) -> impl std::future::Future<Output = Result<()>>;
}
