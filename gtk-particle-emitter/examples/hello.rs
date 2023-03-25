use gtk::prelude::*;
use gtk::{gdk, glib};
use gtk_particle_emitter::ParticleEmitter;

const APP_ID: &str = "com.github.yuraiz.ParticleHello";

fn main() {
    let app = gtk::Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &gtk::Application) {
    let emitter = ParticleEmitter::new(Default::default(), images());

    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .width_request(500)
        .height_request(500)
        .child(&emitter)
        .build();
    window.present();
}

fn image(width: i32, height: i32) -> gdk::Texture {
    gdk::MemoryTexture::new(
        width,
        height,
        gdk::MemoryFormat::R8g8b8,
        &glib::Bytes::from_owned(vec![255; (width * height) as usize * 3]),
        width as usize * 3,
    )
    .upcast()
}

fn images() -> Vec<gdk::Texture> {
    vec![image(18, 18), image(20, 14), image(14, 20)]
}
