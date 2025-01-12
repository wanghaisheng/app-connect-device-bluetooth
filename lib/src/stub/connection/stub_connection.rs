use std::collections::VecDeque;

use macaddr::MacAddr6;
use tokio::sync::{mpsc, watch, Mutex, RwLock};
use uuid::Uuid;

use crate::api::connection::{Connection, ConnectionStatus};

#[derive(Debug)]
pub struct StubConnection {
    name_return: RwLock<Option<crate::Result<String>>>,
    mac_address_return: RwLock<Option<crate::Result<MacAddr6>>>,
    write_return_queue: Mutex<VecDeque<crate::Result<()>>>,
    inbound_packets_channel: Mutex<Option<crate::Result<mpsc::Receiver<Vec<u8>>>>>,
}

impl StubConnection {
    pub fn new() -> Self {
        StubConnection {
            name_return: RwLock::new(None),
            mac_address_return: RwLock::new(None),
            write_return_queue: Mutex::new(VecDeque::new()),
            inbound_packets_channel: Mutex::new(None),
        }
    }

    pub async fn set_name_return(&self, name_return: crate::Result<String>) {
        let mut lock = self.name_return.write().await;
        *lock = Some(name_return);
    }

    pub async fn set_mac_address_return(&self, mac_address_return: crate::Result<MacAddr6>) {
        let mut lock = self.mac_address_return.write().await;
        *lock = Some(mac_address_return);
    }

    pub async fn push_write_return(&self, write_return: crate::Result<()>) {
        let mut lock = self.write_return_queue.lock().await;
        lock.push_back(write_return);
    }

    pub async fn write_return_queue_length(&self) -> usize {
        let lock = self.write_return_queue.lock().await;
        lock.len()
    }

    pub async fn set_inbound_packets_channel(
        &self,
        receiver: crate::Result<mpsc::Receiver<Vec<u8>>>,
    ) {
        let mut lock = self.inbound_packets_channel.lock().await;
        *lock = Some(receiver);
    }
}

impl Connection for StubConnection {
    async fn name(&self) -> crate::Result<String> {
        self.name_return.write().await.take().unwrap()
    }

    async fn mac_address(&self) -> crate::Result<MacAddr6> {
        self.mac_address_return.write().await.take().unwrap()
    }

    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        let (_sender, receiver) = watch::channel(ConnectionStatus::Connected);
        receiver
    }

    fn service_uuid(&self) -> Uuid {
        Uuid::default()
    }

    async fn write_with_response(&self, _data: &[u8]) -> crate::Result<()> {
        let mut lock = self.write_return_queue.lock().await;
        lock.pop_front().unwrap_or(Ok(()))
    }

    async fn write_without_response(&self, _data: &[u8]) -> crate::Result<()> {
        let mut lock = self.write_return_queue.lock().await;
        lock.pop_front().unwrap_or(Ok(()))
    }

    async fn inbound_packets_channel(&self) -> crate::Result<mpsc::Receiver<Vec<u8>>> {
        self.inbound_packets_channel.lock().await.take().unwrap()
    }
}

impl Default for StubConnection {
    fn default() -> Self {
        Self::new()
    }
}
