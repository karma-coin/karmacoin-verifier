// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

extern crate core;
extern crate serde;

pub mod hasher;
pub mod hex_utils;
pub mod logging_service;
pub mod server_config_service;
pub mod tests_helpers;
pub mod verify_number_request;

pub mod karma_coin;

pub const GRPC_DESCRIPTOR: &[u8] = include_bytes!("karma_coin/descriptor.bin");
