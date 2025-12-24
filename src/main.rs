use gtk4::prelude::*;
use libadwaita as adw;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, TextView, 
           MenuButton, gio, HeaderBar, IconTheme, ScrolledWindow, PolicyType, Switch, Label, Align, glib};
use std::cell::RefCell;
use std::rc::Rc;

mod static_data;
mod view_model;
mod helpers;
use view_model::AppViewModel;
use helpers::utils::create_labeled_spin;

const SPACING_MEDIUM: i32 = 12;
const SPACING_LARGE: i32 = 18;

fn main() {
    adw::init().expect("Failed to initialize Libadwaita");
    let app = Application::builder()
        .application_id("app.rayadams.loremgenerator")
        .flags(gio::ApplicationFlags::NON_UNIQUE)
        .build();

    app.connect_activate(build_ui);

    app.run();

}

fn build_ui(app: &Application) {
    let display = gtk4::gdk::Display::default().expect("Could not get default display.");
    let icon_theme = IconTheme::for_display(&display);

        // Check if running in Snap environment
    if let Ok(snap_path) = std::env::var("SNAP") {
        let assets_path = std::path::Path::new(&snap_path).join("assets");
        icon_theme.add_search_path(assets_path);
    } else {
        // Fallback for local development
        icon_theme.add_search_path("assets");

        // Check paths relative to the executable
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // 1. Assets next to executable (e.g. portable tarball)
                let local_assets = exe_dir.join("assets");
                if local_assets.exists() {
                    icon_theme.add_search_path(local_assets);
                }

                // 2. Standard Linux install: ../share/loremgenerator/assets
                // (assuming binary is in /usr/bin or /usr/local/bin)
                if let Some(prefix) = exe_dir.parent() {
                    let system_assets = prefix.join("share").join("loremgenerator").join("assets");
                    if system_assets.exists() {
                        icon_theme.add_search_path(system_assets);
                    }
                }
            }
        }
    }

    let view_model = Rc::new(RefCell::new(AppViewModel::new()));
    
    let window = ApplicationWindow::builder()
        .title("Lorem Ipsum Generator")
        .default_width(720)
        .default_height(600)
        .application(app)
        .build();

    let header_bar = HeaderBar::new();

    let menu = gio::Menu::new();
    menu.append(Some("About"), Some("app.about"));
    
    let menu_button = MenuButton::builder()
        .icon_name("open-menu-symbolic")
        .menu_model(&menu)
        .build();
    
    header_bar.pack_end(&menu_button);
    
    window.set_titlebar(Some(&header_bar));

        let about_action = gio::SimpleAction::new("about", None);
    let window_clone = window.clone();

    about_action.connect_activate(move |_, _| {
        let about = adw::AboutWindow::builder()
            .application_name("Lorem Ipsum Generator")
            .application_icon("app.rayadams.loremgenerator")
            .version(AppViewModel::get_app_version())
            .developers(vec!["Konstantin Adamov".to_string()])
            .website("https://github.com/xrayadams/loremgenerator-sr")
            .issue_url("https://github.com/xrayadams/loremgenerator-rs/issues")
            .license_type(gtk4::License::MitX11)
            .transient_for(&window_clone)
            .modal(true)
            .build();
        about.present();
    });
    app.add_action(&about_action);


    let vbox = GtkBox::builder()
        .orientation(gtk4::Orientation::Vertical)
        .spacing(SPACING_LARGE)
        .margin_start(SPACING_MEDIUM)
        .margin_end(SPACING_MEDIUM)
        .margin_top(SPACING_MEDIUM)
        .margin_bottom(SPACING_MEDIUM)
        .build();
    
    let input_box = GtkBox::new(gtk4::Orientation::Horizontal, SPACING_LARGE);

    let (box_words, spin_words) = create_labeled_spin("Max Words", 3, 25, view_model.borrow().max_words as i32);
    let (box_sentences, spin_sentences) = create_labeled_spin("Max Sentences", 1, 20, view_model.borrow().max_sentences as i32);
    let (box_paragraphs, spin_paragraphs) = create_labeled_spin("Paragraphs", 1, 40, view_model.borrow().paragraphs as i32);

    input_box.append(&box_words);
    input_box.append(&box_sentences);
    input_box.append(&box_paragraphs);

    let switch_box = GtkBox::new(gtk4::Orientation::Vertical, 6);
    let switch_label = Label::builder()
        .label("Start with Lorem")
        .halign(Align::Start)
        .build();
    let switch = Switch::builder()
        .active(view_model.borrow().start_with_lorem)
        .halign(Align::Center)
        .build();
    
    switch_box.append(&switch_label);
    switch_box.append(&switch);
    input_box.append(&switch_box);

    // Connect signals to update ViewModel
    let vm = view_model.clone();
    switch.connect_state_set(move |_, state| {
        vm.borrow_mut().set_start_with_lorem(state);
        glib::Propagation::Proceed
    });

    let vm = view_model.clone();
    spin_words.connect_value_changed(move |spin| {
        vm.borrow_mut().set_max_words(spin.value() as usize);
    });

    let vm = view_model.clone();
    spin_sentences.connect_value_changed(move |spin| {
        vm.borrow_mut().set_max_sentences(spin.value() as usize);
    });

    let vm = view_model.clone();
    spin_paragraphs.connect_value_changed(move |spin| {
        vm.borrow_mut().set_paragraphs(spin.value() as usize);
    });

    vbox.append(&input_box);

    // Create TextView inside ScrolledWindow for output
    let text_view = TextView::builder()
        .editable(false)
        .tooltip_text("Output")
        .wrap_mode(gtk4::WrapMode::Word)
        .left_margin(SPACING_MEDIUM)
        .right_margin(SPACING_MEDIUM)
        .top_margin(SPACING_MEDIUM)
        .bottom_margin(SPACING_MEDIUM)
        .css_classes(vec!["card"])
        .build();

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .child(&text_view)
        .vexpand(true)
        .hexpand(true)
        .build();

    vbox.append(&scrolled_window);

    // Create Buttons inside a Box, horizontally aligned
    let button_box = GtkBox::new(gtk4::Orientation::Horizontal, SPACING_LARGE);

    let generate_button = gtk4::Button::with_mnemonic("_Generate");
    generate_button.set_halign(gtk4::Align::Start);

    let copy_button = gtk4::Button::with_mnemonic("_Copy to Clipboard");
    copy_button.set_halign(gtk4::Align::Start);
    
    // Initially disable copy button if there's no text
    let buffer = text_view.buffer();
    copy_button.set_sensitive(!buffer.text(&buffer.start_iter(), &buffer.end_iter(), true).is_empty());

    // Update copy button sensitivity based on text buffer changes
    let copy_button_clone = copy_button.clone();
    buffer.connect_changed(move |buf| {
        let is_empty = buf.text(&buf.start_iter(), &buf.end_iter(), true).is_empty();
        copy_button_clone.set_sensitive(!is_empty);
    });

    button_box.append(&generate_button);
    button_box.append(&copy_button);
    vbox.append(&button_box);
    
    let toast_overlay = adw::ToastOverlay::new();
    toast_overlay.set_child(Some(&vbox));

    let text_view_clone = text_view.clone();
    let vm = view_model.clone();
    generate_button.connect_clicked(move |_| {
        let lorem_text = vm.borrow().generate();
        text_view_clone.buffer().set_text(&lorem_text);
    });

    let text_view_clone = text_view.clone();
    let toast_overlay_clone = toast_overlay.clone();
    copy_button.connect_clicked(move |_| {
        let buffer = text_view_clone.buffer();
        let (start, end) = buffer.bounds();
        let text = buffer.text(&start, &end, true);
        text_view_clone.clipboard().set_text(&text);
        
        let toast = adw::Toast::new("Copied to clipboard");
        toast_overlay_clone.add_toast(toast);
    });

    window.set_child(Some(&toast_overlay));
    window.show();
}