#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShopCustomizationResponse {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub created_at: u64,
    #[prost(uint64, tag = "4")]
    pub updated_at: u64,
    #[prost(string, optional, tag = "5")]
    pub logo_image_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub banner_image_url: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7")]
    pub header_background_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "8")]
    pub header_background_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "9")]
    pub header_content_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "10")]
    pub header_content_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "11")]
    pub secondary_background_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "12")]
    pub secondary_background_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "13")]
    pub secondary_content_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "14")]
    pub secondary_content_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutShopCustomizationRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub header_background_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "3")]
    pub header_background_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "4")]
    pub header_content_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "5")]
    pub header_content_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "6")]
    pub secondary_background_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "7")]
    pub secondary_background_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "8")]
    pub secondary_content_color_light: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
    #[prost(string, optional, tag = "9")]
    pub secondary_content_color_dark: ::core::option::Option<
        ::prost::alloc::string::String,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutShopCustomizationResponse {
    #[prost(message, optional, tag = "1")]
    pub shop_customization: ::core::option::Option<ShopCustomizationResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopCustomizationRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopCustomizationResponse {
    #[prost(message, optional, tag = "1")]
    pub shop_customization: ::core::option::Option<ShopCustomizationResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShopCustomizationRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteShopCustomizationResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutBannerImageToShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<super::super::media::v1::MediaUpload>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutBannerImageToShopResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveBannerImageFromShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveBannerImageFromShopResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutLogoImageToShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<super::super::media::v1::MediaUpload>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutLogoImageToShopResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLogoImageFromShopRequest {
    #[prost(string, tag = "1")]
    pub shop_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveLogoImageFromShopResponse {}
/// Generated server implementations.
pub mod shop_customization_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ShopCustomizationServiceServer.
    #[async_trait]
    pub trait ShopCustomizationService: Send + Sync + 'static {
        async fn put_shop_customization(
            &self,
            request: tonic::Request<super::PutShopCustomizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutShopCustomizationResponse>,
            tonic::Status,
        >;
        async fn get_shop_customization(
            &self,
            request: tonic::Request<super::GetShopCustomizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetShopCustomizationResponse>,
            tonic::Status,
        >;
        async fn delete_shop_customization(
            &self,
            request: tonic::Request<super::DeleteShopCustomizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteShopCustomizationResponse>,
            tonic::Status,
        >;
        async fn put_banner_image_to_shop(
            &self,
            request: tonic::Request<super::PutBannerImageToShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutBannerImageToShopResponse>,
            tonic::Status,
        >;
        async fn remove_banner_image_from_shop(
            &self,
            request: tonic::Request<super::RemoveBannerImageFromShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveBannerImageFromShopResponse>,
            tonic::Status,
        >;
        async fn put_logo_image_to_shop(
            &self,
            request: tonic::Request<super::PutLogoImageToShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutLogoImageToShopResponse>,
            tonic::Status,
        >;
        async fn remove_logo_image_from_shop(
            &self,
            request: tonic::Request<super::RemoveLogoImageFromShopRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveLogoImageFromShopResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct ShopCustomizationServiceServer<T: ShopCustomizationService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ShopCustomizationService> ShopCustomizationServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>>
    for ShopCustomizationServiceServer<T>
    where
        T: ShopCustomizationService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/peoplesmarkets.commerce.v1.ShopCustomizationService/PutShopCustomization" => {
                    #[allow(non_camel_case_types)]
                    struct PutShopCustomizationSvc<T: ShopCustomizationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ShopCustomizationService,
                    > tonic::server::UnaryService<super::PutShopCustomizationRequest>
                    for PutShopCustomizationSvc<T> {
                        type Response = super::PutShopCustomizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutShopCustomizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).put_shop_customization(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PutShopCustomizationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.ShopCustomizationService/GetShopCustomization" => {
                    #[allow(non_camel_case_types)]
                    struct GetShopCustomizationSvc<T: ShopCustomizationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ShopCustomizationService,
                    > tonic::server::UnaryService<super::GetShopCustomizationRequest>
                    for GetShopCustomizationSvc<T> {
                        type Response = super::GetShopCustomizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetShopCustomizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_shop_customization(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetShopCustomizationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.ShopCustomizationService/DeleteShopCustomization" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteShopCustomizationSvc<T: ShopCustomizationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ShopCustomizationService,
                    > tonic::server::UnaryService<super::DeleteShopCustomizationRequest>
                    for DeleteShopCustomizationSvc<T> {
                        type Response = super::DeleteShopCustomizationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::DeleteShopCustomizationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_shop_customization(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteShopCustomizationSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.ShopCustomizationService/PutBannerImageToShop" => {
                    #[allow(non_camel_case_types)]
                    struct PutBannerImageToShopSvc<T: ShopCustomizationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ShopCustomizationService,
                    > tonic::server::UnaryService<super::PutBannerImageToShopRequest>
                    for PutBannerImageToShopSvc<T> {
                        type Response = super::PutBannerImageToShopResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutBannerImageToShopRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).put_banner_image_to_shop(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PutBannerImageToShopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.ShopCustomizationService/RemoveBannerImageFromShop" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveBannerImageFromShopSvc<T: ShopCustomizationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ShopCustomizationService,
                    > tonic::server::UnaryService<
                        super::RemoveBannerImageFromShopRequest,
                    > for RemoveBannerImageFromShopSvc<T> {
                        type Response = super::RemoveBannerImageFromShopResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RemoveBannerImageFromShopRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_banner_image_from_shop(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveBannerImageFromShopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.ShopCustomizationService/PutLogoImageToShop" => {
                    #[allow(non_camel_case_types)]
                    struct PutLogoImageToShopSvc<T: ShopCustomizationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ShopCustomizationService,
                    > tonic::server::UnaryService<super::PutLogoImageToShopRequest>
                    for PutLogoImageToShopSvc<T> {
                        type Response = super::PutLogoImageToShopResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutLogoImageToShopRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).put_logo_image_to_shop(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PutLogoImageToShopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.ShopCustomizationService/RemoveLogoImageFromShop" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveLogoImageFromShopSvc<T: ShopCustomizationService>(
                        pub Arc<T>,
                    );
                    impl<
                        T: ShopCustomizationService,
                    > tonic::server::UnaryService<super::RemoveLogoImageFromShopRequest>
                    for RemoveLogoImageFromShopSvc<T> {
                        type Response = super::RemoveLogoImageFromShopResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RemoveLogoImageFromShopRequest,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_logo_image_from_shop(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveLogoImageFromShopSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
    impl<T: ShopCustomizationService> Clone for ShopCustomizationServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: ShopCustomizationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ShopCustomizationService> tonic::server::NamedService
    for ShopCustomizationServiceServer<T> {
        const NAME: &'static str = "peoplesmarkets.commerce.v1.ShopCustomizationService";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketBoothResponse {
    #[prost(string, tag = "1")]
    pub market_booth_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub created_at: i64,
    #[prost(int64, tag = "4")]
    pub updated_at: i64,
    #[prost(string, tag = "5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub slug: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "7")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, tag = "8")]
    pub platform_fee_percent: u32,
    #[prost(uint32, tag = "9")]
    pub minimum_platform_fee_cent: u32,
    #[prost(message, optional, tag = "10")]
    pub customization: ::core::option::Option<ShopCustomizationResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMarketBoothRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub slug: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub platform_fee_percent: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub minimum_platform_fee_cent: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateMarketBoothResponse {
    #[prost(message, optional, tag = "1")]
    pub market_booth: ::core::option::Option<MarketBoothResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMarketBoothRequest {
    #[prost(string, tag = "1")]
    pub market_booth_id: ::prost::alloc::string::String,
    #[prost(bool, optional, tag = "2")]
    pub extended: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMarketBoothResponse {
    #[prost(message, optional, tag = "1")]
    pub market_booth: ::core::option::Option<MarketBoothResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopBySlugRequest {
    #[prost(string, tag = "1")]
    pub slug: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetShopBySlugResponse {
    #[prost(message, optional, tag = "1")]
    pub market_booth: ::core::option::Option<MarketBoothResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketBoothsOrderBy {
    #[prost(enumeration = "MarketBoothsOrderByField", tag = "1")]
    pub field: i32,
    #[prost(enumeration = "super::super::ordering::v1::Direction", tag = "2")]
    pub direction: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketBoothsFilter {
    #[prost(enumeration = "MarketBoothsFilterField", tag = "1")]
    pub field: i32,
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShopsRequest {
    #[prost(string, optional, tag = "1")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
    #[prost(message, optional, tag = "3")]
    pub order_by: ::core::option::Option<MarketBoothsOrderBy>,
    #[prost(message, optional, tag = "4")]
    pub filter: ::core::option::Option<MarketBoothsFilter>,
    #[prost(bool, optional, tag = "5")]
    pub extended: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListShopsResponse {
    #[prost(message, repeated, tag = "1")]
    pub market_booths: ::prost::alloc::vec::Vec<MarketBoothResponse>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMarketBoothRequest {
    #[prost(string, tag = "1")]
    pub market_booth_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub slug: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "5")]
    pub platform_fee_percent: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub minimum_platform_fee_cent: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateMarketBoothResponse {
    #[prost(message, optional, tag = "1")]
    pub market_booth: ::core::option::Option<MarketBoothResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMarketBoothRequest {
    #[prost(string, tag = "1")]
    pub market_booth_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteMarketBoothResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MarketBoothsOrderByField {
    Unspecified = 0,
    CreatedAt = 1,
    UpdatedAt = 2,
    Name = 3,
    Random = 4,
}
impl MarketBoothsOrderByField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MarketBoothsOrderByField::Unspecified => {
                "MARKET_BOOTHS_ORDER_BY_FIELD_UNSPECIFIED"
            }
            MarketBoothsOrderByField::CreatedAt => {
                "MARKET_BOOTHS_ORDER_BY_FIELD_CREATED_AT"
            }
            MarketBoothsOrderByField::UpdatedAt => {
                "MARKET_BOOTHS_ORDER_BY_FIELD_UPDATED_AT"
            }
            MarketBoothsOrderByField::Name => "MARKET_BOOTHS_ORDER_BY_FIELD_NAME",
            MarketBoothsOrderByField::Random => "MARKET_BOOTHS_ORDER_BY_FIELD_RANDOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MARKET_BOOTHS_ORDER_BY_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "MARKET_BOOTHS_ORDER_BY_FIELD_CREATED_AT" => Some(Self::CreatedAt),
            "MARKET_BOOTHS_ORDER_BY_FIELD_UPDATED_AT" => Some(Self::UpdatedAt),
            "MARKET_BOOTHS_ORDER_BY_FIELD_NAME" => Some(Self::Name),
            "MARKET_BOOTHS_ORDER_BY_FIELD_RANDOM" => Some(Self::Random),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MarketBoothsFilterField {
    Unspecified = 0,
    Name = 1,
    Description = 2,
    NameAndDescription = 3,
}
impl MarketBoothsFilterField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MarketBoothsFilterField::Unspecified => {
                "MARKET_BOOTHS_FILTER_FIELD_UNSPECIFIED"
            }
            MarketBoothsFilterField::Name => "MARKET_BOOTHS_FILTER_FIELD_NAME",
            MarketBoothsFilterField::Description => {
                "MARKET_BOOTHS_FILTER_FIELD_DESCRIPTION"
            }
            MarketBoothsFilterField::NameAndDescription => {
                "MARKET_BOOTHS_FILTER_FIELD_NAME_AND_DESCRIPTION"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MARKET_BOOTHS_FILTER_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "MARKET_BOOTHS_FILTER_FIELD_NAME" => Some(Self::Name),
            "MARKET_BOOTHS_FILTER_FIELD_DESCRIPTION" => Some(Self::Description),
            "MARKET_BOOTHS_FILTER_FIELD_NAME_AND_DESCRIPTION" => {
                Some(Self::NameAndDescription)
            }
            _ => None,
        }
    }
}
/// Generated server implementations.
pub mod market_booth_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MarketBoothServiceServer.
    #[async_trait]
    pub trait MarketBoothService: Send + Sync + 'static {
        async fn create_market_booth(
            &self,
            request: tonic::Request<super::CreateMarketBoothRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateMarketBoothResponse>,
            tonic::Status,
        >;
        async fn get_market_booth(
            &self,
            request: tonic::Request<super::GetMarketBoothRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMarketBoothResponse>,
            tonic::Status,
        >;
        async fn get_shop_by_slug(
            &self,
            request: tonic::Request<super::GetShopBySlugRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetShopBySlugResponse>,
            tonic::Status,
        >;
        async fn list_shops(
            &self,
            request: tonic::Request<super::ListShopsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListShopsResponse>,
            tonic::Status,
        >;
        async fn update_market_booth(
            &self,
            request: tonic::Request<super::UpdateMarketBoothRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateMarketBoothResponse>,
            tonic::Status,
        >;
        async fn delete_market_booth(
            &self,
            request: tonic::Request<super::DeleteMarketBoothRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteMarketBoothResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct MarketBoothServiceServer<T: MarketBoothService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: MarketBoothService> MarketBoothServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MarketBoothServiceServer<T>
    where
        T: MarketBoothService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/peoplesmarkets.commerce.v1.MarketBoothService/CreateMarketBooth" => {
                    #[allow(non_camel_case_types)]
                    struct CreateMarketBoothSvc<T: MarketBoothService>(pub Arc<T>);
                    impl<
                        T: MarketBoothService,
                    > tonic::server::UnaryService<super::CreateMarketBoothRequest>
                    for CreateMarketBoothSvc<T> {
                        type Response = super::CreateMarketBoothResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateMarketBoothRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_market_booth(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateMarketBoothSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.MarketBoothService/GetMarketBooth" => {
                    #[allow(non_camel_case_types)]
                    struct GetMarketBoothSvc<T: MarketBoothService>(pub Arc<T>);
                    impl<
                        T: MarketBoothService,
                    > tonic::server::UnaryService<super::GetMarketBoothRequest>
                    for GetMarketBoothSvc<T> {
                        type Response = super::GetMarketBoothResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMarketBoothRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_market_booth(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMarketBoothSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.MarketBoothService/GetShopBySlug" => {
                    #[allow(non_camel_case_types)]
                    struct GetShopBySlugSvc<T: MarketBoothService>(pub Arc<T>);
                    impl<
                        T: MarketBoothService,
                    > tonic::server::UnaryService<super::GetShopBySlugRequest>
                    for GetShopBySlugSvc<T> {
                        type Response = super::GetShopBySlugResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetShopBySlugRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_shop_by_slug(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetShopBySlugSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.MarketBoothService/ListShops" => {
                    #[allow(non_camel_case_types)]
                    struct ListShopsSvc<T: MarketBoothService>(pub Arc<T>);
                    impl<
                        T: MarketBoothService,
                    > tonic::server::UnaryService<super::ListShopsRequest>
                    for ListShopsSvc<T> {
                        type Response = super::ListShopsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListShopsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_shops(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListShopsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.MarketBoothService/UpdateMarketBooth" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateMarketBoothSvc<T: MarketBoothService>(pub Arc<T>);
                    impl<
                        T: MarketBoothService,
                    > tonic::server::UnaryService<super::UpdateMarketBoothRequest>
                    for UpdateMarketBoothSvc<T> {
                        type Response = super::UpdateMarketBoothResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateMarketBoothRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_market_booth(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateMarketBoothSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.MarketBoothService/DeleteMarketBooth" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteMarketBoothSvc<T: MarketBoothService>(pub Arc<T>);
                    impl<
                        T: MarketBoothService,
                    > tonic::server::UnaryService<super::DeleteMarketBoothRequest>
                    for DeleteMarketBoothSvc<T> {
                        type Response = super::DeleteMarketBoothResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteMarketBoothRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_market_booth(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteMarketBoothSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
    impl<T: MarketBoothService> Clone for MarketBoothServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: MarketBoothService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MarketBoothService> tonic::server::NamedService
    for MarketBoothServiceServer<T> {
        const NAME: &'static str = "peoplesmarkets.commerce.v1.MarketBoothService";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Recurring {
    #[prost(enumeration = "RecurringInterval", tag = "1")]
    pub interval: i32,
    #[prost(uint32, tag = "2")]
    pub interval_count: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Price {
    #[prost(enumeration = "Currency", tag = "1")]
    pub currency: i32,
    #[prost(enumeration = "PriceType", tag = "2")]
    pub price_type: i32,
    #[prost(enumeration = "PriceBillingScheme", tag = "3")]
    pub billing_scheme: i32,
    #[prost(uint32, tag = "4")]
    pub unit_amount: u32,
    #[prost(message, optional, tag = "5")]
    pub recurring: ::core::option::Option<Recurring>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Currency {
    Unspecified = 0,
    Eur = 1,
}
impl Currency {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Currency::Unspecified => "CURRENCY_UNSPECIFIED",
            Currency::Eur => "CURRENCY_EUR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CURRENCY_UNSPECIFIED" => Some(Self::Unspecified),
            "CURRENCY_EUR" => Some(Self::Eur),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceType {
    Unspecified = 0,
    OneTime = 1,
    Recurring = 2,
}
impl PriceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceType::Unspecified => "PRICE_TYPE_UNSPECIFIED",
            PriceType::OneTime => "PRICE_TYPE_ONE_TIME",
            PriceType::Recurring => "PRICE_TYPE_RECURRING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRICE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PRICE_TYPE_ONE_TIME" => Some(Self::OneTime),
            "PRICE_TYPE_RECURRING" => Some(Self::Recurring),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceBillingScheme {
    Unspecified = 0,
    PerUnit = 1,
}
impl PriceBillingScheme {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceBillingScheme::Unspecified => "PRICE_BILLING_SCHEME_UNSPECIFIED",
            PriceBillingScheme::PerUnit => "PRICE_BILLING_SCHEME_PER_UNIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRICE_BILLING_SCHEME_UNSPECIFIED" => Some(Self::Unspecified),
            "PRICE_BILLING_SCHEME_PER_UNIT" => Some(Self::PerUnit),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RecurringInterval {
    Unspecified = 0,
    Day = 1,
    Week = 2,
    Month = 3,
    Year = 4,
}
impl RecurringInterval {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RecurringInterval::Unspecified => "RECURRING_INTERVAL_UNSPECIFIED",
            RecurringInterval::Day => "RECURRING_INTERVAL_DAY",
            RecurringInterval::Week => "RECURRING_INTERVAL_WEEK",
            RecurringInterval::Month => "RECURRING_INTERVAL_MONTH",
            RecurringInterval::Year => "RECURRING_INTERVAL_YEAR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RECURRING_INTERVAL_UNSPECIFIED" => Some(Self::Unspecified),
            "RECURRING_INTERVAL_DAY" => Some(Self::Day),
            "RECURRING_INTERVAL_WEEK" => Some(Self::Week),
            "RECURRING_INTERVAL_MONTH" => Some(Self::Month),
            "RECURRING_INTERVAL_YEAR" => Some(Self::Year),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferResponse {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub market_booth_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub market_booth_name: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub created_at: i64,
    #[prost(int64, tag = "6")]
    pub updated_at: i64,
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub description: ::prost::alloc::string::String,
    #[prost(bool, tag = "9")]
    pub is_active: bool,
    #[prost(message, repeated, tag = "10")]
    pub images: ::prost::alloc::vec::Vec<OfferImageResponse>,
    #[prost(message, optional, tag = "11")]
    pub price: ::core::option::Option<Price>,
    #[prost(enumeration = "OfferType", tag = "12")]
    pub r#type: i32,
    #[prost(bool, tag = "13")]
    pub is_featured: bool,
    #[prost(string, tag = "14")]
    pub shop_slug: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OfferImageResponse {
    #[prost(string, tag = "1")]
    pub offer_image_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub image_url: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub ordering: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOfferRequest {
    #[prost(string, tag = "1")]
    pub market_booth_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "OfferType", tag = "4")]
    pub r#type: i32,
    #[prost(bool, tag = "5")]
    pub is_featured: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateOfferResponse {
    #[prost(message, optional, tag = "1")]
    pub offer: ::core::option::Option<OfferResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOfferResponse {
    #[prost(message, optional, tag = "1")]
    pub offer: ::core::option::Option<OfferResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OffersOrderBy {
    #[prost(enumeration = "OffersOrderByField", tag = "1")]
    pub field: i32,
    #[prost(enumeration = "super::super::ordering::v1::Direction", tag = "2")]
    pub direction: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OffersFilter {
    #[prost(enumeration = "OffersFilterField", tag = "1")]
    pub field: i32,
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOffersRequest {
    #[prost(string, optional, tag = "1")]
    pub user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub market_booth_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
    #[prost(message, optional, tag = "4")]
    pub order_by: ::core::option::Option<OffersOrderBy>,
    #[prost(message, optional, tag = "5")]
    pub filter: ::core::option::Option<OffersFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListOffersResponse {
    #[prost(message, repeated, tag = "1")]
    pub offers: ::prost::alloc::vec::Vec<OfferResponse>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::pagination::v1::Pagination>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "4")]
    pub is_active: ::core::option::Option<bool>,
    #[prost(enumeration = "OfferType", optional, tag = "5")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "6")]
    pub is_featured: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateOfferResponse {
    #[prost(message, optional, tag = "1")]
    pub offer: ::core::option::Option<OfferResponse>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteOfferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddImageToOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub image: ::core::option::Option<super::super::media::v1::MediaUpload>,
    #[prost(int64, tag = "3")]
    pub ordering: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddImageToOfferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveImageFromOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_image_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveImageFromOfferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutPriceToOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<Price>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PutPriceToOfferResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePriceFromOfferRequest {
    #[prost(string, tag = "1")]
    pub offer_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemovePriceFromOfferResponse {}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OfferType {
    Unspecified = 0,
    Physical = 1,
    Digital = 2,
    Service = 3,
}
impl OfferType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OfferType::Unspecified => "OFFER_TYPE_UNSPECIFIED",
            OfferType::Physical => "OFFER_TYPE_PHYSICAL",
            OfferType::Digital => "OFFER_TYPE_DIGITAL",
            OfferType::Service => "OFFER_TYPE_SERVICE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OFFER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "OFFER_TYPE_PHYSICAL" => Some(Self::Physical),
            "OFFER_TYPE_DIGITAL" => Some(Self::Digital),
            "OFFER_TYPE_SERVICE" => Some(Self::Service),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OffersOrderByField {
    Unspecified = 0,
    CreatedAt = 1,
    UpdatedAt = 2,
    Name = 3,
    Random = 4,
}
impl OffersOrderByField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OffersOrderByField::Unspecified => "OFFERS_ORDER_BY_FIELD_UNSPECIFIED",
            OffersOrderByField::CreatedAt => "OFFERS_ORDER_BY_FIELD_CREATED_AT",
            OffersOrderByField::UpdatedAt => "OFFERS_ORDER_BY_FIELD_UPDATED_AT",
            OffersOrderByField::Name => "OFFERS_ORDER_BY_FIELD_NAME",
            OffersOrderByField::Random => "OFFERS_ORDER_BY_FIELD_RANDOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OFFERS_ORDER_BY_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "OFFERS_ORDER_BY_FIELD_CREATED_AT" => Some(Self::CreatedAt),
            "OFFERS_ORDER_BY_FIELD_UPDATED_AT" => Some(Self::UpdatedAt),
            "OFFERS_ORDER_BY_FIELD_NAME" => Some(Self::Name),
            "OFFERS_ORDER_BY_FIELD_RANDOM" => Some(Self::Random),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OffersFilterField {
    Unspecified = 0,
    Name = 1,
    Description = 2,
    NameAndDescription = 3,
    Type = 4,
    IsFeatured = 5,
}
impl OffersFilterField {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OffersFilterField::Unspecified => "OFFERS_FILTER_FIELD_UNSPECIFIED",
            OffersFilterField::Name => "OFFERS_FILTER_FIELD_NAME",
            OffersFilterField::Description => "OFFERS_FILTER_FIELD_DESCRIPTION",
            OffersFilterField::NameAndDescription => {
                "OFFERS_FILTER_FIELD_NAME_AND_DESCRIPTION"
            }
            OffersFilterField::Type => "OFFERS_FILTER_FIELD_TYPE",
            OffersFilterField::IsFeatured => "OFFERS_FILTER_FIELD_IS_FEATURED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OFFERS_FILTER_FIELD_UNSPECIFIED" => Some(Self::Unspecified),
            "OFFERS_FILTER_FIELD_NAME" => Some(Self::Name),
            "OFFERS_FILTER_FIELD_DESCRIPTION" => Some(Self::Description),
            "OFFERS_FILTER_FIELD_NAME_AND_DESCRIPTION" => Some(Self::NameAndDescription),
            "OFFERS_FILTER_FIELD_TYPE" => Some(Self::Type),
            "OFFERS_FILTER_FIELD_IS_FEATURED" => Some(Self::IsFeatured),
            _ => None,
        }
    }
}
/// Generated server implementations.
pub mod offer_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with OfferServiceServer.
    #[async_trait]
    pub trait OfferService: Send + Sync + 'static {
        async fn create_offer(
            &self,
            request: tonic::Request<super::CreateOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateOfferResponse>,
            tonic::Status,
        >;
        async fn get_offer(
            &self,
            request: tonic::Request<super::GetOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOfferResponse>,
            tonic::Status,
        >;
        async fn list_offers(
            &self,
            request: tonic::Request<super::ListOffersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListOffersResponse>,
            tonic::Status,
        >;
        async fn update_offer(
            &self,
            request: tonic::Request<super::UpdateOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateOfferResponse>,
            tonic::Status,
        >;
        async fn delete_offer(
            &self,
            request: tonic::Request<super::DeleteOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteOfferResponse>,
            tonic::Status,
        >;
        async fn add_image_to_offer(
            &self,
            request: tonic::Request<super::AddImageToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddImageToOfferResponse>,
            tonic::Status,
        >;
        async fn remove_image_from_offer(
            &self,
            request: tonic::Request<super::RemoveImageFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveImageFromOfferResponse>,
            tonic::Status,
        >;
        async fn put_price_to_offer(
            &self,
            request: tonic::Request<super::PutPriceToOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PutPriceToOfferResponse>,
            tonic::Status,
        >;
        async fn remove_price_from_offer(
            &self,
            request: tonic::Request<super::RemovePriceFromOfferRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemovePriceFromOfferResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct OfferServiceServer<T: OfferService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: OfferService> OfferServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
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
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OfferServiceServer<T>
    where
        T: OfferService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/peoplesmarkets.commerce.v1.OfferService/CreateOffer" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOfferSvc<T: OfferService>(pub Arc<T>);
                    impl<
                        T: OfferService,
                    > tonic::server::UnaryService<super::CreateOfferRequest>
                    for CreateOfferSvc<T> {
                        type Response = super::CreateOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).create_offer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.OfferService/GetOffer" => {
                    #[allow(non_camel_case_types)]
                    struct GetOfferSvc<T: OfferService>(pub Arc<T>);
                    impl<
                        T: OfferService,
                    > tonic::server::UnaryService<super::GetOfferRequest>
                    for GetOfferSvc<T> {
                        type Response = super::GetOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).get_offer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.OfferService/ListOffers" => {
                    #[allow(non_camel_case_types)]
                    struct ListOffersSvc<T: OfferService>(pub Arc<T>);
                    impl<
                        T: OfferService,
                    > tonic::server::UnaryService<super::ListOffersRequest>
                    for ListOffersSvc<T> {
                        type Response = super::ListOffersResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListOffersRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).list_offers(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListOffersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.OfferService/UpdateOffer" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateOfferSvc<T: OfferService>(pub Arc<T>);
                    impl<
                        T: OfferService,
                    > tonic::server::UnaryService<super::UpdateOfferRequest>
                    for UpdateOfferSvc<T> {
                        type Response = super::UpdateOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).update_offer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.OfferService/DeleteOffer" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOfferSvc<T: OfferService>(pub Arc<T>);
                    impl<
                        T: OfferService,
                    > tonic::server::UnaryService<super::DeleteOfferRequest>
                    for DeleteOfferSvc<T> {
                        type Response = super::DeleteOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).delete_offer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.OfferService/AddImageToOffer" => {
                    #[allow(non_camel_case_types)]
                    struct AddImageToOfferSvc<T: OfferService>(pub Arc<T>);
                    impl<
                        T: OfferService,
                    > tonic::server::UnaryService<super::AddImageToOfferRequest>
                    for AddImageToOfferSvc<T> {
                        type Response = super::AddImageToOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddImageToOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).add_image_to_offer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddImageToOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.OfferService/RemoveImageFromOffer" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveImageFromOfferSvc<T: OfferService>(pub Arc<T>);
                    impl<
                        T: OfferService,
                    > tonic::server::UnaryService<super::RemoveImageFromOfferRequest>
                    for RemoveImageFromOfferSvc<T> {
                        type Response = super::RemoveImageFromOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveImageFromOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_image_from_offer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveImageFromOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.OfferService/PutPriceToOffer" => {
                    #[allow(non_camel_case_types)]
                    struct PutPriceToOfferSvc<T: OfferService>(pub Arc<T>);
                    impl<
                        T: OfferService,
                    > tonic::server::UnaryService<super::PutPriceToOfferRequest>
                    for PutPriceToOfferSvc<T> {
                        type Response = super::PutPriceToOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PutPriceToOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).put_price_to_offer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PutPriceToOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/peoplesmarkets.commerce.v1.OfferService/RemovePriceFromOffer" => {
                    #[allow(non_camel_case_types)]
                    struct RemovePriceFromOfferSvc<T: OfferService>(pub Arc<T>);
                    impl<
                        T: OfferService,
                    > tonic::server::UnaryService<super::RemovePriceFromOfferRequest>
                    for RemovePriceFromOfferSvc<T> {
                        type Response = super::RemovePriceFromOfferResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemovePriceFromOfferRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).remove_price_from_offer(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemovePriceFromOfferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
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
    impl<T: OfferService> Clone for OfferServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: OfferService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: OfferService> tonic::server::NamedService for OfferServiceServer<T> {
        const NAME: &'static str = "peoplesmarkets.commerce.v1.OfferService";
    }
}
