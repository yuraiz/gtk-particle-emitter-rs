use adw::prelude::*;
use adw::subclass::prelude::*;
use glib::clone;
use gtk::glib;

mod imp {
    use std::time::Duration;

    use particles::*;

    use crate::value_row::{IntervalRow, SpinRow};

    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(string = r#"
        using Adw 1;

        template ParticleEditorWindow : Adw.ApplicationWindow {  
            width-request: 800;
            height-request: 400;

            Adw.Leaflet leaflet {
                Box {
                    orientation: vertical;
                    width-request: 500;

                    Adw.HeaderBar {
                        show-end-title-buttons: bind leaflet.folded;
                    }

                    Adw.PreferencesPage {
                        Adw.PreferencesGroup params {

                        }
                    }
                }

                Separator {}

                Box {
                    hexpand: true;
                    orientation: vertical;
                    
                    Adw.HeaderBar {
                        show-start-title-buttons: bind leaflet.folded;
                    }

                    Adw.Bin emitter_bin {
                        vexpand: true;
                        overflow: hidden;
                    }
                }
            }
        }


        // Just adjustment with maximum lower and upper bounds
        Adjustment adj {
            step-increment: 0.1;
            lower: -179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245490090389328944075868508455133942304583236903222948165808559332123348274797826204144723168738177180919299881250404026184124858368;
            upper: 179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245490090389328944075868508455133942304583236903222948165808559332123348274797826204144723168738177180919299881250404026184124858368;
        }
    "#)]
    pub struct Window {
        #[template_child]
        params: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        emitter_bin: TemplateChild<adw::Bin>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "ParticleEditorWindow";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();

            fn icon_texture(icon_name: &str) -> gtk::gdk::Texture {
                let theme = gtk::IconTheme::for_display(&gtk::gdk::Display::default().unwrap());

                let icon = theme.lookup_icon(
                    icon_name,
                    &[],
                    0,
                    1,
                    gtk::TextDirection::None,
                    gtk::IconLookupFlags::all(),
                );

                gtk::gdk::Texture::from_file(&icon.file().expect("icon not found")).unwrap()
            }

            let config = Config {
                scale: Interval {
                    start: 1.0,
                    end: 1.0,
                },
                ..Default::default()
            };

            let emitter = ParticleEmitter::new(
                config.clone(),
                ["weather-snow-symbolic", "starred-symbolic"]
                    .into_iter()
                    .map(icon_texture)
                    .collect(),
            );

            self.emitter_bin.set_child(Some(&emitter));

            macro_rules! interval {
                ($name:ident) => {
                    IntervalRow::new(
                    stringify!($name),
                    config.$name,
                    clone!(@weak emitter => move |val| {
                        emitter.update_config(move |config| {
                            config.$name = val.clone();
                        })
                    }),
                )};
            }

            self.params.add(&interval!(alpha));
            self.params.add(&interval!(scale));
            // color: Interval<Rgb>,
            self.params.add(&interval!(speed));
            self.params.add(&interval!(start_rotation));

            self.params.add(&IntervalRow::new(
                "lifetime",
                Interval {
                    start: config.lifetime.min.as_secs_f32(),
                    end: config.lifetime.max.as_secs_f32(),
                },
                clone!(@weak emitter => move |val| {
                    let bounds = Bounds {
                        min: Duration::from_secs_f32(val.start.min(val.end).max(0.0)),
                        max: Duration::from_secs_f32(val.end.max(val.start).max(0.0)),
                    };

                    emitter.update_config(move |config| {
                        config.lifetime = bounds.clone();
                    });
                }),
            ));

            // custom_ease: Option<fn(f32) -> f32>,

            self.params.add(&SpinRow::new(
                "spawn_frequency",
                clone!(@weak emitter => move|val|{
                  emitter.update_config(move|config|{
                    config.spawn_frequency = Duration::from_secs_f64(val);
                  })
                }),
                gtk::Adjustment::new(
                    config.spawn_frequency.as_secs_f64(),
                    0.001,
                    0.4,
                    0.01,
                    0.0,
                    0.0,
                ),
                3,
            ));

            self.params.add(&SpinRow::new(
                "max_particles",
                clone!(@weak emitter => move |val| {
                    emitter.update_config(move |config| {
                        config.max_particles = val as usize;
                    })
                }),
                gtk::Adjustment::new(config.max_particles as f64, 1.0, 200000.0, 1000.0, 0.0, 0.0),
                0,
            ));
        }
    }

    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
    impl AdwWindowImpl for Window {}
    impl AdwApplicationWindowImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::Window, adw::ApplicationWindow;
}

impl Window {
    pub fn new(application: &gtk::Application) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }
}
