// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::verifier::send_verification_code::SendVerificationCode;
use crate::services::verifier::verify_number::Verify;
use anyhow::Result;
use base::hex_utils::hex_string;
use base::karma_coin::karma_coin_verifier::verifier_service_server::VerifierService as VerifierServiceTrait;
use base::karma_coin::karma_coin_verifier::{
    SendVerificationCodeRequest, SendVerificationCodeResponse, SendVerificationCodeResult,
    VerifyNumberRequest, VerifyNumberResponse,
};
use base::server_config_service::ServerConfigService;
use sp_core::ed25519::Pair as ED25519;
use sp_core::*;
use tonic::{Request, Response, Status};
use xactor::*;

/// ApiService is a system service that provides access to provider server persisted data as well as an interface to admin the provider's server. It provides a GRPC admin service defined in ServerAdminService. This service is designed to be used by provider admin clients.
pub(crate) struct VerifierService {
    pub(crate) twilio_account_id: Option<String>,
    pub(crate) twilio_service_id: Option<String>,
    pub(crate) twilio_token: Option<String>,
    /// verifier key pair - generated on startup
    pub(crate) key_pair: Option<ED25519>,
}

impl Default for VerifierService {
    fn default() -> Self {
        info!("Verifier Service created");
        VerifierService {
            twilio_account_id: None,
            twilio_service_id: None,
            twilio_token: None,
            key_pair: None,
        }
    }
}

#[async_trait::async_trait]
impl Actor for VerifierService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        // generate verifier identity from config secrets
        let phrase = ServerConfigService::get("verifier.key_mnemonic".into())
            .await?
            .unwrap();

        let password = ServerConfigService::get("verifier.key_password".into())
            .await?
            .unwrap();

        let (pair, _) = ED25519::from_phrase(&phrase, Some(&password)).unwrap();

        info!(
            "Verifier identity public key raw: {}",
            hex_string(&pair.public().to_raw_vec())
        );

        info!("Verifier identity public key ss58: {}", pair.public());

        // Store identity for signing messages
        self.key_pair = Some(pair);

        self.twilio_account_id = Some(
            ServerConfigService::get("twilio.account_sid".into())
                .await?
                .unwrap(),
        );

        self.twilio_service_id = Some(
            ServerConfigService::get("twilio.service_id".into())
                .await?
                .unwrap(),
        );

        self.twilio_token = Some(
            ServerConfigService::get("twilio.auth_token".into())
                .await?
                .unwrap(),
        );

        info!("Verifier service initialized and started");

        Ok(())
    }
}

impl Service for VerifierService {}

#[tonic::async_trait]
impl VerifierServiceTrait for VerifierService {
    async fn send_verification_code(
        &self,
        request: Request<SendVerificationCodeRequest>,
    ) -> std::result::Result<Response<SendVerificationCodeResponse>, Status> {
        let service = VerifierService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {:?}", e)))?;

        match service
            .call(SendVerificationCode(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call verifier api: {:?}", e)))?
        {
            Ok(resp) => {
                info!("Code verification response: {:?}", resp.result);
                if resp.result == SendVerificationCodeResult::Sent as i32 {
                    info!("Code sent, session id: {}", resp.session_id);
                }
                Ok(Response::new(resp))
            }
            Err(e) => Err(Status::internal(format!("internal error: {:?}", e))),
        }
    }

    /// User requests to verify a number with code received via text message
    async fn verify_number(
        &self,
        request: Request<VerifyNumberRequest>,
    ) -> Result<Response<VerifyNumberResponse>, Status> {
        let service = VerifierService::from_registry()
            .await
            .map_err(|e| Status::internal(format!("internal error: {:?}", e)))?;

        match service
            .call(Verify(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call verifier api: {:?}", e)))?
        {
            Ok(resp) => {
                info!("verification successful");
                Ok(Response::new(resp))
            }
            Err(e) => Err(Status::internal(format!("internal error: {:?}", e))),
        }
    }
}
