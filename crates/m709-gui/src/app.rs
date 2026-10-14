use gtk4 as gtk;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{Application, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};

use crate::state::new_shared_state;
use crate::window::MainWindow;

const APP_CSS: &str = r#"
window {
    background-color: #181920;
    color: #ffffff;
    font-size: 14px;
}

label {
    color: #ffffff;
}

.heading {
    font-weight: bold;
    font-size: 1.15rem;
    color: #ffffff;
}

.title-2 {
    font-size: 1.45rem;
    font-weight: 800;
    color: #ffffff;
}

.dim-label {
    color: #c0c4d6;
    font-size: 0.95rem;
}

headerbar {
    background-color: #101116;
    border-bottom: 2px solid #2d3040;
}

headerbar label {
    color: #ffffff;
    font-weight: 600;
}

.card {
    background-color: #222430;
    border-radius: 12px;
    padding: 18px;
    border: 1px solid #3c4054;
    margin-bottom: 8px;
}

button {
    color: #ffffff;
    background-color: #2f3244;
    border: 1px solid #454962;
    border-radius: 8px;
    padding: 6px 14px;
    font-weight: 600;
}

button:hover {
    background-color: #3d4158;
    color: #ffffff;
}

button.suggested-action {
    background-color: #2563eb;
    color: #ffffff;
    border: 1px solid #60a5fa;
    font-weight: bold;
}

button.suggested-action:hover {
    background-color: #1d4ed8;
}

button.destructive-action {
    background-color: #dc2626;
    color: #ffffff;
    border: 1px solid #f87171;
}

stackswitcher button {
    color: #d1d5db;
    background-color: #222430;
    border: 1px solid #3c4054;
    font-weight: 600;
    padding: 8px 18px;
    border-radius: 8px;
    margin: 0 4px;
}

stackswitcher button:checked {
    background-color: #2563eb;
    color: #ffffff;
    border: 1px solid #93c5fd;
}

dropdown {
    background-color: #2b2e3f;
    color: #ffffff;
    border: 1px solid #454962;
    border-radius: 8px;
    padding: 6px 12px;
}

dropdown > button {
    color: #ffffff;
    font-weight: 500;
}

entry {
    background-color: #2b2e3f;
    color: #ffffff;
    border: 1px solid #454962;
    border-radius: 8px;
    padding: 6px 10px;
    font-weight: 500;
}

checkbutton {
    color: #ffffff;
}

checkbutton label {
    color: #ffffff;
    font-weight: 600;
}

.badge {
    padding: 5px 12px;
    border-radius: 14px;
    font-size: 0.9rem;
    font-weight: bold;
}

.badge-success {
    background-color: rgba(34, 197, 94, 0.25);
    color: #4ade80;
    border: 1px solid #22c55e;
}

.badge-danger {
    background-color: rgba(239, 68, 68, 0.25);
    color: #f87171;
    border: 1px solid #ef4444;
}

.preset-btn {
    border-radius: 6px;
    padding: 6px 12px;
    font-size: 0.9rem;
    font-weight: 600;
}

popover {
    background-color: #222430;
    color: #ffffff;
    border: 1px solid #454962;
    border-radius: 8px;
}

popover listview row {
    color: #ffffff;
    padding: 8px 12px;
    font-size: 0.95rem;
}

popover listview row:selected {
    background-color: #2563eb;
    color: #ffffff;
}

scale trough {
    background-color: #353849;
    border-radius: 6px;
    min-height: 8px;
}

scale highlight {
    background-color: #3b82f6;
    border-radius: 6px;
}

scale slider {
    background-color: #ffffff;
    border: 3px solid #2563eb;
    min-width: 20px;
    min-height: 20px;
    border-radius: 50%;
}

.boxed-list {
    background-color: #1a1b24;
    border-radius: 8px;
    border: 1px solid #3c4054;
}

.boxed-list row {
    border-bottom: 1px solid #282a38;
}

.footer {
    border-top: 2px solid #2d3040;
    background-color: #101116;
    padding: 10px 16px;
}

.footer label {
    color: #93c5fd;
    font-weight: 600;
}
"#;

pub fn build_application() -> Application {
    let app = Application::builder()
        .application_id("com.redragon.mouse.m709")
        .build();

    app.connect_startup(|_| {
        load_css();
    });

    app.connect_activate(|app| {
        let state = new_shared_state();
        let main_window = MainWindow::new(app, state);
        main_window.present();
    });

    app
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(APP_CSS);

    if let Some(display) = Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
