//! Lightwallet service RPC implementations.

use std::sync::{atomic::AtomicBool, Arc};

pub mod service;

#[derive(Debug, Clone)]
/// Configuration data for gRPC server.
pub struct GrpcClient {
    /// Lightwalletd uri.
    /// Used by grpc_passthrough to pass on unimplemented RPCs.
    pub lightwalletd_uri: http::Uri,
    /// Zebrad uri.
    pub zebrad_uri: http::Uri,
    /// Represents the Online status of the gRPC server.
    pub online: Arc<AtomicBool>,
}
