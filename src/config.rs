use crate::ssh::Host;

pub fn default_host() -> Host {
    Host {
        user: "my_name".into(),
        host: "my_server_ip".into(),
    }
}
