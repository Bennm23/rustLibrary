use gtk::{
    glib::{self},
    prelude::*,
    subclass::prelude::*,
    Button, CheckButton, Entry, ToggleButton,
};

use std::cell::RefCell;

#[derive(Default, glib::Properties)]
#[properties(wrapper_type = super::RowData)]
pub struct RowData {
    // #[property(set, get)]
    // pub complete_button : CheckButton,
    // #[property(set, get)]
    // pub edit_button : ToggleButton,
    // #[property(set, get)]
    // pub details_field : Entry,
    // #[property(set, get)]
    // pub delete_button : Button,
    #[property(set, get)]
    pub complete_button: RefCell<CheckButton>,
    #[property(set, get)]
    pub edit_button: RefCell<ToggleButton>,
    #[property(set, get)]
    pub details_field: RefCell<Entry>,
    #[property(set, get)]
    pub delete_button: RefCell<Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for RowData {
    const NAME: &'static str = "RowData";
    type Type = super::RowData;
}

#[glib::derived_properties]
impl ObjectImpl for RowData {}
