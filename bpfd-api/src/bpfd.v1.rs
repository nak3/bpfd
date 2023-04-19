#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BytecodeImage {
    #[prost(string, tag = "1")]
    pub url: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub image_pull_policy: i32,
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadRequestCommon {
    #[prost(string, tag = "3")]
    pub section_name: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub program_type: i32,
    #[prost(string, optional, tag = "5")]
    pub id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(map = "string, bytes", tag = "6")]
    pub global_data: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::vec::Vec<u8>,
    >,
    #[prost(oneof = "load_request_common::Location", tags = "1, 2")]
    pub location: ::core::option::Option<load_request_common::Location>,
}
/// Nested message and enum types in `LoadRequestCommon`.
pub mod load_request_common {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Location {
        #[prost(message, tag = "1")]
        Image(super::BytecodeImage),
        #[prost(string, tag = "2")]
        File(::prost::alloc::string::String),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NoAttachInfo {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XdpAttachInfo {
    #[prost(int32, tag = "1")]
    pub priority: i32,
    #[prost(string, tag = "2")]
    pub iface: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub position: i32,
    #[prost(int32, repeated, tag = "4")]
    pub proceed_on: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TcAttachInfo {
    #[prost(int32, tag = "1")]
    pub priority: i32,
    #[prost(string, tag = "2")]
    pub iface: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub position: i32,
    #[prost(string, tag = "4")]
    pub direction: ::prost::alloc::string::String,
    #[prost(int32, repeated, tag = "5")]
    pub proceed_on: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TracepointAttachInfo {
    #[prost(string, tag = "1")]
    pub tracepoint: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadRequest {
    #[prost(message, optional, tag = "1")]
    pub common: ::core::option::Option<LoadRequestCommon>,
    #[prost(oneof = "load_request::AttachInfo", tags = "2, 3, 4")]
    pub attach_info: ::core::option::Option<load_request::AttachInfo>,
}
/// Nested message and enum types in `LoadRequest`.
pub mod load_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum AttachInfo {
        #[prost(message, tag = "2")]
        XdpAttachInfo(super::XdpAttachInfo),
        #[prost(message, tag = "3")]
        TcAttachInfo(super::TcAttachInfo),
        #[prost(message, tag = "4")]
        TracepointAttachInfo(super::TracepointAttachInfo),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadResponse {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnloadRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnloadResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRequest {
    #[prost(int32, optional, tag = "1")]
    pub program_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResponse {
    #[prost(message, repeated, tag = "10")]
    pub results: ::prost::alloc::vec::Vec<list_response::ListResult>,
}
/// Nested message and enum types in `ListResponse`.
pub mod list_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ListResult {
        #[prost(string, tag = "1")]
        pub id: ::prost::alloc::string::String,
        #[prost(string, optional, tag = "2")]
        pub section_name: ::core::option::Option<::prost::alloc::string::String>,
        #[prost(int32, tag = "5")]
        pub program_type: i32,
        #[prost(oneof = "list_result::Location", tags = "3, 4")]
        pub location: ::core::option::Option<list_result::Location>,
        #[prost(oneof = "list_result::AttachInfo", tags = "6, 7, 8, 9")]
        pub attach_info: ::core::option::Option<list_result::AttachInfo>,
    }
    /// Nested message and enum types in `ListResult`.
    pub mod list_result {
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Location {
            #[prost(message, tag = "3")]
            Image(super::super::BytecodeImage),
            #[prost(string, tag = "4")]
            File(::prost::alloc::string::String),
        }
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum AttachInfo {
            #[prost(message, tag = "6")]
            None(super::super::NoAttachInfo),
            #[prost(message, tag = "7")]
            XdpAttachInfo(super::super::XdpAttachInfo),
            #[prost(message, tag = "8")]
            TcAttachInfo(super::super::TcAttachInfo),
            #[prost(message, tag = "9")]
            TracepointAttachInfo(super::super::TracepointAttachInfo),
        }
    }
}
/// Generated client implementations.
pub mod loader_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct LoaderClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl LoaderClient<tonic::transport::Channel> {
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
    impl<T> LoaderClient<T>
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
        ) -> LoaderClient<InterceptedService<T, F>>
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
            LoaderClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn load(
            &mut self,
            request: impl tonic::IntoRequest<super::LoadRequest>,
        ) -> Result<tonic::Response<super::LoadResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/bpfd.v1.Loader/Load");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn unload(
            &mut self,
            request: impl tonic::IntoRequest<super::UnloadRequest>,
        ) -> Result<tonic::Response<super::UnloadResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/bpfd.v1.Loader/Unload");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRequest>,
        ) -> Result<tonic::Response<super::ListResponse>, tonic::Status> {
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
            let path = http::uri::PathAndQuery::from_static("/bpfd.v1.Loader/List");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod loader_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with LoaderServer.
    #[async_trait]
    pub trait Loader: Send + Sync + 'static {
        async fn load(
            &self,
            request: tonic::Request<super::LoadRequest>,
        ) -> Result<tonic::Response<super::LoadResponse>, tonic::Status>;
        async fn unload(
            &self,
            request: tonic::Request<super::UnloadRequest>,
        ) -> Result<tonic::Response<super::UnloadResponse>, tonic::Status>;
        async fn list(
            &self,
            request: tonic::Request<super::ListRequest>,
        ) -> Result<tonic::Response<super::ListResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct LoaderServer<T: Loader> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Loader> LoaderServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for LoaderServer<T>
    where
        T: Loader,
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
                "/bpfd.v1.Loader/Load" => {
                    #[allow(non_camel_case_types)]
                    struct LoadSvc<T: Loader>(pub Arc<T>);
                    impl<T: Loader> tonic::server::UnaryService<super::LoadRequest>
                    for LoadSvc<T> {
                        type Response = super::LoadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LoadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).load(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LoadSvc(inner);
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
                "/bpfd.v1.Loader/Unload" => {
                    #[allow(non_camel_case_types)]
                    struct UnloadSvc<T: Loader>(pub Arc<T>);
                    impl<T: Loader> tonic::server::UnaryService<super::UnloadRequest>
                    for UnloadSvc<T> {
                        type Response = super::UnloadResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnloadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).unload(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UnloadSvc(inner);
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
                "/bpfd.v1.Loader/List" => {
                    #[allow(non_camel_case_types)]
                    struct ListSvc<T: Loader>(pub Arc<T>);
                    impl<T: Loader> tonic::server::UnaryService<super::ListRequest>
                    for ListSvc<T> {
                        type Response = super::ListResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSvc(inner);
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
    impl<T: Loader> Clone for LoaderServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Loader> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Loader> tonic::server::NamedService for LoaderServer<T> {
        const NAME: &'static str = "bpfd.v1.Loader";
    }
}
