use futures_util::Stream;
use std::net::SocketAddr;

/**
 * A point of prescence (pop) is a frontend server that receives and registers a user's websocket connection.
 * Updates to this connection, including new messages, and eventual termination, are handled by this server and
 * forwarded to any selected coordination server.
 */

mod warp;

pub struct PointOfPrescenceOptions {
    addr: SocketAddr
}

/// Defines the interface that a concrete PointOfPrescence must implement.
pub trait PointOfPrescence {

    /// Generates a new client to handle an incoming client connection or updates. 
    fn generate_client_handler() -> impl PointOfPrescenceClientHandler;
    
    /// Initializes the Point of Prescence, allowing connections to be registered.
    /// 
    /// * `options` - Options used by the point of prescence for initialiation
    async fn init(&self, options: PointOfPrescenceOptions);
}

/// Defines the client handler generated for each new client registering with the PointOfPrescence.
pub trait PointOfPrescenceClientHandler {

    /// Generic, allowing for custom connection types to be used, all of which must represent a stream of 
    /// futures_util::Stream values
    type Connection: Stream;
    
    /// Receives and registers a client, upgrading the corresponding WebSocket connection and waiting for messages
    /// or updates. This method consumes self.
    /// 
    /// * `conn` - Represents a stream of client-provided values
    async fn receive_client(self, conn: Self::Connection);
}