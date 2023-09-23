// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::verifier::verifier_service::VerifierService;
use anyhow::Result;
use base::karma_coin::karma_coin_verifier::{
    SendVerificationCodeRequest, SendVerificationCodeResponse, SendVerificationCodeResult,
};
use http::{header, StatusCode};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use xactor::*;

#[message(result = "Result<SendVerificationCodeResponse>")]
pub(crate) struct SendVerificationCode(pub SendVerificationCodeRequest);

#[derive(Deserialize, Debug, Clone)]
pub struct OTPVerifyRequest {
    pub sid: String,
    pub status: String,
}

/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<SendVerificationCode> for VerifierService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: SendVerificationCode,
    ) -> Result<SendVerificationCodeResponse> {
        let req = msg.0;

        let number = req.mobile_number.clone();
        info!("sending verification code to: {}", number);

        if number.is_empty() {
            return Ok(create_response(
                SendVerificationCodeResult::InvalidUserData,
                Some("Missing mobile number".into()),
                None,
            ));
        }

        let url = format!(
            "https://verify.twilio.com/v2/Services/{serv_id}/Verifications",
            serv_id = self.twilio_service_id.as_ref().unwrap(),
        );

        let whatsapp: String = "whatsapp".to_string();

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let mut form_body: HashMap<&str, &String> = HashMap::new();
        form_body.insert("To", &number);
        form_body.insert("Channel", &whatsapp);

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

        return match res {
            Ok(response) => {
                if response.status() != StatusCode::CREATED {
                    info!(
                        "failed. twilio response status code != 201: {}",
                        response.status()
                    );
                    return Ok(create_response(
                        SendVerificationCodeResult::Failed,
                        Some("Code verifier failed to send".into()),
                        None,
                    ));
                }

                let data = response.json::<OTPVerifyRequest>().await;
                return match data {
                    Ok(result) => {
                        info!(
                            "Send verification code via whatsapp. Session id: {}. Status: {}",
                            result.sid, result.status
                        );
                        Ok(create_response(
                            SendVerificationCodeResult::Sent,
                            None,
                            Some(result.sid),
                        ))
                    }
                    Err(e) => {
                        info!("error parsing twilio resp: {}", e);
                        Ok(create_response(
                            SendVerificationCodeResult::Failed,
                            Some("Unexpected code verifier api response".into()),
                            None,
                        ))
                    }
                };
            }
            Err(e) => {
                info!("error calling twilio: {}", e);
                Ok(create_response(
                    SendVerificationCodeResult::Failed,
                    Some("Failed to call code verifier api".into()),
                    None,
                ))
            }
        };
    }
}

/// Helper method to create a response from data
fn create_response(
    result: SendVerificationCodeResult,
    error_message: Option<String>,
    session_id: Option<String>,
) -> SendVerificationCodeResponse {
    SendVerificationCodeResponse {
        result: result as i32,
        session_id: session_id.unwrap_or("".into()),
        error_message: error_message.unwrap_or("".into()),
    }
}
