extern crate gtk4 as gtk;

mod list;

mod row_data;

use std::{sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}};

use gtk::{
    gdk,
    glib::{
        self, clone, 
    },
    prelude::*,
    style_context_add_provider_for_display, Align, ApplicationWindow, Button,
    CheckButton, CssProvider, Entry, GestureDrag, ListBox, ListBoxRow,
    ToggleButton,
};
use list::{TodoEntry, TodoList};
use row_data::RowData;

const CHECKED_STYLE: &str = "checked_item";
const DEFAULT_STYLE: &str = "default_item";

fn main() {
    // gtk::init().expect("Failed to Initialize Gtk");

    // let app = Application::builder()
    //     .application_id("local.benn.todolist")
    //     .build();

    // app.connect_activate(build_ui);
    // app.run();
}

fn build_ui(application: &gtk::Application) {
    let provider = CssProvider::new();
    provider.load_from_path("styles.css");
    style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could Not Load Display"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let window = ApplicationWindow::new(application);
    window.set_title(Some("ToDo List"));

    window.set_default_size(800, 800);

    let window_container = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    window.set_child(Some(&window_container));

    let list_container = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .spacing(5)
        .hexpand(true)
        .margin_top(5)
        .margin_start(30)
        .margin_end(30)
        .build();
    window_container.append(&list_container);

    let list_model = gtk::gio::ListStore::builder()
        .item_type(RowData::static_type())
        .build();

    let list_box = ListBox::new();

    list_box.set_halign(Align::Fill);
    list_container.append(&list_box);

    list_box.bind_model(Some(&list_model), move |item| {
        let todo_item = item
            .downcast_ref::<RowData>()
            .expect("Item Must be a RowData Object");

        println!("Adding Item!");
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
        hbox.style_context().add_class(DEFAULT_STYLE);

        let check = todo_item.get_complete_button();
        let edit = todo_item.get_edit_button();

        edit.set_sensitive(!check.is_active());
        if check.is_active() { hbox.style_context().add_class(CHECKED_STYLE); }
        check.connect_toggled(clone!(
            @weak hbox,
            @weak check,
            @weak edit
            =>
            move |_| {
                if check.is_active() {

                    edit.set_sensitive(false);
                    hbox.style_context().remove_class(DEFAULT_STYLE);
                    hbox.style_context().add_class(CHECKED_STYLE);
                }
                else {
                    edit.set_sensitive(true);
                    hbox.style_context().remove_class(CHECKED_STYLE);
                    hbox.style_context().add_class(DEFAULT_STYLE);
                }
            }
        ));
        //Must unparent all fields because they are still tied to the previous Box
        check.unparent();
        todo_item.get_details_field().unparent();
        todo_item.get_edit_button().unparent();
        todo_item.get_delete_button().unparent();

        hbox.append(&check);
        hbox.append(&todo_item.get_details_field());
        hbox.append(&todo_item.get_edit_button());
        hbox.append(&todo_item.get_delete_button());
        hbox.upcast::<gtk::Widget>()
    });

    let todo_list = TodoList::from_file();
    for todo_entry in todo_list.entries() {
        add_entry(
            &list_model,
            todo_entry.text(),
            todo_entry.completed(),
            false,
        );
    }

    let add_button = Button::builder()
        .label("Add Item")
        .hexpand(true)
        .build();
    let add_row = ListBoxRow::new();
    add_row.set_child(Some(&add_button));
    list_container.append(&add_row);

    add_button.connect_clicked(clone!(
        @weak list_model,
        =>
        move |_| {
            add_entry(&list_model, "Enter Todo", false, true);
        }
    ));

    let drag_controller = GestureDrag::new();

    let start_x = Arc::new(Mutex::new(0.0));
    let start_y = Arc::new(Mutex::new(0.0));
    drag_controller.connect_drag_begin(clone!(
        @weak list_box,
        @weak list_model,
        @strong start_x,
        @strong start_y,
        =>
        move |_, x, y| {
            println!("Drag Start at {}, {}", x, y);
            let curr = list_box.row_at_y(y as i32);
            let mut new_x = start_x.lock().unwrap();
            *new_x = x;
            let mut new_y = start_y.lock().unwrap();
            *new_y = y;

            match curr {
                Some(row) => {
                    println!("Drag Started at Row = {}", row.index());
                },
                None => {
                    println!("No Row Present");
                }
            };
            // gesture.set_state(gtk::EventSequenceState::Claimed);//Don't claim for edit
        }
    ));
    let time = Arc::new(Mutex::new(0.0));
    drag_controller.connect_drag_update(clone!(
        @weak list_box,
        @weak list_model,
        @strong start_x,
        @strong start_y,
        =>
        move |gesture, _, offset_y| {

            let mut update_time = time.lock().unwrap();

            let curr_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time Went Backwards")
                .as_secs_f64();


            //Style the highlighted row, if in top half insert above?
            if *update_time != 0.0 && (curr_time - *update_time < 0.1) {

                return;
            }
            *update_time = curr_time;

            let screen_y = *start_y.lock().unwrap() + offset_y;
            let curr = list_box.row_at_y(screen_y as i32);

            if let Some(row) = curr {
                list_box.drag_highlight_row(&row);
            }

            gesture.set_state(gtk::EventSequenceState::Claimed);
        }
    ));

    drag_controller.connect_drag_end(clone!(
        @weak list_box,
        @weak list_model,
        @strong start_x,
        @strong start_y,
        =>
        move |gesture, offset_x, offset_y| {
            if offset_y.abs() < 50.0 {
                println!("Drag to short");
                return;
            }
            let iy = *start_y.lock().unwrap();
            let screen_x = *start_x.lock().unwrap() - offset_x;
            let screen_y = iy + offset_y;

            let start_row = list_box.row_at_y(iy as i32);
            let end_row = list_box.row_at_y(screen_y as i32);
            println!("Drag End at {}, {}", screen_x, screen_y);

            if let Some(drop_row) = end_row {
                println!("Drag Ended at Row = {}", drop_row.index());
                if let Some(drag_row) = start_row {

                    let start_index = drag_row.index() as u32;
                    let start_item = list_model.item(start_index);

                    list_model.remove(start_index);
                    if let Some(moved_item) = start_item {
                        println!("List Item = {:?}", moved_item);
                        if drop_row.index() == ((list_model.n_items() - 1) as i32) {
                            list_model.append(&moved_item);
                        } else {
                            list_model.insert(drop_row.index() as u32, &moved_item);
                        }
                    }
                    else {
                        println!("Some(moved) not good");
                    }

                    list_box.drag_unhighlight_row();
                }
            }
            else {
                println!("No End Row Present");
            }

            gesture.set_state(gtk::EventSequenceState::Claimed);
        }
    ));
    list_box.add_controller(drag_controller);


    let w = window.clone();
    window.connect_close_request(move |_| {
        println!("Closing");
        let mut list = TodoList::new();

        let mut ctr = 1;
        for item in list_model.iter::<RowData>() {
            match item {
                Ok(data) => {
                    list.add_entry(TodoEntry::new(
                        data.complete_button().is_active(),
                        data.details_field().text().to_string(),
                        ctr,
                    ));
                    ctr += 1;
                }
                Err(_) => {
                    println!("Row Iter Had bad item")
                }
            }
        }

        list.save();

        w.destroy();
        glib::Propagation::Stop
    });
    window.present();
}

