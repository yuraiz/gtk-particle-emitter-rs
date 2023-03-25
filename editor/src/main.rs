mod value_row;
mod window;

use adw::prelude::*;
use window::Window;

const APP_ID: &'static str = "com.github.yuraiz.ParticleEditor";

fn main() {
    adw::init().unwrap();

    let app = gtk::Application::builder().application_id(APP_ID).build();

    app.connect_activate(|app| {
        let window = Window::new(app);
        window.present();
    });

    app.run();
}
