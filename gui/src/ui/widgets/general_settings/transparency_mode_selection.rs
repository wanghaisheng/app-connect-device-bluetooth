use gtk::glib::{self, Object};

glib::wrapper! {
    pub struct TransparencyModeSelection(ObjectSubclass<imp::TransparencyModeSelection>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable, gtk::Buildable, gtk::ConstraintTarget;
}

impl TransparencyModeSelection {
    pub fn new() -> Self {
        Object::builder().build()
    }
}

mod imp {
    // Properties macro creates an enum for internal use. We don't care that it is caught by this lint.
    #![allow(clippy::enum_variant_names)]

    use std::cell::Cell;

    use gtk::{
        glib::{self, ParamSpec, Properties, Value},
        prelude::*,
        subclass::{
            prelude::*,
            widget::{CompositeTemplateClass, CompositeTemplateInitializingExt, WidgetImpl},
        },
        CompositeTemplate, TemplateChild,
    };
    use openscq30_lib::packets::structures::TransparencyMode;

    use crate::objects::BoxedTransparencyMode;

    #[derive(Default, CompositeTemplate, Properties)]
    #[template(
        resource = "/com/oppzippy/OpenSCQ30/ui/widgets/general_settings/transparency_mode_selection.ui"
    )]
    #[properties(wrapper_type=super::TransparencyModeSelection)]
    pub struct TransparencyModeSelection {
        #[template_child]
        pub transparency_mode_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub fully_transparent: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub vocal_mode: TemplateChild<gtk::ToggleButton>,

        #[property(set, get)]
        transparency_mode: Cell<BoxedTransparencyMode>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TransparencyModeSelection {
        const NAME: &'static str = "OpenSCQ30TransparencyModeSelection";
        type Type = super::TransparencyModeSelection;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TransparencyModeSelection {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            [
                (
                    TransparencyMode::FullyTransparent,
                    &self.fully_transparent.get(),
                ),
                (TransparencyMode::VocalMode, &self.vocal_mode.get()),
            ]
            .into_iter()
            .for_each(|(transparency_mode, button)| {
                obj.bind_property("transparency_mode", button, "active")
                    .transform_to(
                        move |_, selected_transparency_mode: BoxedTransparencyMode| {
                            Some(transparency_mode == selected_transparency_mode.0)
                        },
                    )
                    .transform_from(move |_, is_active| {
                        if is_active {
                            Some(BoxedTransparencyMode(transparency_mode))
                        } else {
                            None
                        }
                    })
                    .sync_create()
                    .bidirectional()
                    .build();
            });
        }

        fn properties() -> &'static [ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &Value, pspec: &ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &ParamSpec) -> Value {
            self.derived_property(id, pspec)
        }
    }

    impl WidgetImpl for TransparencyModeSelection {}
    impl BoxImpl for TransparencyModeSelection {}
}
