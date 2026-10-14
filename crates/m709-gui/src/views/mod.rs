pub mod buttons;
pub mod dpi;
pub mod lighting;
pub mod profiles;

use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Align, Label, ListItem, SignalListItemFactory, StringObject};

pub fn create_dropdown_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();
    factory.connect_setup(|_, list_item| {
        let item = list_item.downcast_ref::<ListItem>().unwrap();
        let label = Label::builder()
            .halign(Align::Start)
            .valign(Align::Center)
            .css_classes(["dropdown-label"])
            .build();
        item.set_child(Some(&label));
    });
    factory.connect_bind(|_, list_item| {
        let item = list_item.downcast_ref::<ListItem>().unwrap();
        if let Some(obj) = item.item().and_downcast::<StringObject>() {
            if let Some(label) = item.child().and_downcast::<Label>() {
                label.set_text(&obj.string());
            }
        }
    });
    factory
}

