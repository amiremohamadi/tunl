mod config;
mod proxy;

use crate::config::{Config, Outbound, VlessConfig};
use crate::proxy::*;

use uuid::Uuid;
use worker::*;

#[event(fetch)]
async fn main(_: Request, env: Env, _: Context) -> Result<Response> {
    let uuid = env
        .var("UUID")
        .map(|x| Uuid::parse_str(&x.to_string()).unwrap_or_default())?;

    let config = Config {
        outbound: Outbound::Vless(VlessConfig { uuid }),
    };

    let WebSocketPair { server, client } = WebSocketPair::new()?;

    server.accept()?;
    wasm_bindgen_futures::spawn_local(async move {
        let events = server.events().unwrap();
        let mut stream = match config.outbound {
            Outbound::Vless(c) => VlessStream::new(c, &server, events),
        };

        if let Err(e) = stream.process().await {
            console_debug!("error occured during tunneling stream: {}", e);
        }
    });

    Response::from_websocket(client)
}
