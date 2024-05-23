use uuid::Uuid;

pub enum Outbound {
    Vless(VlessConfig),
}

pub struct VlessConfig {
    pub uuid: Uuid,
}

pub struct Config {
    pub outbound: Outbound,
}
