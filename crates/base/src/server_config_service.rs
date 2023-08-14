// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::{anyhow, Result};
use config::builder::DefaultState;
use config::{Config, ConfigBuilder};
use log::*;
use std::path::Path;
use xactor::*;

// Verifier data
pub const VERIFIER_NAME: &str = "verifier.name";
pub const VERIFIER_ID_PRIVATE_KEY: &str = "verifier.private_key";
pub const VERIFIER_ID_PUBLIC_KEY: &str = "verifier.public_key";

pub const DEFAULT_GRPC_SERVER_PORT: i64 = 9080;
pub const DEFAULT_GRPC_ADMIN_PORT: i64 = 9888;
pub const DEFAULT_START_GRPC_SERVER: bool = true;

/// Start the verification service

/// ConfigService for servers

pub const SERVER_NAME_CONFIG_KEY: &str = "server_name";
pub const GRPC_SERVER_HOST_CONFIG_KEY: &str = "grpc_host";
pub const GRPC_SERVER_HOST_PORT_CONFIG_KEY: &str = "grpc_host_port";
pub const GRPC_ADMIN_PORT_CONFIG_KEY: &str = "grpc_admin_port";

// private identity key (ed25519)

pub struct ServerConfigService {
    config: Config,
    config_file: Option<String>,
}

impl ServerConfigService {
    fn get_default_builder(&self) -> ConfigBuilder<DefaultState> {
        Config::builder()
            .set_default(GRPC_SERVER_HOST_PORT_CONFIG_KEY, DEFAULT_GRPC_SERVER_PORT)
            .unwrap()
            .set_default(GRPC_ADMIN_PORT_CONFIG_KEY, DEFAULT_GRPC_ADMIN_PORT)
            .unwrap()
            .set_default(GRPC_SERVER_HOST_CONFIG_KEY, "[::]")
            .unwrap()
            // we always want to have a peer name - even a generic one
            .set_default(SERVER_NAME_CONFIG_KEY, "Karmachain1.0")
            .unwrap()
    }
}

#[async_trait::async_trait]
impl Actor for ServerConfigService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        let mut builder = self.get_default_builder();

        // load configs from file if it was set
        if let Some(config_file) = &self.config_file {
            builder = builder.add_source(config::File::with_name(config_file));

            #[allow(deprecated)]
            self.config
                .merge(config::File::with_name(config_file))
                .unwrap();

            info!("merged server config file: {}", config_file);
        }

        // todo: if id private key not set then generate random keypair and store private key
        self.config = builder.build().unwrap();

        info!("service started");

        Ok(())
    }
}

impl Service for ServerConfigService {}

impl Default for ServerConfigService {
    fn default() -> Self {
        info!("Service created");
        ServerConfigService {
            config: Config::default(),
            config_file: None,
        }
    }
}

// helpers
impl ServerConfigService {
    pub async fn get(key: String) -> Result<Option<String>> {
        //info!("Get config value for key: {}", key);
        let config = ServerConfigService::from_registry().await?;
        // info!("got service");
        let res = config.call(GetValue(key)).await?;
        // info!("got value for key: {:?}", res);
        Ok(res)
    }

    // helper
    pub async fn get_bool(key: String) -> Result<Option<bool>> {
        let config = ServerConfigService::from_registry().await?;
        let res = config.call(GetBool(key)).await?;
        Ok(res)
    }

    // helper
    pub async fn get_u64(key: String) -> Result<Option<u64>> {
        let config = ServerConfigService::from_registry().await?;
        let res = config.call(GetU64(key)).await?;
        Ok(res)
    }

    pub async fn set(key: String, value: String) -> Result<()> {
        let config = ServerConfigService::from_registry().await?;
        config.call(SetValue { key, value }).await?
    }

    // helper
    pub async fn set_bool(key: String, value: bool) -> Result<()> {
        let config = ServerConfigService::from_registry().await?;
        config.call(SetBool { key, value }).await?
    }

