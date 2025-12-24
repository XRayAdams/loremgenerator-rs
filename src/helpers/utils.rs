use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Adjustment, SpinButton, Align, Orientation};

pub fn create_labeled_spin(label_text: &str, min: i32, max: i32, val: i32) -> (GtkBox, SpinButton) {
    let container = GtkBox::new(Orientation::Vertical, 6);
    let label = Label::builder()
        .label(label_text)
        .halign(Align::Start)
        .build();
    let adjustment = Adjustment::new(val as f64, min as f64, max as f64, 1.0, 10.0, 0.0);
    let spin = SpinButton::new(Some(&adjustment), 1.0, 0);
    container.append(&label);
    container.append(&spin);
    (container, spin)
}
