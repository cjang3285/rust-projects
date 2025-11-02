use sysinfo::Networks;

pub struct NetworkInfo {
    pub interface: String,
    pub received: u64,
    pub transmitted: u64,
}

pub fn get_network_info() -> Vec<NetworkInfo> {
    let networks = Networks::new_with_refreshed_list();
    networks
        .iter()
        .map(|(name, data)| NetworkInfo {
            interface: name.clone(),
            received: data.received(),
            transmitted: data.transmitted(),
        })
        .collect()
}
