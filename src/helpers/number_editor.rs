// Copyright (c) 2025, 2026 Konstantin Adamov. Licensed under MIT.

use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{glib, Align, Box, Label, SpinButton};

glib::wrapper! {
    pub struct NumberEditor(ObjectSubclass<NumberEditorPriv>)
        @extends Box, gtk4::Widget,
        @implements gtk4::Accessible, gtk4::Buildable, gtk4::ConstraintTarget, gtk4::Orientable;
}

impl NumberEditor {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_label(&self, label: &str) {
        self.imp().label.set_text(label);
    }

    pub fn label(&self) -> String {
        self.imp().label.text().to_string()
    }

    pub fn set_min(&self, min: f64) {
        let spin = &self.imp().spin_button;
        let adj = spin.adjustment();
        adj.set_lower(min);
    }

    pub fn set_max(&self, max: f64) {
        let spin = &self.imp().spin_button;
        let adj = spin.adjustment();
        adj.set_upper(max);
    }

    pub fn value(&self) -> f64 {
        self.imp().spin_button.value()
    }

    pub fn set_value(&self, value: f64) {
        self.imp().spin_button.set_value(value);
    }

    pub fn connect_value_changed<F>(&self, f: F) -> glib::SignalHandlerId
    where
        F: Fn(&Self, f64) + 'static,
    {
        self.connect_closure(
            "value-changed",
            false,
            glib::closure_local!(move |obj: &Self, value: f64| {
                f(obj, value);
            }),
        )
    }

    fn emit_value_changed(&self, value: f64) {
        self.emit_by_name::<()>("value-changed", &[&value]);
    }
}

impl Default for NumberEditor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
pub struct NumberEditorPriv {
    label: Label,
    spin_button: SpinButton,
}

#[glib::object_subclass]
impl ObjectSubclass for NumberEditorPriv {
    const NAME: &'static str = "NumberEditor";
    type Type = NumberEditor;
    type ParentType = Box;

    fn class_init(klass: &mut Self::Class) {
        klass.set_layout_manager_type::<gtk4::BoxLayout>();
    }
}

impl ObjectImpl for NumberEditorPriv {
    fn constructed(&self) {
        self.parent_constructed();

        let obj = self.obj();
        obj.set_orientation(gtk4::Orientation::Vertical);
        obj.set_spacing(5);

        self.label.set_halign(Align::Start);
        obj.append(&self.label);

        self.spin_button.set_halign(Align::Start);
        self.spin_button.set_adjustment(&gtk4::Adjustment::new(
            0.0, 0.0, 100.0, 1.0, 10.0, 0.0,
        ));
        obj.append(&self.spin_button);

        self.spin_button.connect_value_changed(glib::clone!(
            #[weak]
            obj,
            move |spin| {
                obj.emit_value_changed(spin.value());
            }
        ));
    }

    fn signals() -> &'static [glib::subclass::Signal] {
        use std::sync::OnceLock;
        static SIGNALS: OnceLock<Vec<glib::subclass::Signal>> = OnceLock::new();
        SIGNALS.get_or_init(|| {
            vec![glib::subclass::Signal::builder("value-changed")
                .param_types([f64::static_type()])
                .build()]
        })
    }
}

impl WidgetImpl for NumberEditorPriv {}
impl BoxImpl for NumberEditorPriv {}
