use gtk::{
    glib::{self, Object, Sender},
    subclass::prelude::ObjectSubclassIsExt,
};
use openscq30_lib::state::DeviceState;

use crate::{
    actions::Action,
    objects::{CustomEqualizerProfileObject, NamedQuickPreset},
};

glib::wrapper! {
    pub struct QuickPresetsScreen(ObjectSubclass<imp::QuickPresetsScreen>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl QuickPresetsScreen {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn set_sender(&self, sender: Sender<Action>) {
        self.imp().set_sender(sender);
    }

    pub fn set_quick_presets(&self, quick_presets: Vec<NamedQuickPreset>) {
        self.imp()
            .quick_presets_listing
            .set_quick_presets(quick_presets)
    }

    pub fn set_device_state(&self, state: &DeviceState) {
        self.imp()
            .edit_quick_preset
            .set_device_feature_flags(state.feature_flags);
    }

    pub fn set_custom_profiles(&self, custom_profiles: Vec<CustomEqualizerProfileObject>) {
        self.imp()
            .edit_quick_preset
            .set_custom_equalizer_profiles(custom_profiles);
    }
}

mod imp {
    use gtk::{
        glib::{self, Sender},
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        template_callbacks, CompositeTemplate,
    };

    use crate::{
        actions::Action,
        objects::NamedQuickPreset,
        ui::widgets::quick_presets::{
            edit_quick_preset::EditQuickPreset, quick_presets_listing::QuickPresetsListing,
        },
    };

    #[derive(Default, CompositeTemplate)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/quick_presets/quick_presets_screen.ui"
    )]
    pub struct QuickPresetsScreen {
        #[template_child]
        pub navigation: TemplateChild<adw::NavigationView>,
        #[template_child]
        pub quick_presets_listing: TemplateChild<QuickPresetsListing>,
        #[template_child]
        pub edit_quick_preset: TemplateChild<EditQuickPreset>,
    }

    #[template_callbacks]
    impl QuickPresetsScreen {
        #[template_callback]
        pub fn handle_edit_quick_preset(
            &self,
            named_quick_preset: NamedQuickPreset,
            _: &QuickPresetsListing,
        ) {
            self.edit_quick_preset.set_quick_preset(named_quick_preset);
            self.navigation.push_by_tag("edit-quick-preset");
        }
    }

    impl QuickPresetsScreen {
        pub fn set_sender(&self, sender: Sender<Action>) {
            self.quick_presets_listing.set_sender(sender.clone());
            self.edit_quick_preset.set_sender(sender.clone());
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for QuickPresetsScreen {
        const NAME: &'static str = "OpenSCQ30QuickPresetsScreen";
        type Type = super::QuickPresetsScreen;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }
    impl ObjectImpl for QuickPresetsScreen {
        fn constructed(&self) {}
    }
    impl WidgetImpl for QuickPresetsScreen {}
    impl BoxImpl for QuickPresetsScreen {}
}
