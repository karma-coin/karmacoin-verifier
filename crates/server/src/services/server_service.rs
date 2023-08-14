// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::verifier::verifier_service::VerifierService;
use anyhow::Result;
use base::server_config_service::SetConfigFile;
use base::server_config_service::{
    ServerConfigService, GRPC_SERVER_HOST_CONFIG_KEY, GRPC_SERVER_HOST_PORT_CONFIG_KEY,
    SERVER_NAME_CONFIG_KEY,
};

use tonic::transport::*;

use base::karma_coin::karma_coin_verifier::verifier_service_server::VerifierServiceServer;
use tonic_web::GrpcWebLayer;
use tower_http::cors::CorsLayer;
use xactor::*;

/// ServerService is a full node p2p network server
/// todo: ServerService should maintain node id identity (for protocol purposes)
#[derive(Default)]
pub struct ServerService {}

#[async_trait::async_trait]
impl Actor for ServerService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        // start the config services to config db, blockchain and the server
        let server_config_service = ServerConfigService::from_registry().await?;

        server_config_service
            .call(SetConfigFile {
                config_file: "./config.yaml".to_string(),
            })
            .await??;

        VerifierService::from_registry().await?;

        info!("started");
        Ok(())
    }
}

impl Service for ServerService {}

///////////////////////////

#[message(result = "Result<()>")]
pub struct Startup;

/// Start the grpc server
#[async_trait::async_trait]
impl Handler<Startup> for ServerService {
    async fn handle(&mut self, _ctx: &mut Context<Self>, _msg: Startup) -> Result<()> {
        info!("configuring server...");

        let server_name = ServerConfigService::get(SERVER_NAME_CONFIG_KEY.into())
            .await?
            .unwrap();

        let host = ServerConfigService::get(GRPC_SERVER_HOST_CONFIG_KEY.into())
            .await?
            .unwrap();

        let port = ServerConfigService::get_u64(GRPC_SERVER_HOST_PORT_CONFIG_KEY.into())
            .await?
            .unwrap() as u32;

        self.start_grpc_server(port, host, server_name).await?;

        info!("KC2 verifier grpc server started");

        Ok(())
    }
}

impl ServerService {
    /// Starts the server's grpc services
    async fn start_grpc_server(&self, port: u32, host: String, peer_name: String) -> Result<()> {
        // setup grpc server and services
        let grpc_server_addr = format!("{}:{}", host, port).parse()?;
        info!(
            "starting {} grpc server on: {}",
            peer_name, grpc_server_addr
        );

        let (mut verifier_health_reporter, verifier_health_service) =
            tonic_health::server::health_reporter();

        verifier_health_reporter
            .set_serving::<VerifierServiceServer<VerifierService>>()
            .await;

        let reflection_server = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(base::GRPC_DESCRIPTOR)
            .build()?;

        spawn(async move {
            // this only return when server is stopped due to error or shutdown
            let router = Server::builder()
                .accept_http1(true)
                //.tls_config(tls).unwrap()
                .layer(CorsLayer::very_permissive())
                .layer(GrpcWebLayer::new())
                .add_service(reflection_server)
                .add_service(verifier_health_service)
                .add_service(VerifierServiceServer::new(VerifierService::default()));

            let res = router.serve(grpc_server_addr).await;

            if res.is_err() {
                info!("grpc server stopped due to error: {:?}", res.err().unwrap());
            } else {
                info!("grpc server stopped");
            }
        });

        Ok(())
    }
}
