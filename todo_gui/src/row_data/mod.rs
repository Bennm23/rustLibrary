mod imp;


use gtk::{
    glib, prelude::ObjectExt, prelude::WidgetExt, Button, CheckButton, Entry, ToggleButton
};

glib::wrapper! {
    pub struct RowData(ObjectSubclass<imp::RowData>);
}

impl RowData {
    pub fn new(
        complete_button: CheckButton,
        edit_button: ToggleButton,
        details_field: Entry,
        delete_button: Button,
    ) -> Self {
        glib::Object::builder()
            .property("complete_button", complete_button)
            .property("edit_button", edit_button)
            .property("details_field", details_field)
            .property("delete_button", delete_button)
            .build()
    }

    pub fn get_complete_button(&self) -> CheckButton {
        self.property("complete_button")
        // self.imp().complete_button.clone().into_inner()
    }
    pub fn get_edit_button(&self) -> ToggleButton {
        self.property("edit_button")
        // self.imp().edit_button.clone().into_inner()
    }
    pub fn get_details_field(&self) -> Entry {
        self.property("details_field")
        // self.imp().details_field.clone().into_inner()
    }
    pub fn get_delete_button(&self) -> Button {
        self.property("delete_button")
        // self.imp().delete_button.clone().into_inner()
    }
}
