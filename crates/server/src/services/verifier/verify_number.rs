// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::verifier::verifier_service::VerifierService;
use anyhow::Result;
use base::karma_coin::karma_coin_verifier::{
    VerificationResult, VerifyNumberRequest, VerifyNumberRequestData, VerifyNumberResponse,
};
use base::server_config_service::ServerConfigService;
use ed25519_dalek::Verifier;
use http::{header, StatusCode};
use prost::Message;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use xactor::*;

#[message(result = "Result<VerifyNumberResponse>")]
pub(crate) struct Verify(pub VerifyNumberRequest);

#[derive(Deserialize, Debug, Clone)]
pub struct OTPVerifyResponse {
    pub status: String,
    pub sid: String,
}
/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<Verify> for VerifierService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: Verify,
    ) -> Result<VerifyNumberResponse> {
        let req = msg.0;

        info!("verify phone number called");

        // decode request data
        let user_data = match VerifyNumberRequestData::decode(req.data.as_ref()) {
            Ok(user_data) => user_data,
            Err(_) => {
                return gen_verification_result(VerificationResult::MissingData).await;
            }
        };

        if user_data.account_id.is_empty() {
            return gen_verification_result(VerificationResult::MissingData).await;
        }

        if user_data.user_name.is_empty() {
            return gen_verification_result(VerificationResult::MissingData).await;
        }

        if user_data.phone_number.is_empty() {
            return gen_verification_result(VerificationResult::MissingData).await;
        }

        // verify request signature
        use ed25519_dalek::ed25519::signature::Signature;
        let signature = &Signature::from_bytes(req.signature.as_ref()).unwrap();
        // todo: extract public key from ss58 user account id
        let pub_key = &ed25519_dalek::PublicKey::from_bytes(&[0x1, 0x0]).unwrap();

        // verify request data signature
        if pub_key.verify(req.data.as_ref(), signature).is_err() {
            return gen_verification_result(VerificationResult::InvalidSignature).await;
        };

        let bypass_token = ServerConfigService::get("verifier.bypass_token".into())
            .await?
            .unwrap();

        // call auth service unless bypass token was provided and matches the configured one
        if !user_data.bypass_token.eq(&bypass_token) {
            // verify code

            let url = format!(
                "https://verify.twilio.com/v2/Services/{serv_id}/VerificationCheck",
                serv_id = self.twilio_service_id.as_ref().unwrap(),
            );

            let mut headers = header::HeaderMap::new();
            headers.insert(
                "Content-Type",
                "application/x-www-form-urlencoded".parse().unwrap(),
            );

            let mut form_body: HashMap<&str, &String> = HashMap::new();
            form_body.insert("To", &user_data.phone_number);
            form_body.insert("Code", &user_data.verification_code);

            let client = Client::new();
            let res = client
                .post(url)
                .basic_auth(
                    self.twilio_account_id.as_ref().unwrap(),
                    Some(self.twilio_token.as_ref().unwrap()),
                )
                .headers(headers)
                .form(&form_body)
                .send()
                .await;

            match res {
                Ok(response) => {
                    if response.status() != StatusCode::OK {
                        info!("twilio response status code != 200");
                        return gen_verification_result(VerificationResult::Failed).await;
                    }

                    let data = response.json::<OTPVerifyResponse>().await;
                    match data {
                        Ok(result) => {
                            if result.status == "approved" {
                                // validate sid
                                if result.sid != user_data.verification_sid {
                                    info!("twilio sid mismatch");
                                    return gen_verification_result(
                                        VerificationResult::MissingData,
                                    )
                                    .await;
                                }
                                info!("Twilio approved code!");
                            } else {
                                info!("Twilio result != approved: {}", result.status);
                                return gen_verification_result(VerificationResult::Failed).await;
                            }
                        }
                        Err(e) => {
                            info!("error parsing twilio resp: {}", e);
                            return gen_verification_result(VerificationResult::Failed).await;
                        }
                    }
                }
                Err(e) => {
                    info!("error calling twilio: {}", e);
                    return gen_verification_result(VerificationResult::Failed).await;
                }
            }
        }

        // todo: generate scale-encoded signed data to be included in signup tx

        let response = VerifyNumberResponse {
            data: vec![],
            result: VerificationResult::Verified as i32,
        };

        info!("Returning verification response");
        Ok(response)
    }
}

/// private helper function to generate a failure result
async fn gen_verification_result(result: VerificationResult) -> Result<VerifyNumberResponse> {
    Ok(VerifyNumberResponse {
        data: vec![],
        result: result as i32,
    })
}
