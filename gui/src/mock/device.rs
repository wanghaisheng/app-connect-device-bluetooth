use std::time::Duration;

use async_trait::async_trait;
use gtk::glib::timeout_future;
use macaddr::MacAddr6;
use mockall::mock;
use openscq30_lib::{
    api::{connection::ConnectionStatus, device::Device},
    packets::structures::{EqualizerConfiguration, SoundModes},
    state::DeviceState,
};
use tokio::sync::{broadcast, watch};

mock! {
    #[derive(Debug)]
    pub Device {
        pub fn subscribe_to_state_updates(&self) -> broadcast::Receiver<DeviceState>;
        pub fn connection_status(&self) -> watch::Receiver<ConnectionStatus>;
        pub fn mac_address(&self) -> openscq30_lib::Result<MacAddr6>;
        pub fn name(&self) -> openscq30_lib::Result<String>;
        pub fn state(&self) -> DeviceState;
        pub fn set_sound_modes(
            &self,
            sound_modes: SoundModes,
        ) -> openscq30_lib::Result<()>;
        pub fn set_equalizer_configuration(
            &self,
            configuration: EqualizerConfiguration,
        ) -> openscq30_lib::Result<()>;
    }
}

#[async_trait]
impl Device for MockDevice {
    fn subscribe_to_state_updates(&self) -> broadcast::Receiver<DeviceState> {
        self.subscribe_to_state_updates()
    }
    fn connection_status(&self) -> watch::Receiver<ConnectionStatus> {
        self.connection_status()
    }
    async fn mac_address(&self) -> openscq30_lib::Result<MacAddr6> {
        timeout_future(Duration::from_millis(10)).await;
        self.mac_address()
    }
    async fn name(&self) -> openscq30_lib::Result<String> {
        timeout_future(Duration::from_millis(10)).await;
        self.name()
    }
    async fn state(&self) -> DeviceState {
        timeout_future(Duration::from_millis(10)).await;
        self.state()
    }
    async fn set_sound_modes(&self, sound_modes: SoundModes) -> openscq30_lib::Result<()> {
        timeout_future(Duration::from_millis(10)).await;
        self.set_sound_modes(sound_modes)
    }
    async fn set_equalizer_configuration(
        &self,
        equalizer_configuration: EqualizerConfiguration,
    ) -> openscq30_lib::Result<()> {
        timeout_future(Duration::from_millis(10)).await;
        self.set_equalizer_configuration(equalizer_configuration)
    }
}
