// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::verifier::send_verification_code::SendVerificationCode;
use crate::services::verifier::verify_number::Verify;
use anyhow::Result;
use base::karma_coin::karma_coin_verifier::verifier_service_server::VerifierService as VerifierServiceTrait;
use base::karma_coin::karma_coin_verifier::{
    SendVerificationCodeRequest, SendVerificationCodeResponse, VerifyNumberRequest,
    VerifyNumberResponse,
};
use tonic::{Request, Response, Status};
use xactor::*;

/// ApiService is a system service that provides access to provider server persisted data as well as an interface to admin the provider's server. It provides a GRPC admin service defined in ServerAdminService. This service is designed to be used by provider admin clients.
#[derive(Debug)]
pub(crate) struct VerifierService {
    // key_pair: Option<KeyPair>,
}

impl Default for VerifierService {
    fn default() -> Self {
        info!("Verifier Service created");
        VerifierService {}
    }
}

#[async_trait::async_trait]
impl Actor for VerifierService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("VerifierService started");

        Ok(())
    }
}

impl Service for VerifierService {}

impl VerifierService {
    // Returns the verifier account id
    /*
    pub(crate) async fn get_account_id(&mut self) -> Result<AccountId> {
        let key_pair = self.get_key_pair().await?;
        Ok(AccountId {
            data: key_pair
                .public_key
                .as_ref()
                .ok_or_else(|| anyhow!("No public key"))?
                .key
                .to_vec(),
        })
    }*/

    // Returns the verifier id key pair
    /*
    pub(crate) async fn get_key_pair(&mut self) -> Result<KeyPair> {
        if let Some(key_pair) = &self.key_pair {
            info!(
                "returning cached verifier id key-pair. account id: {:?}",
                short_hex_string(&key_pair.public_key.as_ref().unwrap().key)
            );
            return Ok(key_pair.clone());
        }

        let key_pair: KeyPair = ServerConfigService::from_registry()
            .await?
            .call(GetVerifierIdKeyPair)
            .await??;

        info!(
            "got key-pair from config service. Verifier account id: {:?}",
            short_hex_string(key_pair.public_key.as_ref().unwrap().key.as_slice())
        );

        self.key_pair = Some(key_pair.clone());
        Ok(key_pair)
    }*/
}

#[tonic::async_trait]
impl VerifierServiceTrait for VerifierService {
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
            Ok(_data) => {
                info!("verification successful");
                Ok(Response::new(VerifyNumberResponse {
                    data: vec![],
                    signature: "".to_string(),
                    result: 0,
                }))
            }
            Err(e) => Err(Status::internal(format!("internal error: {:?}", e))),
        }
    }

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
            Ok(session_id) => {
                info!("Code sent, session id: {}", session_id);
                Ok(Response::new(SendVerificationCodeResponse { session_id }))
            }
            Err(e) => Err(Status::internal(format!("internal error: {:?}", e))),
        }
    }
}
