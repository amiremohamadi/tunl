mod common;
mod config;
mod proxy;

use crate::config::{Config, Outbound, VlessConfig};
use crate::proxy::*;

use serde::Serialize;
use uuid::Uuid;
use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _: Context) -> Result<Response> {
    let uuid = env
        .var("UUID")
        .map(|x| Uuid::parse_str(&x.to_string()).unwrap_or_default())?;

    let outbound = env
        .var("OUTBOUND")
        .map(|x| x.to_string())
        .unwrap_or_default();

    let host = req.url()?.host().map(|x| x.to_string()).unwrap_or_default();

    let config = Config {
        outbound: match outbound.as_str() {
            "vmess" => Outbound::Vmess(VlessConfig { uuid, host }),
            "vless" => Outbound::Vless(VlessConfig { uuid, host }),
            _ => return Err(Error::RustError("invalid outbound".to_string())),
        },
    };

    Router::with_data(config)
        .on_async("/", tunnel)
        .on("/link", link)
        .run(req, env)
        .await
}

async fn tunnel(_: Request, cx: RouteContext<Config>) -> Result<Response> {
    let WebSocketPair { server, client } = WebSocketPair::new()?;

    server.accept()?;
    wasm_bindgen_futures::spawn_local(async move {
        let events = server.events().unwrap();
        if let Err(e) = match cx.data.outbound {
            Outbound::Vless(c) => VlessStream::new(c, &server, events).process().await,
            Outbound::Vmess(c) => VmessStream::new(c, &server, events).process().await,
        } {
            console_debug!("error occured during tunneling stream: {}", e);
        };
    });

    Response::from_websocket(client)
}

fn link(_: Request, cx: RouteContext<Config>) -> Result<Response> {
    #[derive(Serialize)]
    struct Link {
        description: String,
        link: String,
    }

    let link = match cx.data.outbound {
        Outbound::Vmess(c) => {
            format!(
                "vmess://{}@104.24.30.167:80?encryption=zero&type=ws&security=none&host={}#tunl",
                c.uuid, c.host
            )
        }
        Outbound::Vless(c) => {
            format!(
                "vless://{}@104.24.30.167:80?type=ws&security=none&host={}#tunl",
                c.uuid, c.host
            )
        }
    };

    Response::from_json(&Link {
        link,
        description: "visit https://scanner.github1.cloud/ and replace the IP address in the configuration with a clean one".to_string()
    })
}
