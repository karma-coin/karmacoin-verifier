// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

#[macro_use]
extern crate log;
extern crate base;
extern crate clap;

use base::logging_service::{InitLogger, LoggingService};
use server::server_service::{ServerService, Startup};
use tokio::signal;

use clap::{App, Arg};

use xactor::*;

// Start a client app - good for testability / integration testing
pub async fn start() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let _matches = App::new("Karmachain 2.0 Verifier")
        .version("0.2.0")
        .author("AE  <a@karmaco.in>")
        .about("The coin for all of us")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .takes_value(true)
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    // Start app logger
    let logging = LoggingService::from_registry().await.unwrap();
    let _ = logging
        .call(InitLogger {
            peer_name: "Karmachain 2.0 Verifier".into(),
            brief: false, // todo: take from config
        })
        .await
        .unwrap();

    // Start network server
    let server = ServerService::from_registry().await.unwrap();

    server.call(Startup {}).await??;

    // test logging
    info!("Services started");

    signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl-c signal");

    debug!("stopping server-app via ctrl-c signal...");
    spawn(async {
        debug!("resources cleanup completed");
    })
    .await
    .unwrap();

    Ok(())
}
