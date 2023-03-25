use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(string = r#"
        using Adw 1;

        template ComponentsDuration : Adw.ActionRow {
            [suffix] SpinButton spin {
                hexpand: true;
                valign: center;
                value-changed => update_value(ComponentsDuration);
            }
        }
    "#)]
    pub struct DurationRow {
        pub(super) update: RefCell<Option<Box<dyn Fn(f64)>>>,
        #[template_child]
        pub(super) spin: TemplateChild<gtk::SpinButton>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DurationRow {
        const NAME: &'static str = "ComponentsDuration";
        type Type = super::SpinRow;
        type ParentType = adw::ActionRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DurationRow {}
    impl WidgetImpl for DurationRow {}
    impl ListBoxRowImpl for DurationRow {}
    impl PreferencesRowImpl for DurationRow {}
    impl ActionRowImpl for DurationRow {}

    #[gtk::template_callbacks]
    impl DurationRow {
        #[template_callback]
        fn update_value(&self) {
            if let Some(update) = &*self.update.borrow() {
                update(self.spin.value());
            }
        }
    }
}

glib::wrapper! {
    pub struct SpinRow(ObjectSubclass<imp::DurationRow>)
        @extends gtk::Widget, adw::PreferencesRow;
}

impl SpinRow {
    pub fn new<F>(title: &str, update: F, adjustment: gtk::Adjustment, digits: u32) -> Self
    where
        F: Fn(f64) + 'static,
    {
        let obj: Self = glib::Object::new();

        obj.set_title(title);

        let imp = obj.imp();
        imp.update.replace(Some(Box::new(update)));
        imp.spin.set_adjustment(&adjustment);
        imp.spin.set_digits(digits);

        obj
    }
}
