use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;
use particles::Interval;

mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(string = r#"
        using Adw 1;

        template ComponentsInterval : Adw.ActionRow {
            [suffix] SpinButton spin1 {
                digits: 2;
                valign: center;
                width-chars: 5;
                value-changed => update_value(ComponentsInterval);
                adjustment: adj1;
            }
            [suffix] SpinButton spin2 {
                digits: 2;
                valign: center;
                width-chars: 5;
                value-changed => update_value(ComponentsInterval);
                adjustment: adj2;
            }
        }

        Adjustment adj1 {
            step-increment: 0.1;
            lower: -179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245490090389328944075868508455133942304583236903222948165808559332123348274797826204144723168738177180919299881250404026184124858368;
            upper: 179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245490090389328944075868508455133942304583236903222948165808559332123348274797826204144723168738177180919299881250404026184124858368;
        }

        Adjustment adj2 {
            step-increment: 0.1;
            lower: -179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245490090389328944075868508455133942304583236903222948165808559332123348274797826204144723168738177180919299881250404026184124858368;
            upper: 179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558632766878171540458953514382464234321326889464182768467546703537516986049910576551282076245490090389328944075868508455133942304583236903222948165808559332123348274797826204144723168738177180919299881250404026184124858368;
        }
    "#)]
    pub struct IntervalRow {
        pub(super) update: RefCell<Option<Box<dyn Fn(Interval<f32>)>>>,
        #[template_child]
        pub(super) spin1: TemplateChild<gtk::SpinButton>,
        #[template_child]
        pub(super) spin2: TemplateChild<gtk::SpinButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for IntervalRow {
        const NAME: &'static str = "ComponentsInterval";
        type Type = super::IntervalRow;
        type ParentType = adw::ActionRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for IntervalRow {}
    impl WidgetImpl for IntervalRow {}
    impl ListBoxRowImpl for IntervalRow {}
    impl PreferencesRowImpl for IntervalRow {}
    impl ActionRowImpl for IntervalRow {}

    #[gtk::template_callbacks]
    impl IntervalRow {
        #[template_callback]
        fn update_value(&self) {
            let interval = Interval {
                start: self.spin1.value() as f32,
                end: self.spin2.value() as f32,
            };

            if let Some(update) = &*self.update.borrow() {
                update(interval);
            }
        }
    }
}

glib::wrapper! {
    pub struct IntervalRow(ObjectSubclass<imp::IntervalRow>)
        @extends gtk::Widget, adw::PreferencesRow;
}

impl IntervalRow {
    pub fn new<F>(title: &str, init: Interval<f32>, update: F) -> Self
    where
        F: Fn(Interval<f32>) + 'static,
    {
        let obj: Self = glib::Object::new();

        obj.set_title(title);

        let imp = obj.imp();
        imp.update.replace(Some(Box::new(update)));
        imp.spin1.set_value(init.start as f64);
        imp.spin2.set_value(init.end as f64);

        obj
    }
}