fn add_entry(list_model: &gtk::gio::ListStore, entry_text: &str, completed: bool, editing: bool) {
    let entry = Entry::new();
    entry.style_context().add_class("task_entry");
    entry.set_text(entry_text);

    entry.set_focusable(editing);
    entry.set_can_target(editing);
    entry.set_editable(editing);
    if editing {
        entry.style_context().add_class("edit_active");
    }
    entry.set_hexpand(true);

    let check = CheckButton::new();
    check.style_context().add_class("check_button");
    if completed {
        check.set_active(true);
    }

    let edit = ToggleButton::new();

    if editing {
        edit.set_label("\u{2611}")
    } else {
        edit.set_label("\u{270F}")
    };

    edit.style_context().add_class("edit_button");

    let del = Button::with_label("\u{1F5D1}");
    del.style_context().add_class("delete_button");

    edit.set_active(editing);

    edit.connect_toggled(clone!(
        @weak edit,
        @weak entry,
        =>
        move |_| {

            println!("EDIT BUTTON PRESSED");

            edit.grab_focus();
            if edit.is_active() {
                edit.set_label("\u{2611}");
                entry.set_editable(true);
                entry.set_focusable(true);
                entry.set_can_target(true);
                entry.grab_focus();
                entry.style_context().add_class("edit_active");
            }
            else {
                edit.set_label("\u{270F}");
                entry.set_editable(false);
                entry.set_focusable(false);
                entry.select_region(0, 0);
                entry.set_can_target(false);
                entry.style_context().remove_class("edit_active");
            }
        }
    ));

    entry.connect_activate(clone!(
        @weak entry,
        @weak edit,
        =>
        move |_| {
            println!("Enter Happened");
            if edit.is_active() {
                edit.set_active(false);
            }
        }
    ));

    let e = entry.clone();
    let d = del.clone();
    let data = RowData::new(check, edit, entry, del);

    d.connect_clicked(clone!(
        @weak list_model,
        @weak data,
        =>
        move |_| {
            let idx = list_model.find(&data);
            
            if let Some(index) = idx {
                list_model.remove(index);
            }
        }
    ));

    list_model.append(&data);
    if editing {
        e.grab_focus();
    }
}
