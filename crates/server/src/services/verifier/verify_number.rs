// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::verifier::verifier_service::VerifierService;
use anyhow::Result;
use base::karma_coin::karma_coin_verifier::{
    VerificationResult, VerifyNumberRequest, VerifyNumberRequestData, VerifyNumberResponse,
};
use base::server_config_service::ServerConfigService;
use http::{header, StatusCode};
use prost::Message;
use reqwest::Client;
use serde::Deserialize;
use sp_core::{
    crypto::{AccountId32, Ss58Codec},
    ed25519::{Pair, Public, Signature},
    Encode, Pair as PairT,
};
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

        let signature = Signature::from_slice(req.signature.as_ref()).unwrap();
        let account_id = AccountId32::from_ss58check(&user_data.account_id).unwrap();
        let pub_key = Public::from_raw(*account_id.as_ref());

        // verify request data signature
        if !Pair::verify(&signature, req.data, &pub_key) {
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

        let phone_number_hash =
            sp_core::hashing::blake2_512(user_data.phone_number.clone().as_bytes());
        let verification_evidence = sp_rpc::verifier::VerificationEvidence {
            verifier_public_key: self.key_pair.unwrap().public(),
            account_id: account_id,
            username: user_data.user_name,
            phone_number_hash: phone_number_hash,
        };
        let bytes = verification_evidence.encode();

        let response = VerifyNumberResponse {
            data: self.key_pair.unwrap().sign(&bytes).0.to_vec(),
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
