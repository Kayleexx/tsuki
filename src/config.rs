use crate::ssh::Host;

pub fn default_host() -> Host {
    Host {
        user: "kaylee".into(),
        host: "192.168.0.47".into(),
    }
}