use gtk4::prelude::*;
use relm4::prelude::*;
use gtk4::{Align};

pub struct NumberEditor {
    label: String,
    min: usize,
    max: usize,
    value : usize,
}

#[derive(Debug)]
pub enum CounterOutput {
    ValueChanged(usize),
}

#[relm4::component(pub)]
impl SimpleComponent for NumberEditor {
    type Init = (String, usize, usize, usize); // (label, min, max, initial)
    type Input = ();
    type Output = CounterOutput;

    view! {
        root = gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 5,

            gtk::Label {
                #[watch]
                set_label: model.label.as_str(),
                set_halign: Align::Start,
            },

            gtk::SpinButton {
                set_adjustment: &gtk::Adjustment::new(
                    model.value as f64,
                    model.min as f64,
                    model.max as f64,
                    1.0,
                    10.0,
                    0.0,
                ),
                set_value: model.value as f64,
                set_halign: Align::Center,
                connect_value_changed[sender] => move |spin_button| {
                    let value = spin_button.value() as usize;
                    let _ = sender.output(CounterOutput::ValueChanged(value));
                },
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        
        let model = Self {
            label: _init.0,
            min: _init.1,
            max: _init.2,
            value: _init.3,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }
}
