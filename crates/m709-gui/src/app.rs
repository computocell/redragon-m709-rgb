use gtk4 as gtk;
use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{Application, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};

use crate::state::new_shared_state;
use crate::window::MainWindow;

const APP_CSS: &str = r#"
window {
    background-color: #1e1e24;
    color: #e0e0e0;
}

headerbar {
    background-color: #16161a;
    border-bottom: 1px solid #2b2b36;
}

.card {
    background-color: #262630;
    border-radius: 12px;
    padding: 18px;
    border: 1px solid #363645;
}

.heading {
    font-weight: bold;
    font-size: 1.05rem;
    color: #ffffff;
}

.title-2 {
    font-size: 1.35rem;
    font-weight: 800;
    color: #ffffff;
}

.dim-label {
    color: #9e9ea8;
    font-size: 0.9rem;
}

.badge {
    padding: 4px 10px;
    border-radius: 12px;
    font-size: 0.85rem;
    font-weight: 600;
}

.badge-success {
    background-color: rgba(38, 166, 91, 0.25);
    color: #2ecc71;
    border: 1px solid #27ae60;
}

.badge-danger {
    background-color: rgba(231, 76, 60, 0.25);
    color: #e74c3c;
    border: 1px solid #c0392b;
}

.preset-btn {
    border-radius: 6px;
    padding: 4px 10px;
    font-size: 0.85rem;
}

.footer {
    border-top: 1px solid #2b2b36;
    background-color: #16161a;
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
