use openscq30_lib::api::device::DeviceRegistry;

use crate::{
    objects::EqualizerCustomProfileObject,
    settings::{Config, SettingsFile},
};

use super::{State, StateUpdate};

pub fn delete_custom_equalizer_profile<T>(
    state: &State<T>,
    settings_file: &SettingsFile<Config>,
    custom_profile: &EqualizerCustomProfileObject,
) -> anyhow::Result<()>
where
    T: DeviceRegistry + Send + Sync + 'static,
{
    settings_file.edit(|settings| {
        settings.remove_custom_profile(&custom_profile.name());
    })?;
    settings_file.get(|settings| {
        state
            .state_update_sender
            .send(StateUpdate::SetEqualizerCustomProfiles(
                settings
                    .custom_profiles()
                    .iter()
                    .map(|(name, profile)| {
                        EqualizerCustomProfileObject::new(name, profile.volume_offsets())
                    })
                    .collect(),
            ))
            .unwrap();
    })?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        actions::{State, StateUpdate},
        mock::MockDeviceRegistry,
        objects::EqualizerCustomProfileObject,
        settings::{Config, EqualizerCustomProfile, SettingsFile},
    };

    use super::delete_custom_equalizer_profile;

    #[gtk::test]
    async fn it_works() {
        crate::load_resources();
        let registry = MockDeviceRegistry::new();
        let (state, mut receiver) = State::new(registry);

        let file = tempfile::NamedTempFile::new().unwrap();
        let settings_file = SettingsFile::<Config>::new(file.path().to_path_buf());
        let custom_profile = EqualizerCustomProfileObject::new(
            &"custom profile".to_string(),
            [1, 2, 3, 4, 5, 6, 7, 8],
        );
        settings_file
            .edit(|settings| {
                settings.set_custom_profile(
                    custom_profile.name(),
                    EqualizerCustomProfile::new(custom_profile.volume_offsets()),
                );
            })
            .unwrap();
        delete_custom_equalizer_profile(&state, &settings_file, &custom_profile).unwrap();

        let state_update = receiver.recv().await.unwrap();
        assert_eq!(
            StateUpdate::SetEqualizerCustomProfiles(Vec::new()),
            state_update,
        );
        assert_eq!(
            0,
            settings_file
                .get(|settings| settings.custom_profiles().len())
                .unwrap()
        );
    }
}
