use gtk::{
    glib::{self, Object},
    prelude::ObjectExt,
};

mod imp;

glib::wrapper! {
    pub struct EqualizerCustomProfileObject(ObjectSubclass<imp::EqualizerCustomProfileObject>);
}

impl EqualizerCustomProfileObject {
    pub fn new(name: &String) -> Self {
        Object::new(&[("name", name)])
    }

    pub fn name(&self) -> String {
        self.property("name")
    }
}
