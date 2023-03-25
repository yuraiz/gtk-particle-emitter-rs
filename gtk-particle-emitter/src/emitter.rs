use particle_emitter::*;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gdk, graphene};

mod imp {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default, Debug)]
    pub struct ParticleEmitter {
        pub(super) ticker: RefCell<Ticker<gdk::Texture>>,
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
            let ticker = self.ticker.borrow();
            let particles = ticker.particles();

            particles.for_each(|particle| {
                let color_offset = {
                    let c = particle.color();
                    let alpha = particle.alpha();
                    graphene::Vec4::from_float([c.r, c.g, c.b, alpha])
                };

                snapshot.push_color_matrix(&graphene::Matrix::from_float([0.0; 16]), &color_offset);

                let (image, rect) = {
                    let particle = &particle;
                    let (x, y) = particle.coords();
                    let image = particle.image();
                    let scale = particle.scale();

                    let width = image.width() as f32 * scale;
                    let height = image.height() as f32 * scale;

                    let rect =
                        graphene::Rect::new(x - 0.5 * width, y - 0.5 * height, width, height);

                    (image, rect)
                };

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
    pub fn from_config(config: Config<gdk::Texture>) -> Self {
        let obj: Self = glib::Object::new();
        obj.imp().ticker.replace(Ticker::with_config(config));
        obj
    }
}
