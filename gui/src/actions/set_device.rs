use std::rc::Rc;

use anyhow::anyhow;
use gtk::glib::{clone, MainContext};
use openscq30_lib::api::{
    connection::ConnectionStatus,
    device::{Device, DeviceRegistry},
};
use tokio::sync::oneshot;

use crate::objects::DeviceObject;

use super::{State, StateUpdate};

pub async fn set_device<T>(
    state: &Rc<State<T>>,
    new_selected_device: Option<DeviceObject>,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + Send + Sync + 'static,
{
    // Clean up any existing devices
    if let Some(handle) = &*state.connect_to_device_handle.borrow_mut() {
        handle.abort();
    }
    *state.selected_device.borrow_mut() = None;

    // Connect to new device
    if let Some(new_selected_device) = new_selected_device {
        state
            .state_update_sender
            .send(StateUpdate::SetLoading(true))
            .unwrap();
        let main_context = MainContext::default();
        let (sender, receiver) = oneshot::channel();
        let handle = main_context.spawn_local(clone!(@strong state => async move {
            let result: anyhow::Result<()> = (async {
                let mac_address = new_selected_device.mac_address();
                let device = state.registry.device(&mac_address).await?.ok_or_else(|| {
                    state.state_update_sender
                        .send(StateUpdate::SetSelectedDevice(None))
                        .map_err(|err| anyhow!("{err}")) // StateUpdate isn't send
                        .err()
                        .unwrap_or_else(|| anyhow!("device not found: {mac_address}"))
                })?;

                *state.selected_device.borrow_mut() = Some(device.to_owned());
                let receiver = device.subscribe_to_state_updates();
                state.state_update_receiver.replace_receiver(Some(receiver)).await;

                let mut connection_status_receiver = device.connection_status();
                let state_update_sender = state.state_update_sender.clone();
                MainContext::default().spawn_local(async move {
                    loop {
                        match connection_status_receiver.changed().await {
                            Ok(_) => {
                                let connection_status = *connection_status_receiver.borrow();
                                if connection_status == ConnectionStatus::Disconnected {
                                    if let Err(err) = state_update_sender.send(StateUpdate::AddToast("Device Disconnected".to_string())) {
                                        tracing::error!(
                                            "error sending toast: {err:?}",
                                        );
                                    }
                                    if let Err(err) = state_update_sender.send(StateUpdate::SetSelectedDevice(None)) {
                                        tracing::error!(
                                            "setting device state to disconnected after receiving ConnectionStatus::Disconnected: {err:?}",
                                        );
                                    }
                                    break
                                }
                            },
                            Err(err) => {
                                tracing::debug!("connection status sender destroyed, exiting loop: {err:?}");
                                break
                            },
                        };
                    }
                });

                state.state_update_sender
                    .send(StateUpdate::SetAmbientSoundMode(device.ambient_sound_mode().await))
                    .map_err(|err| anyhow!("{err}"))?;
                state.state_update_sender
                    .send(StateUpdate::SetNoiseCancelingMode(device.noise_canceling_mode().await))
                    .map_err(|err| anyhow!("{err}"))?;
                state.state_update_sender
                    .send(StateUpdate::SetEqualizerConfiguration(device.equalizer_configuration().await))
                    .map_err(|err| anyhow!("{err}"))?;

                Ok(())
            }).await;
            state.state_update_sender.send(StateUpdate::SetLoading(false)).unwrap();
            sender.send(result).unwrap();
        }));
        *state.connect_to_device_handle.borrow_mut() = Some(handle);
        // we don't care if the receive fails, since that just means that someone else set a new device and canceled us
        if let Ok(result) = receiver.await {
            return result;
        }
    } else {
        state
            .state_update_sender
            .send(StateUpdate::SetLoading(false))
            .map_err(|err| anyhow::anyhow!("{err}"))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{collections::VecDeque, sync::Arc};

    use mockall::predicate;
    use openscq30_lib::{
        api::connection::ConnectionStatus,
        packets::structures::{
            AmbientSoundMode, EqualizerConfiguration, NoiseCancelingMode, PresetEqualizerProfile,
        },
    };
    use tokio::sync::{broadcast, watch};

    use crate::{
        actions::{State, StateUpdate},
        mock::{MockDevice, MockDeviceRegistry},
        objects::DeviceObject,
    };

    use super::set_device;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let (_sender, receiver) = watch::channel(ConnectionStatus::Connected);
        let mut registry = MockDeviceRegistry::new();
        registry
            .expect_device()
            .with(predicate::eq("00:00:00:00:00:00"))
            .return_once(|_mac_address| {
                let mut device = MockDevice::new();
                device
                    .expect_subscribe_to_state_updates()
                    .once()
                    .return_once(|| {
                        let (_sender, receiver) = broadcast::channel(10);
                        receiver
                    });
                device
                    .expect_connection_status()
                    .once()
                    .return_const(receiver);
                device
                    .expect_ambient_sound_mode()
                    .once()
                    .return_const(AmbientSoundMode::Transparency);
                device
                    .expect_noise_canceling_mode()
                    .once()
                    .return_const(NoiseCancelingMode::Indoor);
                device.expect_equalizer_configuration().once().return_const(
                    EqualizerConfiguration::new_from_preset_profile(
                        PresetEqualizerProfile::Acoustic,
                    ),
                );

                Ok(Some(Arc::new(device)))
            });

        let (state, mut receiver) = State::new(registry);

        let new_selected_device =
            DeviceObject::new(&"Name".to_string(), &"00:00:00:00:00:00".to_string());
        set_device(&state, Some(new_selected_device)).await.unwrap();
        let mut expected_sequence = VecDeque::from([
            StateUpdate::SetLoading(true),
            StateUpdate::SetAmbientSoundMode(AmbientSoundMode::Transparency),
            StateUpdate::SetNoiseCancelingMode(NoiseCancelingMode::Indoor),
            StateUpdate::SetEqualizerConfiguration(
                EqualizerConfiguration::new_from_preset_profile(PresetEqualizerProfile::Acoustic),
            ),
            StateUpdate::SetLoading(false),
        ]);
        loop {
            if let Some(state_update) = receiver.recv().await {
                let expected = expected_sequence.pop_front().unwrap();
                assert_eq!(expected, state_update);

                if expected_sequence.is_empty() {
                    break;
                }
            }
        }
    }

    #[gtk::test]
    async fn test_sets_device_to_none_when_disconnected_is_received() {
        crate::load_resources();
        let (sender, receiver) = watch::channel(ConnectionStatus::Connected);
        let mut registry = MockDeviceRegistry::new();
        registry
            .expect_device()
            .with(predicate::eq("00:00:00:00:00:00"))
            .return_once(|_mac_address| {
                let mut device = MockDevice::new();
                device
                    .expect_subscribe_to_state_updates()
                    .once()
                    .return_once(|| {
                        let (_sender, receiver) = broadcast::channel(10);
                        receiver
                    });
                device
                    .expect_connection_status()
                    .once()
                    .return_const(receiver);
                device
                    .expect_ambient_sound_mode()
                    .once()
                    .return_const(AmbientSoundMode::Transparency);
                device
                    .expect_noise_canceling_mode()
                    .once()
                    .return_const(NoiseCancelingMode::Indoor);
                device.expect_equalizer_configuration().once().return_const(
                    EqualizerConfiguration::new_from_preset_profile(
                        PresetEqualizerProfile::Acoustic,
                    ),
                );

                Ok(Some(Arc::new(device)))
            });

        let (state, _receiver) = State::new(registry);

        let new_selected_device =
            DeviceObject::new(&"Name".to_string(), &"00:00:00:00:00:00".to_string());
        set_device(&state, Some(new_selected_device)).await.unwrap();
        sender.send_replace(ConnectionStatus::Disconnected);
    }
}
