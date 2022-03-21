use std::collections::HashMap;

use crate::types::{Config, ProcessedConfig, RoutingConfig};

pub fn get_processed_config(config: &Config) -> ProcessedConfig {
    ProcessedConfig {
        servers: config
            .servers
            .iter()
            .map(|i| (String::from(&i.name), i.clone()))
            .collect::<HashMap<_, _>>(),
        routes: config
            .routes
            .iter()
            .map(|i| (String::from(&i.name), i.clone()))
            .collect::<HashMap<_, _>>(),
    }
}

pub fn get_matched_route_and_server(
    path: &str,
    config: &ProcessedConfig,
) -> Result<RoutingConfig, String> {
    let found_route = config.routes.values().find(|&x| x.path == path);
    let unwrapped_route = found_route.ok_or_else(|| String::from("no matching route"))?;

    let found_server = config.servers.get(&unwrapped_route.server);
    let unwrapped_server = found_server.ok_or_else(|| String::from("no matching server"))?;

    return Ok(RoutingConfig {
        route: unwrapped_route.clone(),
        server: unwrapped_server.clone(),
    });
}
