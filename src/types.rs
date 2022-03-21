use std::collections::HashMap;

use hyper::{client::HttpConnector, Body, Client};
use serde_derive::Deserialize;

#[derive(Clone, Debug)]
pub struct ProcessedConfig {
    pub servers: HashMap<String, Server>,
    pub routes: HashMap<String, Route>,
}

#[derive(Clone, Debug)]
pub struct RoutingConfig {
    pub route: Route,
    pub server: Server,
}

pub struct State {
    pub client: Client<HttpConnector, Body>,
    pub processed_config: ProcessedConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Server {
    pub endpoint: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Route {
    pub name: String,
    pub path: String,
    pub method: String,
    pub server: String,
    pub server_path: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub servers: Vec<Server>,
    pub routes: Vec<Route>,
}
