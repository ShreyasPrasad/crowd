use std::{collections::HashSet, time::Instant};

use uuid::Uuid;

/**
 * Similar to Cloudflare durable objects, we wish to setup a point of coordination among multiple clients registed with
 * potentially independent PoPs. To do so, we rely on the PoP API to discover and extract groups of related clients.
 * 
 * Logic dicating how these groups are formed can be declaratively constructed. See the builder module.
 */

mod builder;

pub struct OpaqueClient {
    uuid: Uuid
}

pub struct CoordinatorGroup {
    create_time: Instant,
    clients: HashSet<OpaqueClient>
}

/// Defines the interface that a concrete ClientCoordinator must implement. The concrete implementa
pub trait ClientCoordinator {
    /// Builds a CoordinatorGroup, consisting of one or more OpaqueClients.
    async fn coordinate(&self) -> CoordinatorGroup;
}