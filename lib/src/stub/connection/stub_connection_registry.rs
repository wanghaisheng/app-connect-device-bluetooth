use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use async_trait::async_trait;
use macaddr::MacAddr6;

use crate::api::connection::{ConnectionDescriptor, ConnectionRegistry};

use super::{StubConnection, StubConnectionDescriptor};

#[derive(Debug)]
pub struct StubConnectionRegistry {
    connections:
        HashMap<StubConnectionDescriptor, Arc<<Self as ConnectionRegistry>::ConnectionType>>,
}

impl StubConnectionRegistry {
    pub fn new(
        connections: HashMap<
            StubConnectionDescriptor,
            Arc<<Self as ConnectionRegistry>::ConnectionType>,
        >,
    ) -> Self {
        Self { connections }
    }
}

#[async_trait]
impl ConnectionRegistry for StubConnectionRegistry {
    type ConnectionType = StubConnection;
    type DescriptorType = StubConnectionDescriptor;

    async fn connection_descriptors(&self) -> crate::Result<HashSet<Self::DescriptorType>> {
        Ok(self.connections.keys().cloned().collect())
    }

    async fn connection(
        &self,
        mac_address: MacAddr6,
    ) -> crate::Result<Option<Arc<Self::ConnectionType>>> {
        Ok(self
            .connections
            .iter()
            .find(|(descriptor, _connection)| descriptor.mac_address() == mac_address)
            .map(|(_descriptor, connection)| connection)
            .cloned())
    }
}
