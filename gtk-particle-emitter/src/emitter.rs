use particle_emitter::config::*;
use particle_emitter::ticker::Ticker;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, graphene};

mod imp {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default, Debug)]
    pub struct ParticleEmitter {
        pub(super) ticker: RefCell<Ticker>,
        pub(super) textures: RefCell<Vec<gdk::Texture>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ParticleEmitter {
        const NAME: &'static str = "ParticleEmitter";
        type Type = super::ParticleEmitter;
        type ParentType = gtk::Widget;
    }

    impl ObjectImpl for ParticleEmitter {
        fn constructed(&self) {
            self.obj().add_tick_callback(|widget, _clk| {
                widget.imp().ticker.borrow_mut().tick();
                widget.queue_draw();
                Continue(true)
            });
        }
    }

    impl WidgetImpl for ParticleEmitter {
        fn snapshot(&self, snapshot: &gtk::Snapshot) {
            let widget = self.obj();
            let width = widget.width() as f32;
            let height = widget.height() as f32;

            let ticker = self.ticker.borrow();
            let particles = ticker.particles();

            let textures = self.textures.borrow();
            if textures.is_empty() {
                return;
            }

            particles.for_each(|particle| {
                let (image, rect) = {
                    let particle = &particle;
                    let (x, y) = particle.coords();

                    let image_index = particle.image_index();
                    let image = &textures[image_index % textures.len()];

                    let scale = particle.scale();

                    let width = image.width() as f32 * scale;
                    let height = image.height() as f32 * scale;

                    let rect =
                        graphene::Rect::new(x - 0.5 * width, y - 0.5 * height, width, height);

                    (image, rect)
                };

                // no need to draw items those will be cropped
                if widget.overflow() == gtk::Overflow::Hidden {
                    let br = rect.bottom_right();
                    let tl = rect.top_left();

                    if br.x() <= 0.0 || br.y() <= 0.0 || tl.x() >= width || tl.y() >= height {
                        return;
                    }
                }

                let mut color_matrix = [0.0; 16];
                color_matrix[15] = particle.alpha();
                let color_matrix = { graphene::Matrix::from_float(color_matrix) };

                let c = particle.color();
                let color_offset = { graphene::Vec4::from_float([c.r, c.g, c.b, 0.0]) };

                snapshot.push_color_matrix(&color_matrix, &color_offset);

                snapshot.append_texture(image, &rect);
                snapshot.pop();
            });
        }
    }
}

glib::wrapper! {
    /// Widget that displays vector lottie animation
    pub struct ParticleEmitter(ObjectSubclass<imp::ParticleEmitter>)
        @extends gtk::Widget;
}

impl ParticleEmitter {
    pub fn new(config: Config, textures: Vec<gdk::Texture>) -> Self {
        let obj: Self = glib::Object::new();
        obj.set_overflow(gtk::Overflow::Hidden);
        obj.imp().ticker.replace(Ticker::with_config(config));
        obj.imp().textures.replace(textures);
        obj
    }
}
