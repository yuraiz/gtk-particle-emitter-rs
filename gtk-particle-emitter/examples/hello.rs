use gtk::prelude::*;
use gtk_particle_emitter::{helpers::texture_for_icon, Config, Interval, ParticleEmitter};

const APP_ID: &str = "com.github.yuraiz.ParticleHello";

fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let textures = [
        "folder-symbolic",
        "folder-documents-symbolic",
        "image-x-generic-symbolic",
    ]
    .into_iter()
    .map(|s| texture_for_icon(s, 32))
    .collect();

    let emitter = ParticleEmitter::new(
        Config {
            scale: Interval {
                start: 0.5,
                end: 1.0,
            },
            ..Config::default()
        },
        textures,
    );

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .width_request(500)
        .height_request(500)
        .child(&emitter)
        .build();
    window.present();
}
