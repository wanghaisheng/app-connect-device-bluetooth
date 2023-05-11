pub mod imp;

use gtk::{
    glib::{self, Object},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::packets::structures::EqualizerConfiguration;

use crate::objects::CustomEqualizerProfileObject;

glib::wrapper! {
    pub struct EqualizerSettings(ObjectSubclass<imp::EqualizerSettings>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl EqualizerSettings {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_equalizer_configuration(&self, equalizer_configuration: &EqualizerConfiguration) {
        self.imp()
            .set_equalizer_configuration(equalizer_configuration);
    }

    pub fn equalizer_configuration(&self) -> EqualizerConfiguration {
        self.imp().equalizer_configuration()
    }

    pub fn set_custom_profiles(&self, custom_profiles: Vec<CustomEqualizerProfileObject>) {
        self.imp().set_custom_profiles(custom_profiles)
    }
}

#[cfg(test)]
mod tests {
    use gtk::{subclass::prelude::*, traits::WidgetExt};

    use crate::objects::CustomEqualizerProfileObject;

    use super::EqualizerSettings;

    #[gtk::test]
    fn test_only_shows_create_button_with_no_custom_profile_selected() {
        crate::load_resources();
        let settings = EqualizerSettings::new();
        assert_eq!(
            true,
            settings.imp().create_custom_profile_button.is_visible(),
        );
        assert_eq!(
            false,
            settings.imp().delete_custom_profile_button.is_visible(),
        );
    }

    #[gtk::test]
    fn test_only_shows_delete_button_with_custom_profile_selected() {
        crate::load_resources();
        let settings = EqualizerSettings::new();
        settings.set_custom_profiles(vec![CustomEqualizerProfileObject::new(
            "test profile",
            [0, 0, 0, 0, 0, 0, 0, 0],
        )]);
        assert_eq!(
            true,
            settings.imp().create_custom_profile_button.is_visible(),
        );
        assert_eq!(
            false,
            settings.imp().delete_custom_profile_button.is_visible(),
        );
    }
}