    // helper
    pub async fn set_u64(key: String, value: u64) -> Result<()> {
        let config = ServerConfigService::from_registry().await?;
        config.call(SetU64 { key, value }).await?
    }
}

/*
#[message(result = "Result<KeyPair>")]
pub struct GetVerifierIdKeyPair;

#[async_trait::async_trait]
impl Handler<GetVerifierIdKeyPair> for ServerConfigService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        _msg: GetVerifierIdKeyPair,
    ) -> Result<KeyPair> {
        match self.config.get_string(VERIFIER_ID_PRIVATE_KEY) {
            Ok(data) => {
                let private_key_data = hex_from_string(data).unwrap();
                match self.config.get_string(VERIFIER_ID_PUBLIC_KEY) {
                    Ok(pub_data) => {
                        let pub_key_data = hex_from_string(pub_data).unwrap();
                        Ok(KeyPair {
                            private_key: Some(PrivateKey {
                                key: private_key_data,
                            }),
                            public_key: Some(PublicKey { key: pub_key_data }),
                            scheme: 0,
                        })
                    }
                    Err(_) => {
                        panic!("invalid config file: missing verifier public key when private key is provided");
                    }
                }
            }
            Err(_) => {
                panic!("expected verifier private key via config");
            }
        }
    }
}*/

#[message(result = "Result<()>")]
pub struct SetConfigFile {
    pub config_file: String,
}

#[async_trait::async_trait]
impl Handler<SetConfigFile> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetConfigFile) -> Result<()> {
        info!("Current dir: {:?}", std::env::current_dir().unwrap());
        if !Path::new(&msg.config_file).exists() {
            warn!(
                "using default config. Requested config file {:?} does not exist",
                msg.config_file.as_str()
            );
            return Ok(());
        }

        let builder = self.get_default_builder();

        self.config = builder
            .add_source(config::File::with_name(&msg.config_file))
            .build()
            .unwrap();

        // save config file so it can be used if we need to reload config
        self.config_file = Some(msg.config_file.clone());

        info!(
            "merged content of server config file {:?}",
            msg.config_file.as_str()
        );

        Ok(())
    }
}

#[message(result = "Option<bool>")]
pub struct GetBool(pub String);

#[async_trait::async_trait]
impl Handler<GetBool> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: GetBool) -> Option<bool> {
        match self.config.get_bool(&msg.0.as_str()) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
}

#[message(result = "Option<u64>")]
pub struct GetU64(pub String);

#[async_trait::async_trait]
impl Handler<GetU64> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: GetU64) -> Option<u64> {
        match self.config.get_int(&msg.0.as_str()) {
            Ok(res) => Some(res as u64),
            Err(_) => None,
        }
    }
}

#[message(result = "Option<String>")]
pub struct GetValue(pub String);

#[async_trait::async_trait]
impl Handler<GetValue> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: GetValue) -> Option<String> {
        // info!("Getting value for key {:?}", msg.0.as_str());
        match self.config.get_string(&msg.0.as_str()) {
            Ok(res) => Some(res),
            Err(_) => None,
        }
    }
}

#[message(result = "Result<()>")]
pub struct SetValue {
    pub key: String,
    pub value: String,
}

#[async_trait::async_trait]
impl Handler<SetValue> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetValue) -> Result<()> {
        #[allow(deprecated)]
        match self.config.set(msg.key.as_str(), msg.value) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }
}

#[message(result = "Result<()>")]
pub struct SetU64 {
    pub key: String,
    pub value: u64,
}

#[async_trait::async_trait]
impl Handler<SetU64> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetU64) -> Result<()> {
        #[allow(deprecated)]
        match self.config.set(msg.key.as_str(), msg.value.to_string()) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }
}

#[message(result = "Result<()>")]
pub struct SetBool {
    pub key: String,
    pub value: bool,
}

#[async_trait::async_trait]
impl Handler<SetBool> for ServerConfigService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, msg: SetBool) -> Result<()> {
        #[allow(deprecated)]
        match self.config.set(msg.key.as_str(), msg.value) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("{:?}", e)),
        }
    }
}
