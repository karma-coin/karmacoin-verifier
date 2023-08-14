#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendVerificationCodeRequest {
    #[prost(string, tag = "1")]
    pub mobile_number: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SendVerificationCodeResponse {
    #[prost(string, tag = "1")]
    pub session_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyNumberRequest {
    /// serialized VerifyNumberRequestDataEx
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// User signature of binary data field 1
    /// Public key is account_id in the data
    #[prost(bytes = "vec", tag = "2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyNumberResponse {
    /// serialized UserVerificationData. This data should be scale and not protobuf encoded
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// signature over data - should use kc2 signature sheme
    #[prost(string, tag = "2")]
    pub signature: ::prost::alloc::string::String,
    /// verification result
    #[prost(enumeration = "VerificationResult", tag = "3")]
    pub result: i32,
}
/// Created and signed by a verifier to attest that an account owns a mobile number
/// Includes mobile number hash instead of mobile number in response
/// Signature is externally available
/// todo: this should be scale encoded
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserVerificationData {
    #[prost(string, tag = "1")]
    pub verifier_account_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub timestamp: u64,
    #[prost(string, tag = "3")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub mobile_number_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub requested_user_name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub signature: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyNumberRequestData {
    #[prost(uint64, tag = "1")]
    pub timestamp: u64,
    #[prost(string, tag = "2")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub mobile_number: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub requested_user_name: ::prost::alloc::string::String,
    /// optional token to bypass verification
    #[prost(bytes = "vec", tag = "5")]
    pub bypass_token: ::prost::alloc::vec::Vec<u8>,
    /// Auth provider verification code
    #[prost(string, tag = "6")]
    pub verification_code: ::prost::alloc::string::String,
    /// Verification session id
    #[prost(string, tag = "7")]
    pub verification_sid: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VerificationResult {
    Unspecified = 0,
    /// there's already a user with the requested user name
    UserNameTaken = 1,
    /// user is verified using provided token
    Verified = 2,
    /// user is not verifier using provided token
    Unverified = 3,
    /// request is missing required data
    MissingData = 4,
    /// bad client signature
    InvalidSignature = 5,
    /// different account associated with phone number
    AccountMismatch = 6,
}
impl VerificationResult {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VerificationResult::Unspecified => "VERIFICATION_RESULT_UNSPECIFIED",
            VerificationResult::UserNameTaken => "VERIFICATION_RESULT_USER_NAME_TAKEN",
            VerificationResult::Verified => "VERIFICATION_RESULT_VERIFIED",
            VerificationResult::Unverified => "VERIFICATION_RESULT_UNVERIFIED",
            VerificationResult::MissingData => "VERIFICATION_RESULT_MISSING_DATA",
            VerificationResult::InvalidSignature => {
                "VERIFICATION_RESULT_INVALID_SIGNATURE"
            }
            VerificationResult::AccountMismatch => "VERIFICATION_RESULT_ACCOUNT_MISMATCH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VERIFICATION_RESULT_UNSPECIFIED" => Some(Self::Unspecified),
            "VERIFICATION_RESULT_USER_NAME_TAKEN" => Some(Self::UserNameTaken),
            "VERIFICATION_RESULT_VERIFIED" => Some(Self::Verified),
            "VERIFICATION_RESULT_UNVERIFIED" => Some(Self::Unverified),
            "VERIFICATION_RESULT_MISSING_DATA" => Some(Self::MissingData),
            "VERIFICATION_RESULT_INVALID_SIGNATURE" => Some(Self::InvalidSignature),
            "VERIFICATION_RESULT_ACCOUNT_MISMATCH" => Some(Self::AccountMismatch),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod verifier_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// mobile phone numbers verifier api service
    #[derive(Debug, Clone)]
    pub struct VerifierServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl VerifierServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> VerifierServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> VerifierServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            VerifierServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Send verification code to the user's mobile number via whatsapp
        pub async fn send_verification_code(
            &mut self,
            request: impl tonic::IntoRequest<super::SendVerificationCodeRequest>,
        ) -> Result<
            tonic::Response<super::SendVerificationCodeResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.verifier.VerifierService/SendVerificationCode",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// / Verify number using verification provider code
        pub async fn verify_number(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyNumberRequest>,
        ) -> Result<tonic::Response<super::VerifyNumberResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.verifier.VerifierService/VerifyNumber",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod verifier_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with VerifierServiceServer.
    #[async_trait]
    pub trait VerifierService: Send + Sync + 'static {
        /// Send verification code to the user's mobile number via whatsapp
        async fn send_verification_code(
            &self,
            request: tonic::Request<super::SendVerificationCodeRequest>,
        ) -> Result<tonic::Response<super::SendVerificationCodeResponse>, tonic::Status>;
        /// / Verify number using verification provider code
        async fn verify_number(
            &self,
            request: tonic::Request<super::VerifyNumberRequest>,
        ) -> Result<tonic::Response<super::VerifyNumberResponse>, tonic::Status>;
    }
    /// mobile phone numbers verifier api service
    #[derive(Debug)]
    pub struct VerifierServiceServer<T: VerifierService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: VerifierService> VerifierServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for VerifierServiceServer<T>
    where
        T: VerifierService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/karma_coin.verifier.VerifierService/SendVerificationCode" => {
                    #[allow(non_camel_case_types)]
                    struct SendVerificationCodeSvc<T: VerifierService>(pub Arc<T>);
                    impl<
                        T: VerifierService,
                    > tonic::server::UnaryService<super::SendVerificationCodeRequest>
                    for SendVerificationCodeSvc<T> {
                        type Response = super::SendVerificationCodeResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SendVerificationCodeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).send_verification_code(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SendVerificationCodeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.verifier.VerifierService/VerifyNumber" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyNumberSvc<T: VerifierService>(pub Arc<T>);
                    impl<
                        T: VerifierService,
                    > tonic::server::UnaryService<super::VerifyNumberRequest>
                    for VerifyNumberSvc<T> {
                        type Response = super::VerifyNumberResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VerifyNumberRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).verify_number(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = VerifyNumberSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: VerifierService> Clone for VerifierServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: VerifierService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: VerifierService> tonic::server::NamedService for VerifierServiceServer<T> {
        const NAME: &'static str = "karma_coin.verifier.VerifierService";
    }
}
