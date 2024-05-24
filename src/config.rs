use uuid::Uuid;

pub enum Outbound {
    Vless(VlessConfig),
    Vmess(VlessConfig),
}

pub struct VlessConfig {
    pub uuid: Uuid,
    pub host: String,
}

pub struct Config {
    pub outbound: Outbound,
}
