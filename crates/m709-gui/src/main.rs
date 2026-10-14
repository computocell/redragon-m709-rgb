use gtk4::glib;
use gtk4::prelude::*;

mod app;
mod state;
mod views;
mod window;

fn main() -> glib::ExitCode {
    let application = app::build_application();
    application.run()
}
